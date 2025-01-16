use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use metadata::{Error, IdempotencyStrategy, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "sub_workflow")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	// The name of the workflow to be executed. This workflow should have a pre-existing definition
	pub name: String,
	// The version of the workflow to be executed.
	pub version: u32,
	// A map of sub-workflow tasks to specific domains. The keys are the task reference names and the values are the domain names. If not given, the taskToDomain of the executing parent workflow will take over.
	pub task_to_domain: Option<serde_json::Value>,
	// The priority of the subworkflow. Supports values from 0-99 and can be passed as a variable.
	// If set, this priority overrides the parent workflows priority. If not, it inherits the parent workflows priority.
	pub priority: i8,
	// A unique, user-generated key to prevent duplicate workflow executions. Idempotency data is retained for the life of the workflow execution.
	pub idempotency_key: Option<String>,
	// The idempotency strategy for handling duplicate requests. Supported values:
	//  -  RETURN_EXISTING—Return the workflowId of the workflow instance with the same idempotency key.
	//  -  FAIL—Start a new workflow instance only if there are no workflow executions with the same idempotency key.
	//  -  FAIL_ON_RUNNING—Start a new workflow instance only if there are no RUNNING or PAUSED workflows with the same idempotency key. Completed workflows can run again.
	pub idempotency_strategy: Option<IdempotencyStrategy>,
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
		TaskType::SubWorkflow
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.get_queue().push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::SubWorkflow;
		task.status = TaskStatus::Scheduled;
		task.callback_after_seconds = self.task_configuration.start_delay;

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

		let sub_workflow_param = owned
			.sub_workflow_param
			.ok_or_else(|| Error::IllegalArgument("sub_workflow_param is missing".to_string()))?;

		if sub_workflow_param.name.is_empty() {
			return Err(Error::IllegalArgument("sub_workflow_param.name is missing".to_string()));
		}

		if sub_workflow_param.idempotency_key.is_some()
			&& sub_workflow_param.idempotency_strategy.is_none()
		{
			return Err(Error::IllegalArgument(
				"sub_workflow_param.idempotency_strategy is missing".to_string(),
			));
		}

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			name: sub_workflow_param.name,
			version: sub_workflow_param.version,
			priority: sub_workflow_param.priority,
			task_to_domain: sub_workflow_param.task_to_domain,
			idempotency_key: sub_workflow_param.idempotency_key,
			idempotency_strategy: sub_workflow_param.idempotency_strategy,
			task_model_id: None,
		})
	}
}

pub type SubWorkflow = Model;
