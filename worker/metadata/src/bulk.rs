use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct BulkResponse {
	/// Key - entity_id, Value - error message processing this entity
	bulk_error_results: HashMap<String, String>,
	bulk_successful_results: Vec<String>,
	message: String,
}

impl BulkResponse {
	pub fn new() -> Self {
		BulkResponse {
			bulk_error_results: HashMap::new(),
			bulk_successful_results: Vec::new(),
			message: String::from("Bulk Request has been processed."),
		}
	}

	pub fn get_bulk_successful_results(&self) -> &Vec<String> {
		&self.bulk_successful_results
	}

	pub fn get_bulk_error_results(&self) -> &HashMap<String, String> {
		&self.bulk_error_results
	}

	pub fn append_success_response(&mut self, id: String) {
		self.bulk_successful_results.push(id);
	}

	pub fn append_failed_response(&mut self, id: String, error_message: String) {
		self.bulk_error_results.insert(id, error_message);
	}
}

impl Default for BulkResponse {
	fn default() -> Self {
		Self::new()
	}
}

impl std::fmt::Display for BulkResponse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
            f,
            "BulkResponse {{ bulk_successful_results: {:?}, bulk_error_results: {:?}, message: {} }}",
            self.bulk_successful_results, self.bulk_error_results, self.message
        )
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bulk_response() {
		let mut response = BulkResponse::new();

		response.append_success_response("success1".to_string());
		response.append_success_response("success2".to_string());
		response.append_failed_response("error1".to_string(), "Error message 1".to_string());

		assert_eq!(
			response.get_bulk_successful_results(),
			&vec!["success1".to_string(), "success2".to_string()]
		);
		assert_eq!(
			response.get_bulk_error_results().get("error1"),
			Some(&"Error message 1".to_string())
		);
	}
}
