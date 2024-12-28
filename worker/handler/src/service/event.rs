use metadata::EventHandler;

pub trait EventService {
	/// Add a new event handler.
	///
	/// # Arguments
	///
	/// * `event_handler` - Instance of `EventHandler`
	///
	/// # Errors
	///
	/// Returns an error if the `event_handler` is invalid.
	fn add_event_handler(&self, event_handler: EventHandler) -> Result<(), String>;

	/// Update an existing event handler.
	///
	/// # Arguments
	///
	/// * `event_handler` - Instance of `EventHandler`
	///
	/// # Errors
	///
	/// Returns an error if the `event_handler` is invalid.
	fn update_event_handler(&self, event_handler: EventHandler) -> Result<(), String>;

	/// Remove an event handler.
	///
	/// # Arguments
	///
	/// * `name` - Event name
	///
	/// # Errors
	///
	/// Returns an error if the `name` is empty.
	fn remove_event_handler_status(&self, name: &str) -> Result<(), String>;

	/// Get all the event handlers.
	///
	/// # Returns
	///
	/// Returns a list of `EventHandler`s.
	fn get_event_handlers(&self) -> Vec<EventHandler>;

	/// Get event handlers for a given event.
	///
	/// # Arguments
	///
	/// * `event` - Event Name
	/// * `active_only` - `true` for active only events, `false` otherwise
	///
	/// # Returns
	///
	/// Returns a list of `EventHandler`s.
	///
	/// # Errors
	///
	/// Returns an error if the `event` is empty.
	fn get_event_handlers_for_event(
		&self,
		event: &str,
		active_only: bool,
	) -> Result<Vec<EventHandler>, String>;
}
