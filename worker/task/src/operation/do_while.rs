use crate::{Context, SqlTask, TaskConfig, TaskMapper, TaskModel};
use chrono::Utc;
use metadata::{Error, OperationType, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, DatabaseBackend, IntoActiveModel, QueryTrait};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "do_while")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub evaluator_type: String,
	pub loop_condition: String,
	pub loop_over: Vec<Uuid>,
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
		&TaskType::DoWhile
	}

	fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let task_def = self.get_task_def(context, &self.task_configuration.name)?;

		task.task_type = TaskType::DoWhile;
		task.status = TaskStatus::InProgress;
		task.start_time = Utc::now();
		task.retry_count = self.task_configuration.retry_count;
		task.rate_limit_per_frequency = task_def.rate_limit_per_frequency;
		task.rate_limit_frequency_in_seconds = task_def.rate_limit_frequency_in_seconds;

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		// for task in self.loop_over.iter() {
		// 	let task_model = self.to_task(context, Arc::new(task.clone()), workflow)?;
		// 	self.add_to_queue(context, &task_model)?;
		// }

		// match self.loop_over.get(0) {
		// 	Some(task_config) => {
		// 		// let task_model = task_config.to_task()?;

		// 		// task_model.execute(context)?;
		// 		todo!()
		// 	}
		// 	None => return Err(Error::conflict("loop_over task with index 0 not found")),
		// }

		todo!()

		// Ok(())
	}

	async fn save(self, context: &mut Context) -> Result<()> {
		SqlTask::new(
			format!("{}_{}", self.get_task_type().to_string(), self.id.to_string()),
			Entity::insert(self.into_active_model()).build(DatabaseBackend::Postgres).to_string(),
			OperationType::Insert,
			Vec::new(),
			None,
		)
		.execute(context)
		.await?;

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
			evaluator_type: owned
				.evaluator_type
				.ok_or_else(|| Error::IllegalArgument("evaluator_type is missing".to_string()))?,
			loop_condition: owned
				.loop_condition
				.ok_or_else(|| Error::IllegalArgument("loop_condition is missing".to_string()))?,
			loop_over: owned
				.loop_over
				.ok_or_else(|| Error::IllegalArgument("loop_over is missing".to_string()))?,
		})
	}
}

pub type DoWhile = Model;
