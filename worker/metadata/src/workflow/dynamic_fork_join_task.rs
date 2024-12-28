use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicForkJoinTask {
	pub task_name: String,
	pub workflow_name: String,
	pub reference_name: String,
	pub input: HashMap<String, Value>,
	pub task_type: String, // In Rust, `type` is a reserved keyword, so renamed to `task_type`
}

impl DynamicForkJoinTask {
	pub fn new(
		task_name: String,
		workflow_name: String,
		reference_name: String,
		input: HashMap<String, Value>,
	) -> DynamicForkJoinTask {
		DynamicForkJoinTask {
			task_name,
			workflow_name,
			reference_name,
			input,
			task_type: "SIMPLE".to_string(), // Default to SIMPLE
		}
	}

	pub fn with_type(
		task_name: String,
		workflow_name: String,
		reference_name: String,
		task_type: String,
		input: HashMap<String, Value>,
	) -> DynamicForkJoinTask {
		DynamicForkJoinTask {
			task_name,
			workflow_name,
			reference_name,
			input,
			task_type,
		}
	}
}
