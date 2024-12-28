use metadata::BulkResponse;
use std::result::Result;

/// Trait defining bulk operations for workflow management
pub trait WorkflowBulkService {
	/// Maximum number of workflow IDs that can be processed in a single request
	const MAX_REQUEST_ITEMS: usize = 1000;

	/// Pause multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to pause
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn pause_workflow(&self, workflow_ids: &[String]) -> BulkResponse;

	/// Resume multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to resume
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn resume_workflow(&self, workflow_ids: &[String]) -> BulkResponse;

	/// Restart multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to restart
	/// * `use_latest_definitions` - Whether to use the latest workflow definitions
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn restart(&self, workflow_ids: &[String], use_latest_definitions: bool) -> BulkResponse;

	/// Retry multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to retry
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn retry(&self, workflow_ids: &[String]) -> BulkResponse;

	/// Terminate multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to terminate
	/// * `reason` - Reason for termination
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn terminate(&self, workflow_ids: &[String], reason: &str) -> BulkResponse;

	/// Delete multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to delete
	/// * `archive_workflow` - Whether to archive the workflows before deletion
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn delete_workflow(&self, workflow_ids: &[String], archive_workflow: bool) -> BulkResponse;

	/// Terminate and remove multiple workflows
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to terminate and remove
	/// * `reason` - Reason for termination
	/// * `archive_workflow` - Whether to archive the workflows before removal
	///
	/// # Returns
	///
	/// A Result containing a BulkResponse on success, or an error message on failure
	fn terminate_remove(
		&self,
		workflow_ids: &[String],
		reason: &str,
		archive_workflow: bool,
	) -> BulkResponse;

	/// Validate the request
	///
	/// # Arguments
	///
	/// * `workflow_ids` - List of workflow IDs to validate
	///
	/// # Returns
	///
	/// Ok(()) if valid, Err with a message if invalid
	fn validate_request(&self, workflow_ids: &[String]) -> Result<(), String> {
		if workflow_ids.is_empty() {
			Err("WorkflowIds list cannot be null or empty.".to_string())
		} else if workflow_ids.len() > Self::MAX_REQUEST_ITEMS {
			Err(format!(
				"Cannot process more than {} workflows. Please use multiple requests.",
				Self::MAX_REQUEST_ITEMS
			))
		} else {
			Ok(())
		}
	}
}
