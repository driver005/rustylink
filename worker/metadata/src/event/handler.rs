use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Define the Action enum and the various nested structs used in EventHandler
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
	StartWorkflow,
	CompleteTask,
	FailTask,
	TerminateWorkflow,
	UpdateWorkflowVariables,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartWorkflow {
	pub name: String,
	pub version: Option<u32>,
	pub correlation_id: Option<String>,
	pub input: HashMap<String, Value>,
	pub task_to_domain: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDetails {
	pub workflow_id: Option<String>,
	pub task_ref_name: Option<String>,
	pub output: HashMap<String, Value>,
	pub task_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerminateWorkflow {
	pub workflow_id: Option<String>,
	pub termination_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWorkflowVariables {
	pub workflow_id: Option<String>,
	pub variables: HashMap<String, Value>,
	pub append_array: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
	pub action: ActionType,
	pub start_workflow: Option<StartWorkflow>,
	pub complete_task: Option<TaskDetails>,
	pub fail_task: Option<TaskDetails>,
	pub expand_inline_json: Option<bool>,
	pub terminate_workflow: Option<TerminateWorkflow>,
	pub update_workflow_variables: Option<UpdateWorkflowVariables>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventHandler {
	pub name: String,
	pub event: String,
	pub condition: Option<String>,
	pub actions: Vec<Action>,
	pub active: bool,
	pub evaluator_type: Option<String>,
}
