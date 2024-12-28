use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RerunWorkflowRequest {
	/// Workflow ID from which to re-run
	pub re_run_from_workflow_id: String,

	/// Input for the workflow
	pub workflow_input: HashMap<String, serde_json::Value>,

	/// Task ID from which to re-run
	pub re_run_from_task_id: String,

	/// Input for the task
	pub task_input: HashMap<String, serde_json::Value>,

	/// Correlation ID
	pub correlation_id: String,
}

impl RerunWorkflowRequest {
	// Constructor for easy initialization
	pub fn new(
		re_run_from_workflow_id: String,
		workflow_input: HashMap<String, serde_json::Value>,
		re_run_from_task_id: String,
		task_input: HashMap<String, serde_json::Value>,
		correlation_id: String,
	) -> Self {
		RerunWorkflowRequest {
			re_run_from_workflow_id,
			workflow_input,
			re_run_from_task_id,
			task_input,
			correlation_id,
		}
	}
}
