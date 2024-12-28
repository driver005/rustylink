use crate::IdempotencyStrategy;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct SubWorkflowParams {
	pub name: String,
	pub version: u32,
	pub task_to_domain: Option<serde_json::Value>,
	pub idempotency_key: Option<String>,
	pub idempotency_strategy: Option<IdempotencyStrategy>,
}

impl SubWorkflowParams {}
