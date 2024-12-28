use crate::Message;
use chrono::Duration;
use std::collections::HashMap;

pub trait QueueDao {
	/// Pushes a message to the queue.
	fn push(&self, queue_name: &str, id: &str, offset_time_in_seconds: u64);

	/// Pushes a message to the queue with a priority.
	fn push_with_priority(
		&self,
		queue_name: &str,
		id: &str,
		priority: u8,
		offset_time_in_seconds: u64,
	);

	/// Pushes a list of messages to the queue.
	fn push_messages(&self, queue_name: &str, messages: &[Message]);

	/// Pushes a message to the queue if it does not already exist.
	fn push_if_not_exists(&self, queue_name: &str, id: &str, offset_time_in_seconds: u64) -> bool;

	/// Pushes a message to the queue with a priority if it does not already exist.
	fn push_if_not_exists_with_priority(
		&self,
		queue_name: &str,
		id: &str,
		priority: u8,
		offset_time_in_seconds: u64,
	) -> bool;

	/// Pops a list of message IDs from the queue.
	fn pop(&self, queue_name: &str, count: usize, timeout: Duration) -> Vec<String>;

	/// Polls a list of messages from the queue.
	fn poll_messages(&self, queue_name: &str, count: usize, timeout: Duration) -> Vec<Message>;

	/// Removes a message from the queue.
	fn remove(&self, queue_name: &str, message_id: &str);

	/// Returns the size of the queue.
	fn get_size(&self, queue_name: &str) -> usize;

	/// Acknowledges a message in the queue.
	fn ack(&self, queue_name: &str, message_id: &str) -> bool;

	/// Extends the lease of an unacknowledged message.
	fn set_unack_timeout(
		&self,
		queue_name: &str,
		message_id: &str,
		unack_timeout: Duration,
	) -> bool;

	/// Flushes the queue.
	fn flush(&self, queue_name: &str);

	/// Provides detailed information about the queues.
	fn queues_detail(&self) -> HashMap<String, usize>;

	/// Provides detailed verbose information about the queues.
	fn queues_detail_verbose(&self) -> HashMap<String, HashMap<String, HashMap<String, usize>>>;

	/// Processes unacknowledged messages (optional).
	fn process_unacks(&self, queue_name: &str);

	/// Resets the offset time of a message.
	fn reset_offset_time(&self, queue_name: &str, id: &str) -> bool;

	/// Postpones a message by removing and re-adding it with a new delay.
	fn postpone(
		&self,
		queue_name: &str,
		message_id: &str,
		priority: u8,
		postpone_duration_in_seconds: u64,
	) -> bool {
		self.remove(queue_name, message_id);
		self.push_with_priority(queue_name, message_id, priority, postpone_duration_in_seconds);
		true
	}

	/// Checks if the queue contains a message.
	fn contains_message(&self, queue_name: &str, message_id: &str) -> bool {
		panic!("Please ensure your provided Queue implementation overrides and implements this method.");
	}
}
