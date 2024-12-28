use metadata::{TaskDef, WorkflowDef};

pub trait MetadataDao {
	// Create a task definition
	fn create_task_def(&self, task_def: TaskDef) -> TaskDef;

	// Update a task definition
	fn update_task_def(&self, task_def: TaskDef) -> TaskDef;

	// Get a task definition by name
	fn get_task_def(&self, name: &str) -> Option<TaskDef>;

	// Get all task definitions
	fn get_all_task_defs(&self) -> Vec<TaskDef>;

	// Remove a task definition by name
	fn remove_task_def(&self, name: &str);

	// Create a workflow definition
	fn create_workflow_def(&self, def: WorkflowDef);

	// Update a workflow definition
	fn update_workflow_def(&self, def: WorkflowDef);

	// Get the latest workflow definition by name
	fn get_latest_workflow_def(&self, name: &str) -> Option<WorkflowDef>;

	// Get a specific version of a workflow definition
	fn get_workflow_def(&self, name: &str, version: i32) -> Option<WorkflowDef>;

	// Remove a workflow definition by name and version
	fn remove_workflow_def(&self, name: &str, version: i32);

	// Get all workflow definitions
	fn get_all_workflow_defs(&self) -> Vec<WorkflowDef>;

	// Get the latest versions of all workflow definitions
	fn get_all_workflow_defs_latest_versions(&self) -> Vec<WorkflowDef>;
}
