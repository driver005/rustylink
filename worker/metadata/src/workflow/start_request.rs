use crate::IdempotencyStrategy;

use super::WorkflowDef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartWorkflowRequest {
	/// Name of the workflow
	pub name: String,

	/// Version of the workflow
	pub version: Option<u32>,

	/// Correlation ID
	pub correlation_id: Option<String>,

	/// Input parameters for the workflow
	pub input: Option<HashMap<String, serde_json::Value>>,

	/// Mapping of tasks to domains
	pub task_to_domain: Option<HashMap<String, String>>,

	/// Workflow definition
	pub workflow_def: Option<WorkflowDef>,

	/// External input payload storage path
	pub external_input_payload_storage_path: Option<String>,

	/// Priority of the workflow
	#[serde(default)]
	pub priority: Option<u8>,

	/// Created by
	pub created_by: Option<String>,

	/// Idempotency key
	pub idempotency_key: Option<String>,

	/// Idempotency strategy
	pub idempotency_strategy: Option<IdempotencyStrategy>,
}

impl StartWorkflowRequest {
	pub fn new(name: String) -> Self {
		StartWorkflowRequest {
			name,
			version: None,
			correlation_id: None,
			input: None,
			task_to_domain: None,
			workflow_def: None,
			external_input_payload_storage_path: None,
			priority: Some(0),
			created_by: None,
			idempotency_key: None,
			idempotency_strategy: None,
		}
	}
}
