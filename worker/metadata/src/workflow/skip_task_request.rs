use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Any = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkipTaskRequest {
	/// Input for the task
	pub task_input: Option<HashMap<String, serde_json::Value>>,

	/// Output for the task
	pub task_output: Option<HashMap<String, serde_json::Value>>,

	/// Message containing the task input
	#[serde(skip)]
	pub task_input_message: Option<Any>,

	/// Message containing the task output
	#[serde(skip)]
	pub task_output_message: Option<Any>,
}

impl SkipTaskRequest {
	// Constructor for easy initialization
	pub fn new(
		task_input: Option<HashMap<String, serde_json::Value>>,
		task_output: Option<HashMap<String, serde_json::Value>>,
		task_input_message: Option<Any>,
		task_output_message: Option<Any>,
	) -> Self {
		SkipTaskRequest {
			task_input,
			task_output,
			task_input_message,
			task_output_message,
		}
	}
}
