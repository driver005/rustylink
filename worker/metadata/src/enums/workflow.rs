use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
	Debug,
	Clone,
	Copy,
	Hash,
	PartialEq,
	Eq,
	EnumString,
	Display,
	Serialize,
	Deserialize,
	EnumIter,
	DeriveActiveEnum,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "workflow_status")]
pub enum WorkflowStatus {
	#[sea_orm(string_value = "RUNNING")]
	Running,
	#[sea_orm(string_value = "COMPLETED")]
	Completed,
	#[sea_orm(string_value = "FAILED")]
	Failed,
	#[sea_orm(string_value = "TIMED_OUT")]
	TimedOut,
	#[sea_orm(string_value = "TERMINATED")]
	Terminated,
	#[sea_orm(string_value = "PAUSED")]
	Paused,
}

impl WorkflowStatus {
	pub fn is_terminal(&self) -> bool {
		matches!(
			self,
			WorkflowStatus::Completed
				| WorkflowStatus::Failed
				| WorkflowStatus::TimedOut
				| WorkflowStatus::Terminated
		)
	}

	pub fn is_successful(&self) -> bool {
		matches!(self, WorkflowStatus::Completed)
	}
}

#[derive(
	Debug,
	Clone,
	Copy,
	Hash,
	PartialEq,
	Eq,
	EnumString,
	Display,
	Serialize,
	Deserialize,
	EnumIter,
	DeriveActiveEnum,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "idempotency_strategy")]
pub enum IdempotencyStrategy {
	#[sea_orm(string_value = "FAIL")]
	Fail,
	#[sea_orm(string_value = "RUNNING_EXISTING")]
	ReturnExisting,
}
