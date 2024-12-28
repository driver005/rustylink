use super::RateLimitConfig;
use crate::{SchemaDef, TimeoutPolicy, WorkflowTask};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDef {
	pub name: String,
	pub description: Option<String>,
	pub version: Option<u32>,
	pub tasks: Vec<WorkflowTask>,
	pub input_parameters: Option<Vec<String>>,
	pub output_parameters: Option<HashMap<String, Value>>,
	pub input_template: Option<HashMap<String, Value>>,
	pub failure_workflow: Option<String>,
	pub schema_version: u32,
	pub restartable: bool,
	pub workflow_status_listener_enabled: bool,
	pub workflow_status_listener_sink: Option<String>,
	pub owner_email: String,
	pub timeout_policy: TimeoutPolicy,
	pub timeout_seconds: Option<u64>,
	pub variables: Option<HashMap<String, Value>>,
	pub rate_limit_config: Option<RateLimitConfig>,
	pub input_schema: Option<SchemaDef>,
	pub output_schema: Option<SchemaDef>,
	pub enforce_schema: bool,
}

impl WorkflowDef {
	pub fn new(name: String, owner_email: String) -> Self {
		WorkflowDef {
			name,
			description: None,
			version: None,
			tasks: Vec::new(),
			input_parameters: None,
			output_parameters: None,
			failure_workflow: None,
			schema_version: 1,
			restartable: true,
			workflow_status_listener_enabled: false,
			owner_email,
			timeout_policy: TimeoutPolicy::AlertOnly,
			timeout_seconds: None,
			variables: None,
			input_template: None,
			workflow_status_listener_sink: None,
			rate_limit_config: None,
			input_schema: None,
			output_schema: None,
			enforce_schema: true,
		}
	}

	// pub fn key(&self) -> String {
	// 	format!("{}.{:?}", self.name, self.version)
	// }

	// pub fn collect_tasks(&self) -> Vec<WorkflowTask> {
	// 	self.tasks.iter().flat_map(|task| task.collect_tasks()).collect()
	// }

	// pub fn get_task_by_ref_name(&self, task_reference_name: &str) -> Option<WorkflowTask> {
	// 	self.collect_tasks()
	// 		.iter()
	// 		.find(|task| task.task_reference_name == task_reference_name)
	// 		.cloned()
	// }

	// pub fn get_next_task(&self, task_reference_name: &str) -> Option<WorkflowTask> {
	// 	let task = self.get_task_by_ref_name(task_reference_name)?;
	// 	if task.task_type == TaskType::Terminate {
	// 		return None;
	// 	}

	// 	let mut iterator = self.tasks.iter();
	// 	while let Some(t) = iterator.next() {
	// 		if t.task_reference_name == task_reference_name {
	// 			break;
	// 		}
	// 		let next_task = t.next(task_reference_name, None);
	// 		if next_task.is_some() {
	// 			return next_task;
	// 		} else if t.task_type == TaskType::DoWhile
	// 			&& t.task_reference_name != task_reference_name
	// 			&& t.has(task_reference_name)
	// 		{
	// 			return None;
	// 		}

	// 		if t.has(task_reference_name) {
	// 			break;
	// 		}
	// 	}

	// 	match iterator.next() {
	// 		Some(next_task) => Some(next_task.clone()),
	// 		None => None,
	// 	}
	// }
}
