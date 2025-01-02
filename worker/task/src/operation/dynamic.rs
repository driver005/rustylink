#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "dynamic")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	// The input parameter key whose value is used to schedule the task.
	// For example, "taskToExecute", which will then be specified as an input parameter in the Dynamic task.
	pub dynamic_task_name_param: String,
	// The name of the task that will be executed
	pub task_refrence_name: String,
	// The name of the sub-workflow that will be executed if the taskToExecute is set to SUB_WORKFLOW
	pub sub_workflow_name: Option<String>,
	// The version of the sub-workflow that will be executed if the taskToExecute is set to SUB_WORKFLOW
	pub sub_workflow_version: Option<i64>,
	// Reference task model id
	pub task_model_id: Option<Uuid>,
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

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskMapper for Model {
	fn get_task_type() -> TaskType {
		TaskType::Dynamic
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.get_queue().push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let task_def = self
			.get_task_def(context, self.task_configuration.task_reference_name.to_owned())
			.await?;

		task.task_type = TaskType::Dynamic;
		task.status = TaskStatus::Scheduled;
		task.start_delay_in_seconds = self.task_configuration.start_delay;
		task.retry_count = self.task_configuration.retry_count;
		task.callback_after_seconds = self.task_configuration.start_delay;
		task.response_timeout_seconds = Some(task_def.response_timeout_seconds);
		task.retried_task_id = None;

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

		let dynamic_task_name_param = owned.dynamic_task_name_param.clone().ok_or_else(|| {
			Error::IllegalArgument("dynamic_task_name_param is missing".to_string())
		})?;

		let sub_workflow_name =
			owned.get_input_parameter_optinal("sub_workflow_name").map(|v| v.to_string());

		let sub_workflow_version =
			owned.get_input_parameter_optinal("sub_workflow_version").and_then(|v| v.as_i64());

		if dynamic_task_name_param != "SUB_WORKFLOW"
			&& (sub_workflow_name.is_some() || sub_workflow_version.is_some())
		{
			return Err(Error::IllegalArgument(
                "sub_workflow_name and sub_workflow_version can only be used with task_refrence_name = SUB_WORKFLOW".to_string(),
            ));
		}

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			dynamic_task_name_param: dynamic_task_name_param.clone(),
			task_refrence_name: owned
				.get_input_parameter_required(&dynamic_task_name_param)?
				.to_string(),
			sub_workflow_name,
			sub_workflow_version,
			task_model_id: None,
		})
	}
}

pub type Dynamic = Model;
