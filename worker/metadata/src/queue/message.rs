use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
	payload: String,
	id: String,
	receipt: String,
	priority: u32,
}

impl Message {
	pub fn new(id: String, payload: String, receipt: String, priority: u32) -> Self {
		Self {
			id,
			payload,
			receipt,
			priority,
		}
	}
}
