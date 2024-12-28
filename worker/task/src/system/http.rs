#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "http")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub uri: String,
	pub method: String,
	pub accept: Option<String>,
	pub content_type: Option<String>,
	pub termination_condition: Option<String>,
	pub polling_interval: Option<i64>,
	pub polling_strategy: Option<String>,
	pub headers: Option<serde_json::Value>,
	pub body: Option<serde_json::Value>,
	pub encode: Option<bool>,
	pub async_complete: Option<bool>,
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
		&TaskType::Http
	}

	fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let task_def = self.get_task_def(context, &self.task_configuration.name)?;

		task.task_type = TaskType::Http;
		task.status = TaskStatus::Scheduled;
		task.retry_count = self.task_configuration.retry_count;
		task.callback_after_seconds = self.task_configuration.start_delay;
		task.rate_limit_per_frequency = task_def.rate_limit_per_frequency;
		task.rate_limit_frequency_in_seconds = task_def.rate_limit_frequency_in_seconds;
		task.isolation_group_id = task_def.isolation_group_id.clone();
		task.execution_name_space = task_def.execution_name_space.clone();

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.http_id = Some(self.id);

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

#[cfg(feature = "worker")]
impl TaskExecutor for Model {
	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		todo!()
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
			uri: owned.get_input_parameter_required("uri")?.to_string(),
			method: owned.get_input_parameter_required("method")?.to_string(),
			accept: owned.get_input_parameter_optinal("accept").map(|v| v.to_string()),
			content_type: owned.get_input_parameter_optinal("content_type").map(|v| v.to_string()),
			termination_condition: owned
				.get_input_parameter_optinal("termination_condition")
				.map(|v| v.to_string()),
			polling_interval: owned
				.get_input_parameter_optinal("polling_interval")
				.and_then(|v| v.as_i64()),
			polling_strategy: owned
				.get_input_parameter_optinal("polling_strategy")
				.map(|v| v.to_string()),
			headers: owned
				.get_input_parameter_optinal("headers")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			body: owned
				.get_input_parameter_optinal("body")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			encode: owned.get_input_parameter_optinal("encode").and_then(|v| v.as_bool()),
			async_complete: owned
				.get_input_parameter_optinal("async_complete")
				.and_then(|v| v.as_bool()),
		})
	}
}

pub type Http = Model;
