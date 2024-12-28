use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpgradeWorkflowRequest {
	pub name: String,
	pub version: Option<u32>,
	pub workflow_input: Option<HashMap<String, serde_json::Value>>,
	pub task_output: Option<HashMap<String, serde_json::Value>>,
}

impl UpgradeWorkflowRequest {
	pub fn new(
		name: String,
		version: Option<u32>,
		workflow_input: Option<HashMap<String, serde_json::Value>>,
		task_output: Option<HashMap<String, serde_json::Value>>,
	) -> Self {
		UpgradeWorkflowRequest {
			name,
			version,
			workflow_input,
			task_output,
		}
	}
}
