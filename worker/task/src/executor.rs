use crate::{Context, TaskModel};
use metadata::Result;

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
	async fn execute(&self, context: &mut Context) -> Result<TaskModel>;
}
