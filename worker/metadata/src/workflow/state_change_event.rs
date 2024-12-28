use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateChangeEvent {
	/// Event type, must not be null
	#[serde(rename = "type")]
	pub state_type: String,

	/// Payload associated with the event
	pub payload: Option<HashMap<String, serde_json::Value>>,
}

impl StateChangeEvent {
	pub fn new(state_type: String) -> Self {
		StateChangeEvent {
			state_type,
			payload: None,
		}
	}
}

impl std::fmt::Display for StateChangeEvent {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "StateChangeEvent{{ type='{}', payload={:?} }}", self.state_type, self.payload)
	}
}
