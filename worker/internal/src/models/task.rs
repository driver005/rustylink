use chrono::{DateTime, Utc};
use metadata::{TaskStatus, TaskType, WorkflowTask};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

type Any = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskModel {
	pub task_id: String,
	pub task_type: TaskType,
	pub status: TaskStatus,
	pub reference_task_name: String,
	pub retry_count: Option<u32>,
	pub seq: i32,
	pub correlation_id: Option<String>,
	pub poll_count: i32,
	pub task_def_name: String,
	pub scheduled_time: DateTime<Utc>,
	pub start_time: DateTime<Utc>,
	pub end_time: Option<DateTime<Utc>>,
	pub update_time: Option<DateTime<Utc>>,
	pub start_delay_in_seconds: u64,
	pub retried_task_id: Option<String>,
	pub retried: bool,
	pub executed: bool,
	pub callback_from_worker: bool,
	pub response_timeout_seconds: Option<u64>,
	pub workflow_instance_id: Option<String>,
	pub workflow_type: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub callback_after_seconds: u64,
	pub worker_id: Option<String>,
	pub workflow_task: Arc<WorkflowTask>,
	pub domain: Option<String>,
	pub input_message: Option<Any>,
	pub output_message: Option<Any>,
	pub rate_limit_per_frequency: Option<u32>,
	pub rate_limit_frequency_in_seconds: Option<u32>,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub workflow_priority: Option<i32>,
	pub execution_name_space: Option<String>,
	pub isolation_group_id: Option<String>,
	pub iteration: i32,
	pub sub_workflow_id: Option<String>,
	pub subworkflow_changed: bool,
	pub wait_timeout: Option<i64>,
	#[serde(skip)]
	pub input_data: HashMap<String, serde_json::Value>,
	#[serde(skip)]
	pub output_data: HashMap<String, serde_json::Value>,
	#[serde(skip)]
	pub input_payload: HashMap<String, serde_json::Value>,
	#[serde(skip)]
	pub output_payload: HashMap<String, serde_json::Value>,
}

impl TaskModel {
	pub fn new(
		task_id: &str,
		task_type: TaskType,
		task_def_name: &str,
		reference_task_name: &str,
		correlation_id: Option<&str>,
		workflow_instance_id: &str,
		workflow_type: &str,
		scheduled_time: DateTime<Utc>,
		workflow_task: Arc<WorkflowTask>,
		workflow_priority: i32,
	) -> Self {
		Self {
			task_id: task_id.to_string(),
			task_type,
			status: TaskStatus::Scheduled,
			reference_task_name: reference_task_name.to_string(),
			retry_count: None,
			seq: 0,
			correlation_id: correlation_id.map(|c| c.to_string()),
			poll_count: 0,
			task_def_name: task_def_name.to_string(),
			scheduled_time,
			start_time: Utc::now(),
			end_time: None,
			update_time: None,
			start_delay_in_seconds: 0,
			retried_task_id: None,
			retried: false,
			executed: false,
			callback_from_worker: true,
			response_timeout_seconds: None,
			workflow_instance_id: Some(workflow_instance_id.to_string()),
			workflow_type: Some(workflow_type.to_string()),
			reason_for_incompletion: None,
			callback_after_seconds: 0,
			worker_id: None,
			workflow_task,
			domain: None,
			input_message: None,
			output_message: None,
			rate_limit_per_frequency: None,
			rate_limit_frequency_in_seconds: None,
			external_input_payload_storage_path: None,
			external_output_payload_storage_path: None,
			workflow_priority: Some(workflow_priority),
			execution_name_space: None,
			isolation_group_id: None,
			iteration: 0,
			sub_workflow_id: None,
			subworkflow_changed: false,
			wait_timeout: None,
			input_data: HashMap::new(),
			output_data: HashMap::new(),
			input_payload: HashMap::new(),
			output_payload: HashMap::new(),
		}
	}

	// pub fn increment_poll_count(&mut self) {
	// 	self.poll_count += 1;
	// }

	// pub fn is_loop_over_task(&self) -> bool {
	// 	self.iteration > 0
	// }

	// pub fn get_queue_wait_time(&self) -> u64 {
	// 	if self.start_time.timestamp() > 0 && self.scheduled_time.timestamp() > 0 {
	// 		if self.update_time.timestamp() > 0 && self.callback_after_seconds > 0 {
	// 			let wait_time = Utc::now().timestamp() as u64
	// 				- (self.update_time.timestamp() as u64 + self.callback_after_seconds);
	// 			if wait_time > 0 {
	// 				wait_time
	// 			} else {
	// 				0
	// 			}
	// 		} else {
	// 			(self.start_time.timestamp() - self.scheduled_time.timestamp()) as u64
	// 		}
	// 	} else {
	// 		0
	// 	}
	// }

	// pub fn externalize_input(&mut self, path: String) {
	// 	self.input_payload = std::mem::take(&mut self.input_data);
	// 	self.external_input_payload_storage_path = Some(path);
	// }

	// pub fn externalize_output(&mut self, path: String) {
	// 	self.output_payload = std::mem::take(&mut self.output_data);
	// 	self.external_output_payload_storage_path = Some(path);
	// }

	// pub fn internalize_input(&mut self, data: HashMap<String, serde_json::Value>) {
	// 	self.input_data.clear();
	// 	self.input_payload = data;
	// }

	// pub fn internalize_output(&mut self, data: HashMap<String, serde_json::Value>) {
	// 	self.output_data.clear();
	// 	self.output_payload = data;
	// }

	pub fn add_input(&mut self, key: String, value: serde_json::Value) {
		self.input_data.insert(key, value);
	}

	pub fn add_input_map(&mut self, input_data: HashMap<String, serde_json::Value>) {
		self.input_data.extend(input_data);
	}

	pub fn add_output(&mut self, key: String, value: serde_json::Value) {
		self.output_data.insert(key, value);
	}

	pub fn remove_output(&mut self, key: &str) {
		self.output_data.remove(key);
	}

	pub fn add_output_map(&mut self, output_data: HashMap<String, serde_json::Value>) {
		self.output_data.extend(output_data);
	}

	pub fn clear_output(&mut self) {
		self.output_data.clear();
		self.output_payload.clear();
		self.external_output_payload_storage_path = None;
	}

	pub fn add_to_queue(&self) {}

	pub fn rerun(&mut self, task_input: &Option<HashMap<String, serde_json::Value>>) {
		// reset fields before restarting the task
		self.scheduled_time = Utc::now();
		self.start_time = Utc::now();
		self.update_time = None;
		self.end_time = None;
		self.clear_output();
		self.retried = false;
		self.executed = false;

		if self.task_type == TaskType::SubWorkflow {
			self.status == TaskStatus::InProgress;
		} else {
			if let Some(input) = task_input {
				self.input_data = input.clone()
			}

			self.status = TaskStatus::Scheduled;
			self.add_to_queue();
		}
	}
}
