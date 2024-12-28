use std::collections::HashMap;

use metadata::{
	ExternalStorageLocation, PollData, SearchResult, Task, TaskExecLog, TaskResult, TaskStatus,
	TaskSummary,
};

/// Trait defining task-related operations for workflow management
pub trait TaskService {
	/// Poll for a task of a certain type
	///
	/// # Arguments
	///
	/// * `task_type` - Task name
	/// * `worker_id` - Id of the worker
	/// * `domain` - Domain of the workflow
	///
	/// # Returns
	///
	/// The polled Task, if available
	fn poll(&self, task_type: &str, worker_id: &str, domain: &str) -> Option<Task>;

	/// Batch poll for tasks of a certain type
	///
	/// # Arguments
	///
	/// * `task_type` - Task name
	/// * `worker_id` - Id of the worker
	/// * `domain` - Domain of the workflow
	/// * `count` - Number of tasks to poll
	/// * `timeout` - Timeout for polling in milliseconds
	///
	/// # Returns
	///
	/// A vector of polled Tasks
	fn batch_poll(
		&self,
		task_type: &str,
		worker_id: &str,
		domain: &str,
		count: u32,
		timeout: u32,
	) -> Vec<Task>;

	/// Get in-progress tasks (paginated)
	///
	/// # Arguments
	///
	/// * `task_type` - Task name
	/// * `start_key` - Start index of pagination
	/// * `count` - Number of entries to retrieve
	///
	/// # Returns
	///
	/// A vector of in-progress Tasks
	fn get_tasks(&self, task_type: &str, start_key: &str, count: u32) -> Vec<Task>;

	/// Get a pending task for a given workflow
	///
	/// # Arguments
	///
	/// * `workflow_id` - Id of the workflow
	/// * `task_reference_name` - Task reference name
	///
	/// # Returns
	///
	/// The pending Task, if available
	fn get_pending_task_for_workflow(
		&self,
		workflow_id: &str,
		task_reference_name: &str,
	) -> Option<Task>;

	/// Update a task
	///
	/// # Arguments
	///
	/// * `task_result` - The TaskResult to update with
	///
	/// # Returns
	///
	/// The ID of the updated task
	fn update_task(&self, task_result: &TaskResult) -> Result<String, String>;

	/// Acknowledge that a task is received
	///
	/// # Arguments
	///
	/// * `task_id` - Id of the task
	/// * `worker_id` - Id of the worker
	///
	/// # Returns
	///
	/// A string indicating if the task was received
	fn ack_task_received(&self, task_id: &str, worker_id: &str) -> String;

	/// Acknowledge that a task is received (without worker id)
	///
	/// # Arguments
	///
	/// * `task_id` - Id of the task
	///
	/// # Returns
	///
	/// A boolean indicating if the task was received
	fn ack_task_received_no_worker(&self, task_id: &str) -> bool;

	/// Log task execution details
	///
	/// # Arguments
	///
	/// * `task_id` - Id of the task
	/// * `log` - Details to log
	fn log(&self, task_id: &str, log: &str);

	/// Get task execution logs
	///
	/// # Arguments
	///
	/// * `task_id` - Id of the task
	///
	/// # Returns
	///
	/// A vector of TaskExecLog
	fn get_task_logs(&self, task_id: &str) -> Vec<TaskExecLog>;

	/// Get a task by its ID
	///
	/// # Arguments
	///
	/// * `task_id` - Id of the task
	///
	/// # Returns
	///
	/// The Task, if found
	fn get_task(&self, task_id: &str) -> Option<Task>;

	/// Remove a task from a task type queue
	///
	/// # Arguments
	///
	/// * `task_type` - Task name
	/// * `task_id` - ID of the task
	fn remove_task_from_queue(&self, task_type: &str, task_id: &str);

	/// Remove a task from a queue by task ID
	///
	/// # Arguments
	///
	/// * `task_id` - ID of the task
	fn remove_task_from_queue_by_id(&self, task_id: &str);

