use std::collections::HashMap;

use metadata::Task;

pub trait AdminService {
	/// Queue up all the running workflows for sweep.
	///
	/// # Parameters:
	/// * `workflow_id` - The ID of the workflow to be swept.
	///
	/// # Returns:
	/// The ID of the workflow instance that can be used for tracking.
	fn requeue_sweep(&self, workflow_id: String) -> Option<String>;

	/// Get all the configuration parameters.
	///
	/// # Returns:
	/// A map containing all configuration parameters.
	fn get_all_config(&self) -> HashMap<String, String>;

	/// Get the list of pending tasks for a given task type.
	///
	/// # Parameters:
	/// * `task_type` - Name of the task.
	/// * `start` - Start index of pagination.
	/// * `count` - Number of entries to retrieve.
	///
	/// # Returns:
	/// A vector containing a list of pending `Task`.
	fn get_list_of_pending_task(
		&self,
		task_type: String,
		start: Option<u32>,
		count: Option<u32>,
	) -> Vec<Task>;

	/// Verify that the Workflow is consistent, and run repairs as needed.
	///
	/// # Parameters:
	/// * `workflow_id` - ID of the workflow to be verified and repaired.
	///
	/// # Returns:
	/// `true` if the repair was successful, otherwise `false`.
	fn verify_and_repair_workflow_consistency(&self, workflow_id: String) -> bool;

	/// Get registered queues.
	///
	/// # Parameters:
	/// * `verbose` - Whether to return verbose logs.
	///
	/// # Returns:
	/// A map of event queues.
	fn get_event_queues(&self, verbose: bool) -> HashMap<String, String>;
}
