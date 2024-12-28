use std::collections::HashMap;

/// Trait for managing event queues.
pub trait EventQueueManager {
	/// Retrieves a map of queue names to their respective queue identifiers.
	///
	/// # Returns
	///
	/// A `HashMap` where the keys are `String` representing queue names, and the values are `String` representing
	/// queue identifiers.
	fn get_queues(&self) -> HashMap<String, String>;

	/// Retrieves a map of queue sizes.
	///
	/// # Returns
	///
	/// A `HashMap` where the keys are `String` representing queue names. Each value is another `HashMap` where
	/// the keys are `String` representing specific queue attributes, and the values are `u64` representing the
	/// size of the queue attribute.
	fn get_queue_sizes(&self) -> HashMap<String, HashMap<String, u64>>;
}
