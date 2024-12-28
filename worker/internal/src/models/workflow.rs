use crate::{constants::DECIDER_QUEUE, Context, ExecutionDao, QueueDao};

use super::TaskModel;
use chrono::{DateTime, Utc};
use metadata::{Error, Result, StartWorkflowInput, TaskType, WorkflowDef, WorkflowStatus};
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, HashSet, LinkedList, VecDeque},
	sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowModel {
	pub status: WorkflowStatus,
	pub end_time: DateTime<Utc>,
	pub workflow_id: String,
	pub parent_workflow_id: Option<String>,
	pub parent_workflow_task_id: Option<String>,
	pub tasks: VecDeque<Arc<TaskModel>>,
	pub correlation_id: Option<String>,
	pub re_run_from_workflow_id: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub event: Option<String>,
	pub task_to_domain: Option<HashMap<String, String>>,
	pub failed_reference_task_names: HashSet<String>,
	pub failed_task_names: HashSet<String>,
	pub workflow_definition: Option<Arc<WorkflowDef>>,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub priority: u8,
	pub variables: Option<HashMap<String, serde_json::Value>>,
	pub last_retried_time: DateTime<Utc>,
	pub owner_app: Option<String>,
	pub create_time: DateTime<Utc>,
	pub updated_time: Option<DateTime<Utc>>,
	pub created_by: Option<String>,
	pub updated_by: Option<String>,
	pub failed_task_id: Option<String>,
	pub previous_status: Option<WorkflowStatus>,
	#[serde(skip)]
	pub input: HashMap<String, serde_json::Value>,
	#[serde(skip)]
	pub output: HashMap<String, serde_json::Value>,
	#[serde(skip)]
	pub input_payload: HashMap<String, serde_json::Value>,
	#[serde(skip)]
	pub output_payload: HashMap<String, serde_json::Value>,
}

impl WorkflowModel {
	pub fn new() -> Self {
		Self {
			status: WorkflowStatus::Running,
			end_time: Utc::now(),
			workflow_id: String::new(),
			parent_workflow_id: None,
			parent_workflow_task_id: None,
			tasks: VecDeque::new(),
			correlation_id: None,
			re_run_from_workflow_id: None,
			reason_for_incompletion: None,
			event: None,
			task_to_domain: None,
			failed_reference_task_names: HashSet::new(),
			failed_task_names: HashSet::new(),
			workflow_definition: None,
			external_input_payload_storage_path: None,
			external_output_payload_storage_path: None,
			priority: 0,
			variables: None,
			last_retried_time: Utc::now(),
			owner_app: None,
			create_time: Utc::now(),
			updated_time: None,
			created_by: None,
			updated_by: None,
			failed_task_id: None,
			previous_status: None,
			input: HashMap::new(),
			output: HashMap::new(),
			input_payload: HashMap::new(),
			output_payload: HashMap::new(),
		}
	}

	pub fn get_workflow_definition(&self) -> Result<Arc<WorkflowDef>> {
		self.workflow_definition
			.clone()
			.ok_or(Error::illegal_argument("Missing workflow definition"))
	}

	pub fn decide(&self) {}

	pub fn start(&mut self, input: Box<StartWorkflowInput>) -> Result<()> {
		let workflow_definition = match input.workflow_definition {
			Some(workflow_definition) => workflow_definition,
			None => {
				return Err(Error::illegal_argument("Missing workflow definition"));
			}
		};

		let workflow_id = match input.workflow_id {
			Some(workflow_id) => workflow_id,
			None => {
				//TODO: generate workflow id
				"test".to_string()
			}
		};

		self.workflow_id = workflow_id;
		self.correlation_id = input.correlation_id;
		self.priority = match input.priority {
			Some(priority) => priority,
			None => 0,
		};
		self.status = WorkflowStatus::Running;
		self.parent_workflow_id = input.parent_workflow_id;
		self.parent_workflow_task_id = input.parent_workflow_task_id;
		//TODO: set owner app to service name
		self.owner_app = Some("test".to_string());
		self.create_time = Utc::now();
		self.updated_by = None;
		self.updated_time = None;
		self.event = input.event;
		self.task_to_domain = input.task_to_domain;
		self.variables = workflow_definition.variables.clone();
		self.workflow_definition = Some(workflow_definition);

		Ok(())
	}

	pub fn rerun(
		&mut self,
		context: &Context,
		task_id: &Option<String>,
		task_input: &Option<HashMap<String, serde_json::Value>>,
		correlation_id: Option<String>,
		workflow_input: Option<HashMap<String, serde_json::Value>>,
	) -> Result<bool> {
		if self.status.is_terminal() {
			return Err(Error::Conflict(format!("Workflow is in terminal state: {}", self.status)));
		};

		// Set workflow as Running
		self.status = WorkflowStatus::Running;
		// Reset failure reason from previous run to default
		self.reason_for_incompletion = None;
		self.failed_task_id = None;
		self.failed_reference_task_names.clear();
		self.failed_task_names.clear();

		if let Some(corr_id) = correlation_id {
			self.correlation_id = Some(corr_id);
		}

		if let Some(input) = workflow_input {
			self.input = input;
		}

		//TODO: use config file instead
		let offset_time = 30;
		context.queue.push_with_priority(
			&DECIDER_QUEUE,
			&self.workflow_id,
			self.priority,
			offset_time,
		);

		if let Some(id) = task_id {
			let mut task_model = self.tasks.iter().find(|task| task.task_id == *id);

			if task_model.is_none() {
				for model in
					self.tasks.iter().filter(|task| task.task_type == TaskType::SubWorkflow)
				{
					if let Some(workflow_id) = &model.sub_workflow_id {
						if let Some(mut workflow_model) =
							context.execution.get_workflow(workflow_id)
						{
							if workflow_model.rerun(context, task_id, task_input, None, None)? {
								task_model = Some(model);
								break;
							}
						}
					}
				}
			}

			if let Some(task_model) = task_model {
				let mut filtered_tasks = VecDeque::new();
				self.tasks.iter().for_each(|task| {
					if task.seq > task_model.seq {
						context.execution.remove_task(&task.task_id);
					} else {
						filtered_tasks.push_back(task.clone());
					}
				});

				self.tasks = filtered_tasks;

				task_model.rerun(task_input);

				self.decide();

				return Ok(true);
			}
		} else {
			self.tasks.iter().for_each(|task| {
				context.execution.remove_task(&task.task_id);
			});
			self.tasks.clear();

			self.decide();

			return Ok(true);
		}

		Ok(false)
	}
}
