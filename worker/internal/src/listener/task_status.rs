use log::debug;
use metadata::Result;
use model::TaskModel;

/// Trait for listening to task status changes.
pub trait TaskStatusListener {
	/// Called when a task is scheduled.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the scheduled task.
	fn on_task_scheduled(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task is in progress.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the task in progress.
	fn on_task_in_progress(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task is canceled.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the canceled task.
	fn on_task_canceled(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task fails.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the failed task.
	fn on_task_failed(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task fails with a terminal error.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the task that failed with a terminal error.
	fn on_task_failed_with_terminal_error(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task is completed.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the completed task.
	fn on_task_completed(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task is completed with errors.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the task that was completed with errors.
	fn on_task_completed_with_errors(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task times out.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the timed-out task.
	fn on_task_timed_out(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}

	/// Called when a task is skipped.
	///
	/// # Parameters
	///
	/// - `task`: A reference to a `TaskModel` representing the skipped task.
	fn on_task_skipped(&self, task: &TaskModel) -> Result<()> {
		Ok(())
	}
}

pub struct DefaultTaskStatusListener;

impl TaskStatusListener for DefaultTaskStatusListener {
	fn on_task_scheduled(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is scheduled", task.task_id);
		Ok(())
	}

	fn on_task_canceled(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is canceled", task.task_id);
		Ok(())
	}

	fn on_task_completed(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is completed", task.task_id);
		Ok(())
	}

	fn on_task_completed_with_errors(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is completed with errors", task.task_id);
		Ok(())
	}

	fn on_task_failed(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is failed", task.task_id);
		Ok(())
	}

	fn on_task_failed_with_terminal_error(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is failed with terminal error", task.task_id);
		Ok(())
	}

	fn on_task_in_progress(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is in-progress", task.task_id);
		Ok(())
	}

	fn on_task_skipped(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is skipped", task.task_id);
		Ok(())
	}

	fn on_task_timed_out(&self, task: &TaskModel) -> Result<()> {
		debug!("Task {} is timed out", task.task_id);
		Ok(())
	}
}
