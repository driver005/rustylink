use crate::{ Context, TaskConfig, TaskMapper, TaskModel};
use chrono::Utc;
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel };
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "switch")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub evaluator_type: String,
	pub expression: String,
	pub decision_cases: serde_json::Value,
	pub default_case: Option<Vec<Uuid>>,
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
		&TaskType::Switch
	}

	fn map_task(&self, _context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::Switch;
		task.status = TaskStatus::InProgress;
		task.task_def_name = TaskType::TASK_TYPE_SWITCH.to_string();
		task.start_time = Utc::now();
		task.reason_for_incompletion = None;

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.switch_id = Some(self.id);

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
			evaluator_type: owned
				.evaluator_type
				.ok_or_else(|| Error::IllegalArgument("evaluator_type is missing".to_string()))?,
			expression: owned
				.expression
				.ok_or_else(|| Error::IllegalArgument("expression is missing".to_string()))?,
			decision_cases: owned
				.decision_cases
				.ok_or_else(|| Error::IllegalArgument("decision_cases is missing".to_string()))?,
			default_case: owned.default_case,
		})
	}
}

pub type Switch = Model;
