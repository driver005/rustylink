use crate::{Context, TaskConfig, TaskDefinition, TaskModel};
use chrono::Utc;
use metadata::{Result, TaskDef, TaskType};
use std::sync::Arc;

// Trait representing the TaskMapper interface in Java.
// It handles task mapping logic without using getters or setters.
#[async_trait::async_trait]
pub trait TaskMapper {
	fn get_task_def(&self, context: &Context, name: &str) -> Result<TaskDefinition> {
		// context
		// 	.metadata
		// 	.get_task_def(name)
		// 	.ok_or_else(|| Error::not_found("task definition not found"))
		Ok(TaskDefinition::new("test_def".to_string()))
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

	async fn save(self, context: &mut Context) -> Result<()>;

	/// Get the name of the system task.
	///
	/// # Returns
	///
	/// The task type name.
	fn get_task_type(&self) -> &TaskType;

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
	fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()>;

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
	async fn execute(&self, context: &mut Context) -> Result<TaskModel>;
}
