use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{RetryLogic, SchemaDef, TimeoutPolicy};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDef {
	pub name: String,
	pub description: Option<String>,
	pub retry_count: i32,
	pub timeout_seconds: Option<i64>,
	pub input_keys: Option<Vec<String>>,
	pub output_keys: Option<Vec<String>>,
	pub timeout_policy: TimeoutPolicy,
	pub retry_logic: RetryLogic,
	pub retry_delay_seconds: i32,
	pub response_timeout_seconds: i64,
	pub concurrent_exec_limit: Option<i32>,
	pub input_template: Option<serde_json::Value>,
	pub rate_limit_per_frequency: Option<u32>,
	pub rate_limit_frequency_in_seconds: Option<u32>,
	pub isolation_group_id: Option<String>,
	pub execution_name_space: Option<String>,
	pub owner_email: Option<String>,
	pub poll_timeout_seconds: Option<i32>,
	pub backoff_scale_factor: i32,
	pub base_type: Option<String>,
	pub input_schema: Option<SchemaDef>,
	pub enforce_schema: bool,
	pub output_schema: Option<SchemaDef>,
	pub created_on: DateTime<Utc>,
	pub created_by: Option<String>,
	pub modified_on: DateTime<Utc>,
	pub modified_by: Option<String>,
}

impl TaskDef {
	pub const ONE_HOUR: i64 = 60 * 60;

	// Constructor
	pub fn new(name: String) -> Self {
		TaskDef {
			name,
			description: None,
			retry_count: 3,
			timeout_seconds: None,
			input_keys: None,
			output_keys: None,
			timeout_policy: TimeoutPolicy::TimeOutWf,
			retry_logic: RetryLogic::Fixed,
			retry_delay_seconds: 60,
			response_timeout_seconds: Self::ONE_HOUR,
			concurrent_exec_limit: None,
			input_template: None,
			rate_limit_per_frequency: None,
			rate_limit_frequency_in_seconds: None,
			isolation_group_id: None,
			execution_name_space: None,
			owner_email: None,
			poll_timeout_seconds: None,
			backoff_scale_factor: 1,
			base_type: None,
			input_schema: None,
			output_schema: None,
			enforce_schema: false,
			created_on: Utc::now(),
			created_by: None,
			modified_on: Utc::now(),
			modified_by: None,
		}
	}
}
