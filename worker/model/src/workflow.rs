use super::TaskModel;
use chrono::{DateTime, Utc};
use metadata::{Error, Result, WorkflowDef, WorkflowStatus};
use std::{
	collections::{HashMap, HashSet, VecDeque},
	sync::Arc,
};

#[derive(Debug, Clone)]
pub struct WorkflowModel {
	pub status: WorkflowStatus,
	pub end_time: DateTime<Utc>,
	pub workflow_id: String,
	pub parent_workflow_id: Option<String>,
	pub parent_workflow_task_id: Option<String>,
	pub tasks: VecDeque<TaskModel>,
	pub correlation_id: Option<String>,
	pub re_run_from_workflow_id: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub event: Option<String>,
	pub task_to_domain: Option<HashMap<String, String>>,
	pub failed_reference_task_names: HashSet<String>,
	pub failed_task_names: HashSet<String>,
	pub workflow_definition: Option<Arc<WorkflowDef>>,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub priority: u8,
	pub variables: Option<HashMap<String, serde_json::Value>>,
	pub last_retried_time: DateTime<Utc>,
	pub owner_app: Option<String>,
	pub create_time: DateTime<Utc>,
	pub updated_time: Option<DateTime<Utc>>,
	pub created_by: Option<String>,
	pub updated_by: Option<String>,
	pub failed_task_id: Option<String>,
	pub previous_status: Option<WorkflowStatus>,
	pub input: HashMap<String, serde_json::Value>,
	pub output: HashMap<String, serde_json::Value>,
	pub input_payload: HashMap<String, serde_json::Value>,
	pub output_payload: HashMap<String, serde_json::Value>,
}

impl WorkflowModel {
	pub fn new(workflow_id: String) -> Self {
		Self {
			status: WorkflowStatus::Running,
			end_time: Utc::now(),
			workflow_id,
			parent_workflow_id: None,
			parent_workflow_task_id: None,
			tasks: VecDeque::new(),
			correlation_id: None,
			re_run_from_workflow_id: None,
			reason_for_incompletion: None,
			event: None,
			task_to_domain: None,
			failed_reference_task_names: HashSet::new(),
			failed_task_names: HashSet::new(),
			workflow_definition: None,
			external_input_payload_storage_path: None,
			external_output_payload_storage_path: None,
			priority: 0,
			variables: None,
			last_retried_time: Utc::now(),
			owner_app: None,
			create_time: Utc::now(),
			updated_time: None,
			created_by: None,
			updated_by: None,
			failed_task_id: None,
			previous_status: None,
			input: HashMap::new(),
			output: HashMap::new(),
			input_payload: HashMap::new(),
			output_payload: HashMap::new(),
		}
	}

	pub fn get_workflow_definition(&self) -> Result<Arc<WorkflowDef>> {
		self.workflow_definition
			.clone()
			.ok_or(Error::illegal_argument("Missing workflow definition"))
	}
}
