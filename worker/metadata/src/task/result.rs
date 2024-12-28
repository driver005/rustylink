use super::TaskExecLog;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskResultStatus {
	InProgress,
	Failed,
	FailedWithTerminalError,
	Completed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskResult {
	pub workflow_instance_id: String,
	pub task_id: String,
	pub reason_for_incompletion: Option<String>,
	pub callback_after_seconds: i64,
	pub worker_id: Option<String>,
	pub status: TaskResultStatus,
	pub output_data: serde_json::Value,
	pub output_message: Option<Vec<u8>>, // Equivalent to Any in protobuf
	pub logs: VecDeque<TaskExecLog>,
	pub external_output_payload_storage_path: Option<String>,
	pub sub_workflow_id: Option<String>,
	pub extend_lease: bool,
}

impl TaskResult {
	pub fn new() -> Self {
		Self {
			workflow_instance_id: String::new(),
			task_id: String::new(),
			reason_for_incompletion: None,
			callback_after_seconds: 0,
			worker_id: None,
			status: TaskResultStatus::InProgress,
			output_data: serde_json::Value::Null,
			output_message: None,
			logs: VecDeque::new(),
			external_output_payload_storage_path: None,
			sub_workflow_id: None,
			extend_lease: false,
		}
	}

	pub fn set_reason_for_incompletion(&mut self, reason: String) {
		self.reason_for_incompletion = Some(reason.chars().take(500).collect());
	}

	pub fn add_output_data(&mut self, value: serde_json::Value) -> &mut Self {
		// self.output_data.insert(key, value);
		todo!()
	}

	pub fn log(&mut self, log: String) -> &mut Self {
		self.logs.push_back(TaskExecLog::new(log));
		self
	}

	pub fn complete() -> Self {
		Self::new_with_TaskResultstatus(TaskResultStatus::Completed)
	}

	pub fn failed() -> Self {
		Self::new_with_TaskResultstatus(TaskResultStatus::Failed)
	}

	pub fn failed_with_reason(failure_reason: String) -> Self {
		let mut result = Self::new_with_TaskResultstatus(TaskResultStatus::Failed);
		result.set_reason_for_incompletion(failure_reason);
		result
	}

	pub fn in_progress() -> Self {
		Self::new_with_TaskResultstatus(TaskResultStatus::InProgress)
	}

	pub fn new_with_TaskResultstatus(TaskResultstatus: TaskResultStatus) -> Self {
		let mut result = Self::new();
		result.status = TaskResultstatus;
		result
	}
}