	/// Get task type queue sizes
	///
	/// # Arguments
	///
	/// * `task_types` - List of task types
	///
	/// # Returns
	///
	/// A HashMap of task type to queue size
	fn get_task_queue_sizes(&self, task_types: &[String]) -> HashMap<String, u32>;

	/// Get the queue size for a specific task type
	///
	/// # Arguments
	///
	/// * `task_type` - Task type
	/// * `domain` - Domain (optional)
	/// * `isolation_group_id` - Isolation group ID (optional)
	/// * `execution_namespace` - Execution namespace (optional)
	///
	/// # Returns
	///
	/// The queue size
	fn get_task_queue_size(
		&self,
		task_type: &str,
		domain: Option<&str>,
		isolation_group_id: Option<&str>,
		execution_namespace: Option<&str>,
	) -> u32;

	/// Get detailed information about all queues
	///
	/// # Returns
	///
	/// A nested HashMap structure with queue details
	fn all_verbose(&self) -> HashMap<String, HashMap<String, HashMap<String, u64>>>;

	/// Get summary information about all queues
	///
	/// # Returns
	///
	/// A HashMap of queue names to their sizes
	fn get_all_queue_details(&self) -> HashMap<String, u64>;

	/// Get the last poll data for a given task type
	///
	/// # Arguments
	///
	/// * `task_type` - Task name
	///
	/// # Returns
	///
	/// A vector of PollData
	fn get_poll_data(&self, task_type: &str) -> Vec<PollData>;

	/// Get the last poll data for all task types
	///
	/// # Returns
	///
	/// A vector of PollData for all task types
	fn get_all_poll_data(&self) -> Vec<PollData>;

	/// Requeue pending tasks
	///
	/// # Arguments
	///
	/// * `task_type` - Task name
	///
	/// # Returns
	///
	/// A string indicating the number of tasks requeued
	fn requeue_pending_task(&self, task_type: &str) -> String;

	/// Search for tasks
	///
	/// # Arguments
	///
	/// * `start` - Start index of pagination
	/// * `size` - Number of entries
	/// * `sort` - Sorting type (ASC|DESC)
	/// * `free_text` - Text to search
	/// * `query` - Query to search
	///
	/// # Returns
	///
	/// A SearchResult containing TaskSummary items
	fn search(
		&self,
		start: u32,
		size: u32,
		sort: &str,
		free_text: &str,
		query: &str,
	) -> SearchResult<TaskSummary>;

	/// Search for tasks (version 2)
	///
	/// # Arguments
	///
	/// * `start` - Start index of pagination
	/// * `size` - Number of entries
	/// * `sort` - Sorting type (ASC|DESC)
	/// * `free_text` - Text to search
	/// * `query` - Query to search
	///
	/// # Returns
	///
	/// A SearchResult containing Task items
	fn search_v2(
		&self,
		start: u32,
		size: u32,
		sort: &str,
		free_text: &str,
		query: &str,
	) -> SearchResult<Task>;

	/// Get the external storage location for task output payload
	///
	/// # Arguments
	///
	/// * `path` - The path for which the external storage location is to be populated
	/// * `operation` - The operation to be performed (read or write)
	/// * `payload_type` - The type of payload (input or output)
	///
	/// # Returns
	///
	/// An ExternalStorageLocation containing the URI and path
	fn get_external_storage_location(
		&self,
		path: &str,
		operation: &str,
		payload_type: &str,
	) -> ExternalStorageLocation;

	/// Update a task with specific parameters
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow
	/// * `task_ref_name` - Task reference name
	/// * `status` - New status of the task
	/// * `worker_id` - ID of the worker
	/// * `output` - Output data of the task
	///
	/// # Returns
	///
	/// A string indicating the result of the update operation
	fn update_task_with_params(
		&self,
		workflow_id: &str,
		task_ref_name: &str,
		status: TaskStatus,
		worker_id: &str,
		output: &HashMap<String, serde_json::Value>,
	) -> String;
}
