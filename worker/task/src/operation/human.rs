use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use chrono::{DateTime, Utc};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{
	entity::prelude::*, FromJsonQueryResult, IntoActiveModel ,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct UserFormTemplate {
	pub name: String,
	pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Assignee {
	pub user_type: String,
	pub user: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Assignment {
	pub assignee: Assignee,
	pub sla_minutes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct TaskTrigger {
	pub trigger_type: String,
	pub start_workflow_request: TaskConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "human")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub task_type: TaskType,
	pub status: TaskStatus,
	pub start_time: DateTime<Utc>,
	pub assignment_completion_strategy: String,
	pub display_name: String,
	pub user_form_template: Option<UserFormTemplate>,
	pub assignments: Option<Vec<Assignment>>,
	pub task_triggers: Option<Vec<TaskTrigger>>,
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
		&TaskType::Human
	}

	fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::Human;
		task.status = TaskStatus::InProgress;
		task.start_time = Utc::now();

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.human_id = Some(self.id);

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

pub type Human = Model;
