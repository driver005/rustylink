use crate::{TaskStatus, WorkflowTask};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
	pub task_type: String,
	pub status: TaskStatus,
	pub input_data: HashMap<String, serde_json::Value>,
	pub reference_task_name: Option<String>,
	pub retry_count: i32,
	pub seq: i32,
	pub correlation_id: Option<String>,
	pub poll_count: i32,
	pub task_def_name: Option<String>,
	pub scheduled_time: i64,
	pub start_time: i64,
	pub end_time: i64,
	pub update_time: i64,
	pub start_delay_in_seconds: i32,
	pub retried_task_id: Option<String>,
	pub retried: bool,
	pub executed: bool,
	pub callback_from_worker: bool,
	pub response_timeout_seconds: i64,
	pub workflow_instance_id: Option<String>,
	pub workflow_type: Option<String>,
	pub task_id: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub callback_after_seconds: i64,
	pub worker_id: Option<String>,
	pub output_data: HashMap<String, serde_json::Value>,
	pub workflow_task: Option<WorkflowTask>,
	pub domain: Option<String>,
	pub rate_limit_per_frequency: i32,
	pub rate_limit_frequency_in_seconds: i32,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub workflow_priority: i32,
	pub execution_name_space: Option<String>,
	pub isolation_group_id: Option<String>,
	pub iteration: i32,
	pub sub_workflow_id: Option<String>,
	pub subworkflow_changed: bool,
	pub parent_task_id: Option<String>,
}

impl Task {
	pub fn increment_poll_count(&mut self) {
		self.poll_count += 1;
	}

	pub fn queue_wait_time(&self) -> i64 {
		if self.start_time > 0 && self.scheduled_time > 0 {
			if self.update_time > 0 && self.callback_after_seconds > 0 {
				let wait_time = std::time::SystemTime::now()
					.duration_since(std::time::UNIX_EPOCH)
					.unwrap()
					.as_millis() as i64 - (self.update_time
					+ (self.callback_after_seconds * 1000));
				if wait_time > 0 {
					return wait_time;
				}
			} else {
				return self.start_time - self.scheduled_time;
			}
		}
		0
	}

	pub fn is_loop_over_task(&self) -> bool {
		self.iteration > 0
	}
}
