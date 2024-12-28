use metadata::PollData;

pub trait PollDataDao {
	/// Updates the `PollData` information with the most recently polled data for a task queue.
	fn update_last_poll_data(&self, task_def_name: &str, domain: &str, worker_id: &str);

	/// Retrieve the `PollData` for the given task in the given domain.
	fn get_poll_data(&self, task_def_name: &str, domain: &str) -> PollData;

	/// Retrieve the `PollData` for the given task across all domains.
	fn get_poll_data_for_all_domains(&self, task_def_name: &str) -> Vec<PollData>;

	/// Retrieve the `PollData` for all task types.
	fn get_all_poll_data(&self) -> Result<Vec<PollData>, String> {
		Err(format!(
			"The selected PollDataDAO ({}) does not implement the get_all_poll_data() method",
			self.get_type_name()
		))
	}

	/// Helper method to get the type name of the implementing struct.
	fn get_type_name(&self) -> String {
		self.type_name()
	}

	/// Placeholder for actual implementation
	fn type_name(&self) -> String;
}
