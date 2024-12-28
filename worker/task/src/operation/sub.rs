use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use metadata::{Error, IdempotencyStrategy, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel };
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sub_workflow")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub name: String,
	pub version: u32,
	pub task_to_domain: Option<serde_json::Value>,
	pub idempotency_key: Option<String>,
	pub idempotency_strategy: Option<IdempotencyStrategy>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "crate::model::Entity")]
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
	fn get_task_type(&self) -> &TaskType {
		&TaskType::SubWorkflow
	}

	fn map_task(&self, _context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::SubWorkflow;
		task.status = TaskStatus::Scheduled;
		task.callback_after_seconds = self.task_configuration.start_delay;

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.sub_workflow_id = Some(self.id);

		self.to_owned().save(context).await?;

		context.queue.push(&self.get_task_type().to_string(), self.id.to_string())?;

		Ok(task_model)
	}

	async fn save(self, context: &mut Context) -> Result<()> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
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
			task_to_domain: sub_workflow_param.task_to_domain,
			idempotency_key: sub_workflow_param.idempotency_key,
			idempotency_strategy: sub_workflow_param.idempotency_strategy,
		})
	}
}

pub type SubWorkflow = Model;
