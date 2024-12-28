use super::Message;

//TODO: Implement the Observable trait
pub trait Observable {
	type Item;
}

/// Trait representing an observable queue.
pub trait ObservableQueue {
	/// Returns an observable for the queue.
	///
	/// # Returns
	///
	/// An `Observable` of `Message`. You will need to define or import the `Message` type and
	/// the `Observable` trait or struct according to your application's requirements.
	fn observe(&self) -> Box<dyn Observable<Item = Message>>;

	/// Returns the type of the queue.
	///
	/// # Returns
	///
	/// A `String` representing the type of the queue.
	fn get_type(&self) -> String;

	/// Returns the name of the queue.
	///
	/// # Returns
	///
	/// A `String` representing the name of the queue.
	fn get_name(&self) -> String;

	/// Returns the URI identifier for the queue.
	///
	/// # Returns
	///
	/// A `String` representing the URI identifier for the queue.
	fn get_uri(&self) -> String;

	/// Acknowledges the given messages.
	///
	/// # Parameters
	///
	/// - `messages`: A list of `Message` objects to be acknowledged.
	///
	/// # Returns
	///
	/// A `Vec<String>` containing the IDs of the messages that could not be acknowledged.
	fn ack(&self, messages: Vec<Message>) -> Vec<String>;

	/// Nacknowledges the given messages. Default implementation does nothing.
	///
	/// # Parameters
	///
	/// - `messages`: A list of `Message` objects to be nacknowledged.
	fn nack(&self, messages: Vec<Message>) {
		// Default implementation does nothing
	}

	/// Publishes the given messages to the queue.
	///
	/// # Parameters
	///
	/// - `messages`: A list of `Message` objects to be published.
	fn publish(&self, messages: Vec<Message>);

	/// Determines if the queue supports re-publishing messages for retriability.
	///
	/// # Returns
	///
	/// `true` if the queue must re-publish messages for retriability, `false` otherwise.
	fn re_publish_if_no_ack(&self) -> bool {
		false // Default implementation
	}

	/// Extends the lease of an unacknowledged message.
	///
	/// # Parameters
	///
	/// - `message`: The `Message` for which the timeout should be changed.
	/// - `unack_timeout`: Timeout in milliseconds to extend the unacknowledged lease.
	fn set_unack_timeout(&self, message: Message, unack_timeout: u64);

	/// Returns the size of the queue, i.e., the number of pending messages.
	///
	/// # Returns
	///
	/// The number of messages pending in the queue. This can be an approximation.
	fn size(&self) -> u64;

	/// Closes the queue instance before removing it from the queues. Default implementation does nothing.
	fn close(&self) {
		// Default implementation does nothing
	}
}
