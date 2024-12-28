use metadata::EventHandler;

pub trait EventHandlerDao {
	// Add a new event handler
	fn add_event_handler(&self, event_handler: EventHandler);

	// Update an existing event handler
	fn update_event_handler(&self, event_handler: EventHandler);

	// Remove an event handler by its name
	fn remove_event_handler(&self, name: &str);

	// Retrieve all event handlers
	fn get_all_event_handlers(&self) -> Vec<EventHandler>;

	// Get event handlers for a specific event, optionally only active ones
	fn get_event_handlers_for_event(&self, event: &str, active_only: bool) -> Vec<EventHandler>;
}
