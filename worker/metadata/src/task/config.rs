use crate::{Error, Result, StateChangeEvent, SubWorkflowParams, TaskDef, TaskType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents a task definition as part of a workflow
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTask {
	pub name: String,
	pub task_reference_name: String,
	pub task_type: TaskType,
	pub description: Option<String>,
	#[serde(default)]
	pub optional: bool,
	input_parameters: HashMap<String, serde_json::Value>,
	#[serde(default)]
	pub async_complete: bool,
	#[serde(default)]
	pub start_delay: u64,
	#[serde(default)]
	pub permissive: bool,
	pub loop_condition: Option<String>,
	pub loop_over: Option<Vec<WorkflowTask>>,
	pub dynamic_task_name_param: Option<String>,
	pub dynamic_fork_tasks_param: Option<String>,
	pub dynamic_fork_tasks_input_param_name: Option<String>,
	pub fork_tasks: Option<Vec<Vec<WorkflowTask>>>,
	pub join_on: Option<Vec<String>>,
	pub join_status: Option<String>,
	pub sub_workflow_param: Option<SubWorkflowParams>,
	pub decision_cases: Option<HashMap<String, Vec<WorkflowTask>>>,
	pub default_case: Option<Vec<WorkflowTask>>,
	pub evaluator_type: Option<String>,
	pub expression: Option<String>,
	pub sink: Option<String>,
	pub trigger_failure_workflow: Option<bool>,
	pub script_expression: Option<String>,
	pub task_definition: Option<TaskDef>,
	pub rate_limited: Option<bool>,
	pub default_exclusive_join_task: Option<Vec<String>>,
	pub retry_count: Option<u32>,
	pub on_state_change: Option<HashMap<String, Vec<StateChangeEvent>>>,
	pub cache_config: Option<CacheConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CacheConfig {
	pub key: Option<String>,
	pub ttl_in_second: u32,
}

impl Default for WorkflowTask {
	fn default() -> Self {
		WorkflowTask {
			name: String::new(),
			task_reference_name: String::new(),
			description: None,
			input_parameters: HashMap::new(),
			task_type: TaskType::Simple,
			dynamic_task_name_param: None,
			script_expression: None,
			decision_cases: None,
			dynamic_fork_tasks_param: None,
			dynamic_fork_tasks_input_param_name: None,
			default_case: None,
			fork_tasks: None,
			start_delay: 0,
			sub_workflow_param: None,
			join_on: None,
			sink: None,
			optional: false,
			task_definition: None,
			rate_limited: None,
			default_exclusive_join_task: None,
			async_complete: false,
			loop_condition: None,
			loop_over: None,
			retry_count: None,
			evaluator_type: None,
			expression: None,
			on_state_change: None,
			join_status: None,
			cache_config: None,
			permissive: false,
			trigger_failure_workflow: None,
		}
	}
}

impl fmt::Display for WorkflowTask {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.name, self.task_reference_name)
	}
}

impl WorkflowTask {
	pub fn new(
		name: String,
		task_reference_name: String,
		task_type: TaskType,
		input_parameters: HashMap<String, serde_json::Value>,
		async_complete: bool,
		start_delay: u64,
		permissive: bool,
	) -> Self {
		WorkflowTask {
			name,
			task_reference_name,
			input_parameters,
			task_type,
			async_complete,
			start_delay,
			permissive,
			description: None,
			dynamic_task_name_param: None,
			script_expression: None,
			decision_cases: None,
			dynamic_fork_tasks_param: None,
			dynamic_fork_tasks_input_param_name: None,
			default_case: None,
			fork_tasks: None,
			sub_workflow_param: None,
			join_on: None,
			sink: None,
			optional: false,
			task_definition: None,
			rate_limited: None,
			default_exclusive_join_task: None,
			loop_condition: None,
			loop_over: None,
			retry_count: None,
			evaluator_type: None,
			expression: None,
			on_state_change: None,
			join_status: None,
			cache_config: None,
			trigger_failure_workflow: None,
		}
	}

	pub fn get_input_parameter_required(&self, name: &str) -> Result<&serde_json::Value> {
		self.input_parameters
			.get(name)
			.ok_or(Error::IllegalArgument(format!("Missing input parameter: {name}")))
	}

	pub fn get_input_parameter_optinal(&self, name: &str) -> Option<&serde_json::Value> {
		self.input_parameters.get(name)
	}

	// pub fn collect_tasks(&self) -> Vec<WorkflowTask> {
	// 	let mut tasks = Vec::new();
	// 	tasks.push(self.clone());
	// 	for child_list in self.children() {
	// 		for task in child_list {
	// 			tasks.extend(task.collect_tasks());
	// 		}
	// 	}
	// 	tasks
	// }

	// fn children(&self) -> Vec<Vec<WorkflowTask>> {
	// 	let mut result = Vec::new();
	// 	match self.task_type {
	// 		TaskType::Decision | TaskType::Switch => {
	// 			result.extend(self.decision_cases.values().cloned());
	// 			result.push(self.default_case.clone());
	// 		}
	// 		TaskType::ForkJoin => {
	// 			result.extend(self.fork_tasks.clone());
	// 		}
	// 		TaskType::DoWhile => {
	// 			result.push(self.loop_over.clone());
	// 		}
	// 		_ => {}
	// 	}
	// 	result
	// }

	// pub fn next(
	// 	&self,
	// 	task_reference_name: &str,
	// 	parent: Option<&WorkflowTask>,
	// ) -> Option<WorkflowTask> {
	// 	match self.task_type {
	// 		TaskType::Decision | TaskType::Switch => {
	// 			for child_list in self.children() {
	// 				for task in child_list {
	// 					if task.task_reference_name == task_reference_name {
	// 						return Some(task);
	// 					}
	// 					if let Some(next_task) = task.next(task_reference_name, Some(self)) {
	// 						return Some(next_task);
	// 					}
	// 					if task.has(task_reference_name) {
	// 						break;
	// 					}
	// 				}
	// 			}
	// 			if self.has(task_reference_name) {
	// 				return Some(self.clone());
	// 			}
	// 		}
	// 		TaskType::ForkJoin => {
	// 			for child_list in self.children() {
	// 				for task in child_list {
	// 					if task.task_reference_name == task_reference_name {
	// 						if let Some(parent) = parent {
	// 							return parent.next(&self.task_reference_name, Some(parent));
	// 						}
	// 					}
	// 				}
	// 			}
	// 		}
	// 		_ => {}
	// 	}
	// 	None
	// }

	// pub fn has(&self, task_reference_name: &str) -> bool {
	// 	if self.task_reference_name == task_reference_name {
	// 		return true;
	// 	}
	// 	self.children()
	// 		.iter()
	// 		.any(|child_list| child_list.iter().any(|task| task.has(task_reference_name)))
	// }

	// pub fn get(&self, task_reference_name: &str) -> Option<WorkflowTask> {
	// 	if self.task_reference_name == task_reference_name {
	// 		return Some(self.clone());
	// 	}
	// 	self.children()
	// 		.iter()
	// 		.find_map(|child_list| child_list.iter().find_map(|task| task.get(task_reference_name)))
	// }
}
