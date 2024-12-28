use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollData {
	pub queue_name: String,
	pub domain: String,
	pub worker_id: String,
	pub last_poll_time: i64,
}

impl PollData {
	pub fn new(queue_name: String, domain: String, worker_id: String, last_poll_time: i64) -> Self {
		PollData {
			queue_name,
			domain,
			worker_id,
			last_poll_time,
		}
	}
}
