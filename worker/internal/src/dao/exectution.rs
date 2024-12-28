use chrono::Duration;
use metadata::EventExecution;
use model::{TaskModel, WorkflowModel};

pub trait ExecutionDao {
	// Get pending tasks for a given workflow
	fn get_pending_tasks_by_workflow(&self, task_name: &str, workflow_id: &str) -> Vec<TaskModel>;

	// Get tasks by task type with pagination support
	fn get_tasks(&self, task_type: &str, start_key: &str, count: usize) -> Vec<TaskModel>;

	// Create new tasks
	fn create_tasks(&self, tasks: Vec<TaskModel>) -> Vec<TaskModel>;

	// Update a task
	fn update_task(&self, task: &TaskModel);

	// Remove a task by task ID
	fn remove_task(&self, task_id: &str) -> bool;

	// Retrieve a task by its ID
	fn get_task(&self, task_id: &str) -> Option<TaskModel>;

	// Retrieve tasks by a list of task IDs
	fn get_tasks_by_ids(&self, task_ids: Vec<String>) -> Vec<TaskModel>;

	// Get pending tasks for a specific task type
	fn get_pending_tasks_for_task_type(&self, task_type: &str) -> Vec<TaskModel>;

	// Get tasks associated with a workflow
	fn get_tasks_for_workflow(&self, workflow_id: &str) -> Vec<TaskModel>;

	// Create a new workflow
	fn create_workflow(&self, workflow: WorkflowModel) -> String;

	// Update a workflow
	fn update_workflow(&self, workflow: WorkflowModel) -> String;

	// Remove a workflow
	fn remove_workflow(&self, workflow_id: &str) -> bool;

	// Remove workflow with an expiration (TTL)
	fn remove_workflow_with_expiry(&self, workflow_id: &str, ttl_seconds: u64) -> bool;

	// Retrieve a workflow by ID
	fn get_workflow(&self, workflow_id: &str) -> Option<WorkflowModel>;

	// Retrieve a workflow by ID, optionally including tasks
	fn get_workflow_with_tasks(
		&self,
		workflow_id: &str,
		include_tasks: bool,
	) -> Option<WorkflowModel>;

	// Get running workflow IDs by name and version
	fn get_running_workflow_ids(&self, workflow_name: &str, version: i32) -> Vec<String>;

	// Get workflows that are pending by type and version
	fn get_pending_workflows_by_type(
		&self,
		workflow_name: &str,
		version: i32,
	) -> Vec<WorkflowModel>;

	// Get count of pending workflows for a specific name
	fn get_pending_workflow_count(&self, workflow_name: &str) -> u64;

	// Get count of tasks in progress for a task definition
	fn get_in_progress_task_count(&self, task_def_name: &str) -> u64;

	// Get workflows by type and time range
	fn get_workflows_by_type(
		&self,
		workflow_name: &str,
		start_time: Duration,
		end_time: Duration,
	) -> Vec<WorkflowModel>;

	// Get workflows by correlation ID
	fn get_workflows_by_correlation_id(
		&self,
		workflow_name: &str,
		correlation_id: &str,
		include_tasks: bool,
	) -> Vec<WorkflowModel>;

	// Check if the DAO can search across workflows
	fn can_search_across_workflows(&self) -> bool;

	// Events
	fn add_event_execution(&self, event_execution: EventExecution) -> bool;

	fn update_event_execution(&self, event_execution: EventExecution);

	fn remove_event_execution(&self, event_execution: EventExecution);
}
