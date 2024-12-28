use std::collections::HashMap;

use metadata::{
	ExternalStorageLocation, RerunWorkflowRequest, SearchResult, SkipTaskRequest,
	StartWorkflowRequest, Workflow, WorkflowDef, WorkflowSummary,
};

/// Trait defining operations for managing workflows.
trait WorkflowService {
	/// Starts a new workflow with the provided `StartWorkflowRequest`.
	///
	/// # Arguments
	///
	/// * `request` - The request containing details for starting the workflow.
	///
	/// # Returns
	///
	/// Returns the ID of the started workflow instance.
	fn start_workflow_request(&self, request: StartWorkflowRequest) -> String;

	/// Starts a new workflow with the specified parameters.
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow to start.
	/// * `version` - Optional version of the workflow.
	/// * `correlation_id` - Optional correlation ID for the workflow.
	/// * `priority` - Optional priority of the workflow (0-99).
	/// * `input` - Input parameters for the workflow.
	///
	/// # Returns
	///
	/// Returns the ID of the started workflow instance.
	fn start_workflow(
		&self,
		name: String,
		version: Option<u32>,
		correlation_id: Option<String>,
		priority: Option<u32>,
		input: HashMap<String, serde_json::Value>,
	) -> String;

	/// Starts a new workflow with additional detailed parameters.
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow to start.
	/// * `version` - Optional version of the workflow.
	/// * `correlation_id` - Optional correlation ID for the workflow.
	/// * `priority` - Optional priority of the workflow (0-99).
	/// * `input` - Input parameters for the workflow.
	/// * `external_input_payload_storage_path` - Optional path for external input payload storage.
	/// * `task_to_domain` - Optional mapping of tasks to domains.
	/// * `workflow_def` - Definition of the workflow.
	///
	/// # Returns
	///
	/// Returns the ID of the started workflow instance.
	fn start_workflow_detailed(
		&self,
		name: String,
		version: Option<u32>,
		correlation_id: Option<String>,
		priority: Option<u32>,
		input: HashMap<String, serde_json::Value>,
		external_input_payload_storage_path: Option<String>,
		task_to_domain: HashMap<String, String>,
		workflow_def: WorkflowDef,
	) -> String;

	/// Retrieves a list of workflows based on the given parameters.
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow.
	/// * `correlation_id` - Optional correlation ID for filtering workflows.
	/// * `include_closed` - Whether to include closed (non-running) workflows.
	/// * `include_tasks` - Whether to include tasks associated with workflows.
	///
	/// # Returns
	///
	/// Returns a list of workflows.
	fn get_workflows(
		&self,
		name: String,
		correlation_id: Option<String>,
		include_closed: bool,
		include_tasks: bool,
	) -> Vec<Workflow>;

	/// Retrieves a map of workflows based on correlation IDs.
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow.
	/// * `include_closed` - Whether to include closed (non-running) workflows.
	/// * `include_tasks` - Whether to include tasks associated with workflows.
	/// * `correlation_ids` - List of correlation IDs to filter workflows.
	///
	/// # Returns
	///
	/// Returns a map with correlation IDs as keys and lists of workflows as values.
	fn get_workflows_multiple(
		&self,
		name: String,
		include_closed: bool,
		include_tasks: bool,
		correlation_ids: Vec<String>,
	) -> HashMap<String, Vec<Workflow>>;

	/// Retrieves the status of a workflow by its ID.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to retrieve.
	/// * `include_tasks` - Whether to include tasks associated with the workflow.
	///
	/// # Returns
	///
	/// Returns the workflow status.
	fn get_execution_status(&self, workflow_id: String, include_tasks: bool) -> Workflow;

	/// Deletes a workflow from the system.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to delete.
	/// * `archive_workflow` - Whether to archive the workflow instead of deleting it.
	fn delete_workflow(&self, workflow_id: String, archive_workflow: bool);

	/// Retrieves a list of running workflows.
	///
	/// # Arguments
	///
	/// * `workflow_name` - Name of the workflow to filter by.
	/// * `version` - Optional version of the workflow.
	/// * `start_time` - Optional start time for filtering workflows.
	/// * `end_time` - Optional end time for filtering workflows.
	///
	/// # Returns
	///
	/// Returns a list of IDs of running workflows.
	fn get_running_workflows(
		&self,
		workflow_name: String,
		version: Option<u32>,
		start_time: Option<u64>,
		end_time: Option<u64>,
	) -> Vec<String>;

	/// Starts a decision task for a workflow.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to start the decision task for.
	fn decide_workflow(&self, workflow_id: String);

	/// Pauses a workflow.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to pause.
	fn pause_workflow(&self, workflow_id: String);

