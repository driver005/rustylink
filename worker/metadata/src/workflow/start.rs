use std::{collections::HashMap, sync::Arc};

use super::WorkflowDef;

#[derive(Debug, Clone, PartialEq)]
pub struct StartWorkflowInput {
	pub name: Option<String>,
	pub version: Option<i32>,
	pub workflow_definition: Option<Arc<WorkflowDef>>,
	pub workflow_input: Option<HashMap<String, serde_json::Value>>,
	pub external_input_payload_storage_path: Option<String>,
	pub correlation_id: Option<String>,
	pub priority: Option<u8>,
	pub parent_workflow_id: Option<String>,
	pub parent_workflow_task_id: Option<String>,
	pub event: Option<String>,
	pub task_to_domain: Option<HashMap<String, String>>,
	pub workflow_id: Option<String>,
	pub triggering_workflow_id: Option<String>,
}
