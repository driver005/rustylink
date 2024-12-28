use std::error::Error;

use super::ObservableQueue;

/// Trait for providing event queues.
pub trait EventQueueProvider {
	/// Returns the type of queue this provider handles.
	///
	/// # Returns
	///
	/// A `String` representing the type of queue.
	fn get_queue_type(&self) -> String;

	/// Creates or retrieves the `ObservableQueue` for the given `queue_uri`.
	///
	/// # Parameters
	///
	/// - `queue_uri`: A `String` representing the URI of the queue.
	///
	/// # Returns
	///
	/// An `ObservableQueue` implementation for the specified `queue_uri`.
	///
	/// # Errors
	///
	/// Returns an error if an `ObservableQueue` cannot be created or retrieved for the `queue_uri`.
	fn get_queue(&self, queue_uri: &str) -> Result<Box<dyn ObservableQueue>, Box<dyn Error>>;
}
