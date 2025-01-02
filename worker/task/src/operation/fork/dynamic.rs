#[cfg(feature = "worker")]
use crate::TaskExecutor;
use crate::TaskModel;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskStorage};
use chrono::Utc;
use metadata::{Error, ForkType, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "dynamic_fork")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub fork_type: ForkType,
	// Reference task model id
	pub task_model_id: Option<Uuid>,

	// Fields for DifferentTask
	pub dynamic_fork_tasks_param: Option<String>,
	pub dynamic_fork_tasks_input_param_name: Option<String>,

	// Fields for SameTask
	pub fork_task_name: Option<String>,
	pub fork_task_inputs: Option<serde_json::Value>, // JSON input for SameTask and SameTaskSubWorkflow

	// Fields for SameTaskSubWorkflow
	pub fork_task_workflow: Option<String>,
	pub fork_task_workflow_version: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "crate::data::model::Entity",
		from = "Column::TaskModelId",
		to = "crate::data::model::Column::TaskId",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Taskmodel,
}

impl Related<crate::model::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Taskmodel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(feature = "worker")]
impl TaskExecutor for Model {
	async fn execute(&mut self, context: &mut Context) -> Result<TaskModel> {
		todo!()
	}
}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskMapper for Model {
	fn get_task_type() -> TaskType {
		TaskType::ForkJoinDynamic
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.get_queue().push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let current_time = Utc::now();
		task.task_type = TaskType::ForkJoinDynamic;
		task.task_def_name = TaskType::TASK_TYPE_FORK.to_string();
		task.status = TaskStatus::Completed;
		task.start_time = current_time;
		task.end_time = Some(current_time);

        task.to_owned().insert(context).await?;

		Ok(())
	}

	async fn execute(&mut self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model).await?;

		self.task_model_id = Some(task_model.task_id);

		self.to_owned().save(context).await?;

		Ok(task_model)
	}
}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskStorage for Model {
	type Entity = Entity;
	type Model = Self;
	type PrimaryKey = Uuid;
	type ActiveModel = ActiveModel;

	async fn insert(self, context: &Context) -> Result<Self::Model> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn update(self, context: &Context) -> Result<Self::Model> {
		ActiveModel::update(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn save(self, context: &Context) -> Result<Self::ActiveModel> {
		ActiveModel::save(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn delete(self, context: &Context) -> Result<()> {
		ActiveModel::delete(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}

	fn find() -> Select<Self::Entity> {
		Entity::find()
	}

	async fn find_by_id(context: &Context, task_id: Self::PrimaryKey) -> Result<Self::Model> {
		let task = Entity::find_by_id(task_id)
			.one(&context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		if let Some(m) = task {
			Ok(m)
		} else {
			Err(Error::NotFound(format!(
				"Could not find {} task with id: {}",
				Self::get_task_type(),
				task_id
			)))
		}
	}
}

#[cfg(feature = "handler")]
impl TryFrom<Arc<TaskConfig>> for Model {
	type Error = Error;

	fn try_from(value: Arc<TaskConfig>) -> Result<Self, Self::Error> {
		let task_configuration = Arc::clone(&value);
		let owned = match Arc::try_unwrap(value) {
			Ok(val) => val,
			Err(_) => return Err(Error::conflict("could not unwrap workflow task")),
		};

		let fork_type;

		if owned.dynamic_fork_tasks_param.is_some()
			|| owned.dynamic_fork_tasks_input_param_name.is_some()
		{
			fork_type = ForkType::DifferentTask;
		} else if owned.get_input_parameter_required("fork_task_name").is_ok() {
			fork_type = ForkType::SameTask;
		} else if owned.get_input_parameter_required("fork_task_workflow").is_ok() {
			fork_type = ForkType::SameTaskSubWorkflow;
		} else {
			return Err(Error::IllegalArgument(
				"dynamic_fork_tasks_param, dynamic_fork_tasks_input_param_name or fork_task_name, fork_task_inputs or fork_task_workflow, fork_task_workflow_version, fork_task_inputs are missing".to_string(),
			));
		};

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			fork_type,
			dynamic_fork_tasks_param: owned.dynamic_fork_tasks_param.clone(),
			dynamic_fork_tasks_input_param_name: owned.dynamic_fork_tasks_input_param_name.clone(),
			fork_task_name: owned
				.get_input_parameter_optinal("fork_task_name")
				.map(|v| v.to_string()),
			fork_task_inputs: owned
				.get_input_parameter_optinal("fork_task_inputs")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			fork_task_workflow: owned
				.get_input_parameter_optinal("fork_task_workflow")
				.map(|v| v.to_string()),
			fork_task_workflow_version: owned
				.get_input_parameter_optinal("fork_task_workflow_version")
				.map(|v| v.to_string()),
			task_model_id: None,
		})
	}
}

pub type DynamicFork = Model;
