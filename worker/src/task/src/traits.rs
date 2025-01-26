use crate::{Context, TaskConfig, TaskDefinition, TaskModel};
use chrono::Utc;
use common::{Result, TaskType};
use sea_orm::{EntityTrait, Select};
use std::sync::Arc;
use uuid::Uuid;

// Trait representing the TaskMapper interface in Java.
// It handles task mapping logic without using getters or setters.
#[cfg(feature = "handler")]
#[async_trait::async_trait]
pub trait TaskMapper {
	async fn get_task_def(&self, context: &Context, name: String) -> Result<TaskDefinition> {
		TaskDefinition::find_by_id(context, name).await
	}

	fn new_task(&self, task_config: &Arc<TaskConfig>) -> Result<TaskModel> {
		let task = TaskModel::new(
			task_config.task_type,
			task_config.task_reference_name.as_str(),
			task_config.task_reference_name.as_str(),
			Utc::now(),
		);

		Ok(task)
	}

	fn get_primary_key(&self) -> Uuid;

	fn add_to_queue(&self, context: &Context) -> Result<()>;

	/// Get the name of the system task.
	///
	/// # Returns
	///
	/// The task type name.
	fn get_task_type() -> TaskType
	where
		Self: Sized;

	/// Maps tasks based on the provided context.
	/// This method is asynchronous and can return a Error.
	///
	/// # Arguments
	///
	/// * `context` - The context containing the workflow and task information.
	///
	/// # Returns
	///
	/// A result containing either a vector of `TaskModel` instances
	/// or a `Error` if an error occurs.
	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()>;

	/// "Execute" the task.
	///
	/// # Parameters
	///
	/// - `workflow`: Workflow for which the task is being started.
	/// - `task`: Instance of the task.
	/// - `workflow_executor`: The workflow executor.
	///
	/// # Returns
	///
	/// True if the execution has changed the task status, false otherwise.
	async fn execute(&mut self, context: &mut Context) -> Result<TaskModel>;
}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
pub trait TaskStorage {
	type Entity: EntityTrait;
	type Model;
	type PrimaryKey;
	type ActiveModel;

	async fn insert(self, context: &Context) -> Result<Self::Model>;

	async fn update(self, context: &Context) -> Result<Self::Model>;

	async fn save(self, context: &Context) -> Result<Self::ActiveModel>;

	async fn delete(self, context: &Context) -> Result<()>;

	fn find() -> Select<Self::Entity>;

	async fn find_by_id(context: &Context, task_id: Self::PrimaryKey) -> Result<Self::Model>;
}

#[cfg(feature = "worker")]
#[async_trait::async_trait]
pub trait TaskExecutor {
	/// "Execute" the task.
	///
	/// # Parameters
	///
	/// - `workflow`: Workflow for which the task is being started.
	/// - `task`: Instance of the task.
	/// - `workflow_executor`: The workflow executor.
	///
	/// # Returns
	///
	/// True if the execution has changed the task status, false otherwise.
	async fn execute(&mut self, context: &mut Context) -> Result<()>;
}
