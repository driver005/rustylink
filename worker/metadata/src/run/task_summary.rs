use crate::TaskStatus;
use chrono::{DateTime, Utc};
use std::option::Option;

#[derive(Debug, Clone)]
pub struct TaskSummary {
	pub workflow_id: String,
	pub workflow_type: String,
	pub correlation_id: Option<String>,
	pub scheduled_time: DateTime<Utc>,
	pub start_time: Option<DateTime<Utc>>,
	pub update_time: Option<DateTime<Utc>>,
	pub end_time: Option<DateTime<Utc>>,
	pub status: TaskStatus,
	pub reason_for_incompletion: Option<String>,
	pub execution_time: Option<i64>,
	pub queue_wait_time: i64,
	pub task_def_name: String,
	pub task_type: String,
	pub input: Option<String>,
	pub output: Option<String>,
	pub task_id: String,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub workflow_priority: i32,
	pub domain: Option<String>,
}

impl TaskSummary {
	// pub fn new(task: &Task) -> Self {
	// 	let scheduled_time = DateTime::<Utc>::from_utc(
	// 		chrono::NaiveDateTime::from_timestamp(task.scheduled_time / 1000, 0),
	// 		Utc,
	// 	);
	// 	let start_time = task.start_time.map(|ts| {
	// 		DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(ts / 1000, 0), Utc)
	// 	});
	// 	let update_time = task.update_time.map(|ts| {
	// 		DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(ts / 1000, 0), Utc)
	// 	});
	// 	let end_time = task.end_time.map(|ts| {
	// 		DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(ts / 1000, 0), Utc)
	// 	});

	// 	let execution_time = end_time
	// 		.and_then(|et| start_time.map(|st| (et.timestamp_millis() - st.timestamp_millis())));

	// 	TaskSummary {
	// 		workflow_id: task.workflow_instance_id,
	// 		workflow_type: task.workflow_type,
	// 		correlation_id: task.correlation_id,
	// 		scheduled_time,
	// 		start_time,
	// 		update_time,
	// 		end_time,
	// 		status: task.status,
	// 		reason_for_incompletion: task.reason_for_incompletion,
	// 		execution_time,
	// 		queue_wait_time: task.queue_wait_time,
	// 		task_def_name: task.task_def_name,
	// 		task_type: task.task_type,
	// 		input: task.input_data.as_ref().map(|input| SummaryUtil::serialize_input_output(input)),
	// 		output: task
	// 			.output_data
	// 			.as_ref()
	// 			.map(|output| SummaryUtil::serialize_input_output(output)),
	// 		task_id: task.task_id,
	// 		external_input_payload_storage_path: task.external_input_payload_storage_path,
	// 		external_output_payload_storage_path: task.external_output_payload_storage_path,
	// 		workflow_priority: task.workflow_priority,
	// 		domain: task.domain,
	// 	}
	// }
}

// The Task struct and SummaryUtil would be defined similarly, with appropriate methods and fields.
