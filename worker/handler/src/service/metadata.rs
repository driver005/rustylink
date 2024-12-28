use metadata::{BulkResponse, EventHandler, TaskDef, WorkflowDef};
use std::collections::{HashMap, HashSet};

/// Trait defining metadata operations for workflow management
pub trait MetadataService {
	/// Register task definitions
	///
	/// # Arguments
	///
	/// * `task_definitions` - A vector of TaskDef to register
	fn register_task_def(&self, task_definitions: Vec<TaskDef>) -> Result<(), String>;

	/// Update an existing task definition
	///
	/// # Arguments
	///
	/// * `task_definition` - The TaskDef to be updated
	fn update_task_def(&self, task_definition: TaskDef) -> Result<(), String>;

	/// Remove a task definition
	///
	/// # Arguments
	///
	/// * `task_type` - The type of task to remove
	fn unregister_task_def(&self, task_type: &str) -> Result<(), String>;

	/// Retrieve all registered task definitions
	///
	/// # Returns
	///
	/// A vector of all registered TaskDef
	fn get_task_defs(&self) -> Vec<TaskDef>;

	/// Retrieve a specific task definition
	///
	/// # Arguments
	///
	/// * `task_type` - The type of task to retrieve
	///
	/// # Returns
	///
	/// An Option containing the TaskDef if found, None otherwise
	fn get_task_def(&self, task_type: &str) -> Option<TaskDef>;

	/// Update a workflow definition
	///
	/// # Arguments
	///
	/// * `def` - The WorkflowDef to be updated
	fn update_workflow_def(&self, def: WorkflowDef) -> Result<(), String>;

	/// Update multiple workflow definitions
	///
	/// # Arguments
	///
	/// * `workflow_def_list` - A vector of WorkflowDef to be updated
	///
	/// # Returns
	///
	/// A BulkResponse containing the results of the operation
	fn update_workflow_defs(
		&self,
		workflow_def_list: Vec<WorkflowDef>,
	) -> Result<BulkResponse, String>;

	/// Retrieve a specific workflow definition
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow to retrieve
	/// * `version` - Optional version of the workflow. If None, retrieves the latest version
	///
	/// # Returns
	///
	/// An Option containing the WorkflowDef if found, None otherwise
	fn get_workflow_def(&self, name: &str, version: Option<u32>) -> Option<WorkflowDef>;

	/// Retrieve the latest version of a workflow definition
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow to retrieve
	///
	/// # Returns
	///
	/// An Option containing the latest WorkflowDef if found, None otherwise
	fn get_latest_workflow(&self, name: &str) -> Option<WorkflowDef>;

	/// Retrieve all workflow definitions (all versions)
	///
	/// # Returns
	///
	/// A vector of all WorkflowDef
	fn get_workflow_defs(&self) -> Vec<WorkflowDef>;

	/// Retrieve workflow names and versions (without definition bodies)
	///
	/// # Returns
	///
	/// A HashMap where keys are workflow names and values are sets of WorkflowDefSummary
	//TODO: add funcktion WorkflowDefSummary not found
	// fn get_workflow_names_and_versions(&self) -> HashMap<String, HashSet<WorkflowDefSummary>>;

	/// Register a new workflow definition
	///
	/// # Arguments
	///
	/// * `workflow_def` - The WorkflowDef to be registered
	fn register_workflow_def(&self, workflow_def: WorkflowDef) -> Result<(), String>;

	/// Validate a workflow definition
	///
	/// # Arguments
	///
	/// * `workflow_def` - The WorkflowDef to be validated
	///
	/// # Returns
	///
	/// Ok(()) if validation passes, Err with a message otherwise
	fn validate_workflow_def(&self, workflow_def: &WorkflowDef) -> Result<(), String> {
		// Default implementation that does nothing
		Ok(())
	}

	/// Remove a specific version of a workflow definition
	///
	/// # Arguments
	///
	/// * `name` - Name of the workflow definition to be removed
	/// * `version` - Version of the workflow definition to be removed
	fn unregister_workflow_def(&self, name: &str, version: u32) -> Result<(), String>;

	/// Add a new event handler
	///
	/// # Arguments
	///
	/// * `event_handler` - The EventHandler to be added
	fn add_event_handler(&self, event_handler: EventHandler) -> Result<(), String>;

	/// Update an existing event handler
	///
	/// # Arguments
	///
	/// * `event_handler` - The EventHandler to be updated
	fn update_event_handler(&self, event_handler: EventHandler) -> Result<(), String>;

	/// Remove an event handler
	///
	/// # Arguments
	///
	/// * `name` - Name of the event handler to be removed
	fn remove_event_handler_status(&self, name: &str) -> Result<(), String>;

	/// Retrieve all registered event handlers
	///
	/// # Returns
	///
	/// A vector of all EventHandler
	fn get_all_event_handlers(&self) -> Vec<EventHandler>;

	/// Retrieve event handlers for a specific event
	///
	/// # Arguments
	///
	/// * `event` - Name of the event
	/// * `active_only` - If true, returns only active handlers
	///
	/// # Returns
	///
	/// A vector of EventHandler for the specified event
	fn get_event_handlers_for_event(&self, event: &str, active_only: bool) -> Vec<EventHandler>;

	/// Retrieve the latest versions of all workflow definitions
	///
	/// # Returns
	///
	/// A vector of the latest versions of all WorkflowDef
	fn get_workflow_defs_latest_versions(&self) -> Vec<WorkflowDef>;
}
