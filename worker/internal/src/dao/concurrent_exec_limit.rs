use model::TaskModel;

pub trait ConcurrentExecutionLimitDao {
	// Add a task to the concurrency limit tracking
	fn add_task_to_limit(&self, task: TaskModel) {
		panic!("{} does not support add_task_to_limit method.", std::any::type_name::<Self>());
	}

	// Remove a task from the concurrency limit tracking
	fn remove_task_from_limit(&self, task: TaskModel) {
		panic!("{} does not support remove_task_from_limit method.", std::any::type_name::<Self>());
	}

	// Check if adding this task will exceed the concurrency limit
	fn exceeds_limit(&self, task: &TaskModel) -> bool;
}
