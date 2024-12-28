use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use metadata::{Error, IdempotencyStrategy, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "start_workflow")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub name: String,
	pub version: Option<i64>,
	pub correlation_id: Option<String>,
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
		&TaskType::StartWorkflow
	}

	fn map_task(&self, _context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::StartWorkflow;
		task.status = TaskStatus::Scheduled;
		task.callback_after_seconds = self.task_configuration.start_delay;

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.start_workflow_id = Some(self.id);

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

		let name = owned.get_input_parameter_required("name")?.to_string();

		if name.is_empty() {
			return Err(Error::IllegalArgument("name is missing".to_string()));
		}

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			name,
			version: owned.get_input_parameter_optinal("version").and_then(|v| v.as_i64()),
			correlation_id: owned
				.get_input_parameter_optinal("correlation_id")
				.map(|v| v.to_string()),
			idempotency_key: owned
				.get_input_parameter_optinal("idempotency_key")
				.map(|v| v.to_string()),
			idempotency_strategy: match owned
				.get_input_parameter_optinal("idempotency_strategy")
				.and_then(|v| v.as_str())
			{
				Some(val) => Some(serde_json::from_str::<IdempotencyStrategy>(val)?),
				None => None,
			},
		})
	}
}

pub type StartWorkflow = Model;
