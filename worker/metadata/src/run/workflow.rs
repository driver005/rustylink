use crate::{Task, WorkflowDef, WorkflowStatus};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, LinkedList};

// Define the Workflow struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
	pub status: WorkflowStatus,
	pub end_time: i64,
	pub workflow_id: String,
	pub parent_workflow_id: Option<String>,
	pub parent_workflow_task_id: Option<String>,
	pub tasks: LinkedList<Task>,
	pub input: HashMap<String, serde_json::Value>,
	pub output: HashMap<String, serde_json::Value>,
	pub correlation_id: Option<String>,
	pub re_run_from_workflow_id: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub event: Option<String>,
	pub task_to_domain: HashMap<String, String>,
	pub failed_reference_task_names: HashSet<String>,
	pub workflow_definition: Option<WorkflowDef>,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub priority: i32,
	pub variables: HashMap<String, serde_json::Value>,
	pub last_retried_time: i64,
	pub failed_task_names: HashSet<String>,
	pub history: LinkedList<Workflow>,
	pub idempotency_key: Option<String>,
	pub rate_limit_key: Option<String>,
	pub rate_limited: bool,
}

impl Workflow {
	pub fn new() -> Self {
		Workflow {
			status: WorkflowStatus::Running,
			end_time: 0,
			workflow_id: String::new(),
			parent_workflow_id: None,
			parent_workflow_task_id: None,
			tasks: LinkedList::new(),
			input: HashMap::new(),
			output: HashMap::new(),
			correlation_id: None,
			re_run_from_workflow_id: None,
			reason_for_incompletion: None,
			event: None,
			task_to_domain: HashMap::new(),
			failed_reference_task_names: HashSet::new(),
			workflow_definition: None,
			external_input_payload_storage_path: None,
			external_output_payload_storage_path: None,
			priority: 0,
			variables: HashMap::new(),
			last_retried_time: 0,
			failed_task_names: HashSet::new(),
			history: LinkedList::new(),
			idempotency_key: None,
			rate_limit_key: None,
			rate_limited: false,
		}
	}
}
