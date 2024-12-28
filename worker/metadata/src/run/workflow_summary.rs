use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::WorkflowStatus;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSummary {
	pub workflow_type: String,
	pub version: i32,
	pub workflow_id: String,
	pub correlation_id: Option<String>,
	pub start_time: Option<DateTime<Utc>>,
	pub update_time: Option<DateTime<Utc>>,
	pub end_time: Option<DateTime<Utc>>,
	pub status: WorkflowStatus,
	pub input: Option<String>,
	pub output: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub execution_time: Option<i64>,
	pub event: Option<String>,
	pub failed_reference_task_names: String,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub priority: i32,
	pub failed_task_names: HashSet<String>,
	pub created_by: Option<String>,
}

// impl WorkflowSummary {
// 	pub fn new(workflow: &Workflow) -> Self {
// 		let formatter = "%Y-%m-%dT%H:%M:%S.%fZ";
// 		let format = |timestamp: Option<i64>| -> Option<DateTime<Utc>> {
// 			timestamp.map(|t| {
// 				let datetime =
// 					chrono::NaiveDateTime::from_timestamp(t / 1000, (t % 1000 * 1_000_000) as u32);
// 				DateTime::<Utc>::from_utc(datetime, Utc)
// 			})
// 		};

// 		let input = workflow.input().map(SummaryUtil::serialize_input_output);
// 		let output = workflow.output().map(SummaryUtil::serialize_input_output);

// 		let failed_reference_task_names = workflow.failed_reference_task_names().join(",");
// 		let failed_task_names = workflow.failed_task_names().iter().cloned().collect();

// 		Self {
// 			workflow_type: workflow.workflow_name().to_string(),
// 			version: workflow.workflow_version(),
// 			workflow_id: workflow.workflow_id().to_string(),
// 			correlation_id: workflow.correlation_id().map(String::from),
// 			start_time: format(workflow.create_time()),
// 			update_time: format(workflow.update_time()),
// 			end_time: format(workflow.end_time()),
// 			status: workflow.status(),
// 			input,
// 			output,
// 			reason_for_incompletion: workflow.reason_for_incompletion().map(String::from),
// 			execution_time: workflow.end_time().map(|end| end - workflow.start_time().unwrap_or(0)),
// 			event: workflow.event().map(String::from),
// 			failed_reference_task_names,
// 			external_input_payload_storage_path: workflow
// 				.external_input_payload_storage_path()
// 				.map(String::from),
// 			external_output_payload_storage_path: workflow
// 				.external_output_payload_storage_path()
// 				.map(String::from),
// 			priority: workflow.priority(),
// 			failed_task_names,
// 			created_by: workflow.created_by().map(String::from),
// 		}
// 	}
// }
