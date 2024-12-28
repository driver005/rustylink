use log::debug;
use model::WorkflowModel;

/// Trait for listening to workflow status changes.
pub trait WorkflowStatusListener {
	/// Called when a workflow is completed, if the workflow status listener is enabled.
	///
	/// # Parameters
	///
	/// - `workflow`: A reference to a `WorkflowModel` representing the completed workflow.
	fn on_workflow_completed_if_enabled(&self, workflow: &WorkflowModel) {
		if let Some(workflow_definition) = &workflow.workflow_definition {
			if workflow_definition.workflow_status_listener_enabled {
				self.on_workflow_completed(workflow);
			}
		}
	}

	/// Called when a workflow is terminated, if the workflow status listener is enabled.
	///
	/// # Parameters
	///
	/// - `workflow`: A reference to a `WorkflowModel` representing the terminated workflow.
	fn on_workflow_terminated_if_enabled(&self, workflow: &WorkflowModel) {
		if let Some(workflow_definition) = &workflow.workflow_definition {
			if workflow_definition.workflow_status_listener_enabled {
				self.on_workflow_terminated(workflow);
			}
		}
	}

	/// Called when a workflow is finalized, if the workflow status listener is enabled.
	///
	/// # Parameters
	///
	/// - `workflow`: A reference to a `WorkflowModel` representing the finalized workflow.
	fn on_workflow_finalized_if_enabled(&self, workflow: &WorkflowModel) {
		if let Some(workflow_definition) = &workflow.workflow_definition {
			if workflow_definition.workflow_status_listener_enabled {
				self.on_workflow_finalized(workflow);
			}
		}
	}

	/// Called when a workflow is completed.
	///
	/// # Parameters
	///
	/// - `workflow`: A reference to a `WorkflowModel` representing the completed workflow.
	fn on_workflow_completed(&self, workflow: &WorkflowModel);

	/// Called when a workflow is terminated.
	///
	/// # Parameters
	///
	/// - `workflow`: A reference to a `WorkflowModel` representing the terminated workflow.
	fn on_workflow_terminated(&self, workflow: &WorkflowModel);

	/// Called when a workflow is finalized. Default implementation does nothing.
	///
	/// # Parameters
	///
	/// - `workflow`: A reference to a `WorkflowModel` representing the finalized workflow.
	fn on_workflow_finalized(&self, workflow: &WorkflowModel) {}
}

pub struct DefaultWorkflowStatusListener;

impl WorkflowStatusListener for DefaultWorkflowStatusListener {
	fn on_workflow_completed(&self, workflow: &WorkflowModel) {
		debug!("Workflow {} is completed", workflow.workflow_id);
	}

	fn on_workflow_terminated(&self, workflow: &WorkflowModel) {
		debug!("Workflow {} is terminated", workflow.workflow_id);
	}

	fn on_workflow_finalized(&self, workflow: &WorkflowModel) {
		debug!("Workflow {} is finalized", workflow.workflow_id);
	}
}
