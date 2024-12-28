use metadata::Action;
use std::any::Any;
use std::collections::HashMap;

/// Trait for processing actions.
pub trait ActionProcessor {
	/// Executes an action with the given parameters.
	///
	/// # Parameters
	///
	/// - `action`: An `Action` representing the specific action to execute.
	/// - `payload_object`: An object representing the payload for the action, which is a trait object to allow
	///   dynamic typing.
	/// - `event`: A `String` representing the event associated with the action.
	/// - `message_id`: A `String` representing the identifier for the message.
	///
	/// # Returns
	///
	/// A `HashMap` where the keys are `String` and the values are boxed trait objects (`Box<dyn Any>`). This allows
	/// for dynamic typing of the values, similar to Java's `Object` type.
	fn execute(
		&self,
		action: Action,
		payload_object: &dyn Any,
		event: &str,
		message_id: &str,
	) -> HashMap<String, Box<dyn Any>>;
}