	/// Resumes a paused workflow.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to resume.
	fn resume_workflow(&self, workflow_id: String);

	/// Skips a task from the specified workflow.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow containing the task.
	/// * `task_reference_name` - Reference name of the task to skip.
	/// * `skip_task_request` - Request containing details for skipping the task.
	fn skip_task_from_workflow(
		&self,
		workflow_id: String,
		task_reference_name: String,
		skip_task_request: SkipTaskRequest,
	);

	/// Reruns a workflow from a specific task.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to rerun.
	/// * `request` - Request containing details for rerunning the workflow.
	///
	/// # Returns
	///
	/// Returns the ID of the rerun workflow.
	fn rerun_workflow(&self, workflow_id: String, request: RerunWorkflowRequest) -> String;

	/// Restarts a completed workflow.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to restart.
	/// * `use_latest_definitions` - Whether to use the latest workflow and task definitions.
	fn restart_workflow(&self, workflow_id: String, use_latest_definitions: bool);

	/// Retries the last failed task in a workflow.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow containing the task to retry.
	/// * `resume_subworkflow_tasks` - Whether to resume tasks in sub-workflows.
	fn retry_workflow(&self, workflow_id: String, resume_subworkflow_tasks: bool);

	/// Resets callback times of all non-terminal SIMPLE tasks to 0.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to reset.
	fn reset_workflow(&self, workflow_id: String);

	/// Terminates a workflow execution.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to terminate.
	/// * `reason` - Optional reason for terminating the workflow.
	fn terminate_workflow(&self, workflow_id: String, reason: Option<String>);

	/// Terminates a workflow and removes it from the system.
	///
	/// # Arguments
	///
	/// * `workflow_id` - ID of the workflow to terminate and remove.
	/// * `reason` - Optional reason for terminating the workflow.
	/// * `archive_workflow` - Whether to archive the workflow instead of permanently removing it.
	fn terminate_remove_workflow(
		&self,
		workflow_id: String,
		reason: Option<String>,
		archive_workflow: bool,
	);

	/// Searches for workflows based on the given parameters.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - Optional sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflow summaries.
	fn search_workflows(
		&self,
		start: u32,
		size: u32,
		sort: Option<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<WorkflowSummary>;

	/// Searches for workflows with detailed information.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - Optional sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflows.
	fn search_workflows_v2(
		&self,
		start: u32,
		size: u32,
		sort: Option<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<Workflow>;

	/// Searches for workflows with sorting options.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - List of sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflow summaries.
	fn search_workflows_sorted(
		&self,
		start: u32,
		size: u32,
		sort: Vec<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<WorkflowSummary>;

	/// Searches for workflows with detailed information and sorting options.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - List of sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflows.
	fn search_workflows_v2_sorted(
		&self,
		start: u32,
		size: u32,
		sort: Vec<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<Workflow>;

	/// Searches for workflows by tasks with given parameters.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - Optional sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflow summaries.
	fn search_workflows_by_tasks(
		&self,
		start: u32,
		size: u32,
		sort: Option<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<WorkflowSummary>;

	/// Searches for workflows by tasks with detailed information.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - Optional sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflows.
	fn search_workflows_by_tasks_v2(
		&self,
		start: u32,
		size: u32,
		sort: Option<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<Workflow>;

	/// Searches for workflows by tasks with sorting options.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - List of sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflow summaries.
	fn search_workflows_by_tasks_sorted(
		&self,
		start: u32,
		size: u32,
		sort: Vec<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<WorkflowSummary>;

	/// Searches for workflows by tasks with detailed information and sorting options.
	///
	/// # Arguments
	///
	/// * `start` - Starting index for pagination.
	/// * `size` - Number of results to return per page.
	/// * `sort` - List of sorting criteria.
	/// * `free_text` - Optional free text search.
	/// * `query` - Optional query for more complex searches.
	///
	/// # Returns
	///
	/// Returns a search result containing a list of workflows.
	fn search_workflows_by_tasks_v2_sorted(
		&self,
		start: u32,
		size: u32,
		sort: Vec<String>,
		free_text: Option<String>,
		query: Option<String>,
	) -> SearchResult<Workflow>;

	/// Retrieves the location of external storage.
	///
	/// # Arguments
	///
	/// * `path` - Path to the external storage.
	/// * `operation` - Type of operation to perform (e.g., read, write).
	/// * `payload_type` - Type of payload being accessed.
	///
	/// # Returns
	///
	/// Returns details about the external storage location.
	fn get_external_storage_location(
		&self,
		path: String,
		operation: String,
		payload_type: String,
	) -> ExternalStorageLocation;
}
