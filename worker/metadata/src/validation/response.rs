use std::collections::HashMap;

use super::ValidationError;

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorResponse {
	status: i32,
	code: Option<String>,
	message: Option<String>,
	instance: Option<String>,
	retryable: bool,
	validation_errors: Option<Vec<ValidationError>>,
	metadata: Option<HashMap<String, serde_json::Value>>,
}

impl ErrorResponse {
	pub fn new() -> Self {
		ErrorResponse {
			status: 0,
			code: None,
			message: None,
			instance: None,
			retryable: false,
			validation_errors: None,
			metadata: None,
		}
	}

	pub fn status(&self) -> i32 {
		self.status
	}

	pub fn set_status(&mut self, status: i32) {
		self.status = status;
	}

	pub fn code(&self) -> Option<&str> {
		self.code.as_deref()
	}

	pub fn set_code(&mut self, code: String) {
		self.code = Some(code);
	}

	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}

	pub fn set_message(&mut self, message: String) {
		self.message = Some(message);
	}

	pub fn instance(&self) -> Option<&str> {
		self.instance.as_deref()
	}

	pub fn set_instance(&mut self, instance: String) {
		self.instance = Some(instance);
	}

	pub fn is_retryable(&self) -> bool {
		self.retryable
	}

	pub fn set_retryable(&mut self, retryable: bool) {
		self.retryable = retryable;
	}

	pub fn validation_errors(&self) -> Option<&Vec<ValidationError>> {
		self.validation_errors.as_ref()
	}

	pub fn set_validation_errors(&mut self, validation_errors: Vec<ValidationError>) {
		self.validation_errors = Some(validation_errors);
	}

	pub fn metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
		self.metadata.as_ref()
	}

	pub fn set_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) {
		self.metadata = Some(metadata);
	}
}

impl Default for ErrorResponse {
	fn default() -> Self {
		Self::new()
	}
}
