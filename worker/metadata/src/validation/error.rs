use std::fmt;

/// Captures a validation error that can be returned in `ErrorResponse`.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
	path: Option<String>,
	message: Option<String>,
	invalid_value: Option<String>,
}

impl ValidationError {
	pub fn new() -> Self {
		ValidationError {
			path: None,
			message: None,
			invalid_value: None,
		}
	}

	pub fn with_values(path: String, message: String, invalid_value: String) -> Self {
		ValidationError {
			path: Some(path),
			message: Some(message),
			invalid_value: Some(invalid_value),
		}
	}

	pub fn path(&self) -> Option<&str> {
		self.path.as_deref()
	}

	pub fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}

	pub fn invalid_value(&self) -> Option<&str> {
		self.invalid_value.as_deref()
	}

	pub fn set_path(&mut self, path: String) {
		self.path = Some(path);
	}

	pub fn set_message(&mut self, message: String) {
		self.message = Some(message);
	}

	pub fn set_invalid_value(&mut self, invalid_value: String) {
		self.invalid_value = Some(invalid_value);
	}
}

impl Default for ValidationError {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Display for ValidationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"ValidationError {{ path: {:?}, message: {:?}, invalid_value: {:?} }}",
			self.path, self.message, self.invalid_value
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_validation_error() {
		let mut error = ValidationError::new();
		error.set_path("test.path".to_string());
		error.set_message("Invalid input".to_string());
		error.set_invalid_value("123".to_string());

		assert_eq!(error.path(), Some("test.path"));
		assert_eq!(error.message(), Some("Invalid input"));
		assert_eq!(error.invalid_value(), Some("123"));

		let error2 = ValidationError::with_values(
			"test.path".to_string(),
			"Invalid input".to_string(),
			"123".to_string(),
		);

		assert_eq!(error, error2);
	}
}
