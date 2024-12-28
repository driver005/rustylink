use metadata::WorkflowStatus;
use model::TaskModel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("IllegalArgument: {0}")]
	IllegalArgument(String),

	#[error("Conflict: {0}")]
	Conflict(String),

	#[error("Not Found: {0}")]
	NotFound(String),

	#[error("Non-Transient Error: {0}")]
	NonTransient(String),

	#[error("Transient Error: {0}")]
	Transient(String),

	#[error("Terminate Workflow: {reason}, Status: {workflow_status:?}, Task: {task:?}")]
	TerminateWorkflow {
		reason: String,
		workflow_status: WorkflowStatus,
		task: Option<TaskModel>,
	},

	#[error("Workflow: {0}")]
	Workflow(String),
}

impl Error {
	// Conflict error constructors
	pub fn illegal_argument(reason: &str) -> Self {
		Error::IllegalArgument(reason.to_string())
	}

	// Conflict error constructors
	pub fn conflict(reason: &str) -> Self {
		Error::Conflict(reason.to_string())
	}

	// Non-transient error constructors
	pub fn non_transient(reason: &str) -> Self {
		Error::NonTransient(reason.to_string())
	}

	// Not-found error constructors
	pub fn not_found(reason: &str) -> Self {
		Error::NotFound(reason.to_string())
	}

	// Terminate workflow error constructors
	pub fn terminate_workflow(
		reason: &str,
		workflow_status: WorkflowStatus,
		task: Option<TaskModel>,
	) -> Self {
		Error::TerminateWorkflow {
			reason: reason.to_string(),
			workflow_status,
			task,
		}
	}

	// Transient error constructors
	pub fn transient(reason: &str) -> Self {
		Error::Transient(reason.to_string())
	}
}
