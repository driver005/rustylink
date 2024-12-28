use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitConfig {
	/// Key that defines the rate limit (a combination of workflow payload like name or correlationId)
	pub rate_limit_key: String,

	/// Number of concurrently running workflows allowed per key
	pub concurrent_exec_limit: usize,
}

impl RateLimitConfig {
	// Constructor for easy initialization
	pub fn new(rate_limit_key: String, concurrent_exec_limit: usize) -> Self {
		RateLimitConfig {
			rate_limit_key,
			concurrent_exec_limit,
		}
	}
}
