use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult<T> {
	pub total_hits: u64,
	pub results: Vec<T>,
}

impl<T> SearchResult<T> {
	pub fn new(total_hits: u64, results: Vec<T>) -> Self {
		SearchResult {
			total_hits,
			results,
		}
	}
}
