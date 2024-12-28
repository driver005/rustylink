#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use chrono::Utc;
use metadata::{Error, Result, TaskStatus, TaskTerminationStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel };
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "terminate_task")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub termination_status: TaskTerminationStatus,
	pub termination_reason: Option<String>,
	pub workflow_output: Option<serde_json::Value>,
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
		&TaskType::TerminateTask
	}

	fn map_task(&self, _context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::TerminateTask;
		task.status = TaskStatus::InProgress;
		task.start_time = Utc::now();

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.terminate_task_id = Some(self.id);

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
		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			termination_status: match owned
				.get_input_parameter_required("termination_status")?
				.as_str()
			{
				Some(val) => serde_json::from_str::<TaskTerminationStatus>(val)?,
				None => {
					return Err(Error::IllegalArgument("termination_status is missing".to_string()))
				}
			},
			workflow_output: owned
				.get_input_parameter_optinal("workflow_output")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			termination_reason: owned
				.get_input_parameter_optinal("termination_reason")
				.map(|v| v.to_string()),
		})
	}
}

pub type TerminateTask = Model;
