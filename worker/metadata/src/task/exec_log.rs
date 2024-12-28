use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskExecLog {
	log: String,
	task_id: Option<String>,
	created_time: u64,
}

impl TaskExecLog {
	pub fn new(log: String) -> Self {
		let created_time =
			SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
				as u64;

		Self {
			log,
			task_id: None,
			created_time,
		}
	}
}
