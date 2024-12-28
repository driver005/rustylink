use metadata::TaskDef;

use model::TaskModel;

pub trait RateLimitingDao {
	/// Checks if the task exceeds its rate limit based on the rate limit frequency and count.
	fn exceeds_rate_limit_per_frequency(&self, task: &TaskModel, task_def: &TaskDef) -> bool;
}
