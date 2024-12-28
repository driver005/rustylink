use sea_orm::DbErr;
use std::num::ParseIntError;
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

	#[error("Terminate Workflow: {0}")]
	TerminateWorkflow(String),

	#[error("Workflow: {0}")]
	Workflow(String),

	#[error("DbError: {0}")]
	DbError(DbErr),
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Error::IllegalArgument(e.to_string())
	}
}

impl From<chrono::ParseError> for Error {
	fn from(e: chrono::ParseError) -> Self {
		Error::IllegalArgument(e.to_string())
	}
}

impl From<ParseIntError> for Error {
	fn from(e: ParseIntError) -> Self {
		Error::IllegalArgument(e.to_string())
	}
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
	pub fn terminate_workflow(reason: &str) -> Self {
		Error::TerminateWorkflow(reason.to_string())
	}

	// Transient error constructors
	pub fn transient(reason: &str) -> Self {
		Error::Transient(reason.to_string())
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
