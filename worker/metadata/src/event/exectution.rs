use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::ActionType;

// Define the Status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
	InProgress,
	Completed,
	Failed,
	Skipped,
}

// Define the EventExecution struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventExecution {
	pub id: String,
	pub message_id: String,
	pub name: String,
	pub event: String,
	pub created: i64, // or use `u64` if you prefer to represent it as an unsigned integer
	pub status: Status,
	pub action: ActionType,
	pub output: HashMap<String, serde_json::Value>,
}

impl EventExecution {
	pub fn new(id: String, message_id: String) -> Self {
		Self {
			id,
			message_id,
			name: String::new(),
			event: String::new(),
			created: 0,
			status: Status::InProgress,
			action: ActionType::StartWorkflow, // Default action, adjust as needed
			output: HashMap::new(),
		}
	}
}
