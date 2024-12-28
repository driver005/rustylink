use once_cell::sync::Lazy;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum_macros::{Display, EnumString};

pub static BUILT_IN_TASKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
	let mut set = HashSet::new();
	set.insert(TaskType::TASK_TYPE_DECISION);
	set.insert(TaskType::TASK_TYPE_SWITCH);
	set.insert(TaskType::TASK_TYPE_FORK);
	set.insert(TaskType::TASK_TYPE_JOIN);
	set.insert(TaskType::TASK_TYPE_EXCLUSIVE_JOIN);
	set.insert(TaskType::TASK_TYPE_DO_WHILE);
	set
});

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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_type")]
pub enum TaskType {
	#[sea_orm(string_value = "SIMPLE")]
	Simple,
	#[sea_orm(string_value = "DYNAMIC")]
	Dynamic,
	#[sea_orm(string_value = "FORK_JOIN")]
	ForkJoin,
	#[sea_orm(string_value = "FORK_JOIN_DYNAMIC")]
	ForkJoinDynamic,
	#[sea_orm(string_value = "SWITCH")]
	Switch,
	#[sea_orm(string_value = "JOIN")]
	Join,
	#[sea_orm(string_value = "DO_WHILE")]
	DoWhile,
	#[sea_orm(string_value = "SUB_WORKFLOW")]
	SubWorkflow,
	#[sea_orm(string_value = "START_WORKFLOW")]
	StartWorkflow,
	#[sea_orm(string_value = "EVENT")]
	Event,
	#[sea_orm(string_value = "WAIT")]
	Wait,
	#[sea_orm(string_value = "HUMAN")]
	Human,
	#[sea_orm(string_value = "USER_DEFINED")]
	UserDefined,
	#[sea_orm(string_value = "HTTP")]
	Http,
	#[sea_orm(string_value = "INLINE")]
	Inline,
	#[sea_orm(string_value = "EXCLUSIVE_JOIN")]
	ExclusiveJoin,
	#[sea_orm(string_value = "TERMINATE_TASK")]
	TerminateTask,
	#[sea_orm(string_value = "TERMINATE_WORKFLOW")]
	TerminateWorkflow,
	#[sea_orm(string_value = "KAFKA_PUBLISH")]
	KafkaPublish,
	#[sea_orm(string_value = "JSON_JQ_TRANSFORM")]
	JsonJqTransform,
	#[sea_orm(string_value = "SET_VARIABLE")]
	SetVariable,
	#[sea_orm(string_value = "UPDATE_TASK")]
	UpdateTask,
	#[sea_orm(string_value = "WAIT_FOR_WEBHOOK")]
	WaitForWebhook,
	#[sea_orm(string_value = "BUISSNESS_RULE")]
	BuissnessRule,
	#[sea_orm(string_value = "GET_SIGNED_JWT")]
	GetSignedJwt,
	#[sea_orm(string_value = "UPDATE_SECRET")]
	UpdateSecret,
	#[sea_orm(string_value = "SQL_TASK")]
	SqlTask,
}

impl TaskType {
	pub const TASK_TYPE_DECISION: &'static str = "DECISION";
	pub const TASK_TYPE_SWITCH: &'static str = "SWITCH";
	pub const TASK_TYPE_DYNAMIC: &'static str = "DYNAMIC";
	pub const TASK_TYPE_JOIN: &'static str = "JOIN";
	pub const TASK_TYPE_DO_WHILE: &'static str = "DO_WHILE";
	pub const TASK_TYPE_FORK_JOIN_DYNAMIC: &'static str = "FORK_JOIN_DYNAMIC";
	pub const TASK_TYPE_EVENT: &'static str = "EVENT";
	pub const TASK_TYPE_WAIT: &'static str = "WAIT";
	pub const TASK_TYPE_HUMAN: &'static str = "HUMAN";
	pub const TASK_TYPE_SUB_WORKFLOW: &'static str = "SUB_WORKFLOW";
	pub const TASK_TYPE_START_WORKFLOW: &'static str = "START_WORKFLOW";
	pub const TASK_TYPE_FORK_JOIN: &'static str = "FORK_JOIN";
	pub const TASK_TYPE_SIMPLE: &'static str = "SIMPLE";
	pub const TASK_TYPE_HTTP: &'static str = "HTTP";
	pub const TASK_TYPE_LAMBDA: &'static str = "LAMBDA";
	pub const TASK_TYPE_INLINE: &'static str = "INLINE";
	pub const TASK_TYPE_EXCLUSIVE_JOIN: &'static str = "EXCLUSIVE_JOIN";
	pub const TASK_TYPE_TERMINATE: &'static str = "TERMINATE";
	pub const TASK_TYPE_KAFKA_PUBLISH: &'static str = "KAFKA_PUBLISH";
	pub const TASK_TYPE_JSON_JQ_TRANSFORM: &'static str = "JSON_JQ_TRANSFORM";
	pub const TASK_TYPE_SET_VARIABLE: &'static str = "SET_VARIABLE";
	pub const TASK_TYPE_FORK: &'static str = "FORK";
	pub const TASK_TYPE_NOOP: &'static str = "NOOP";

	pub fn of(task_type: &str) -> Self {
		task_type.parse().unwrap_or(TaskType::UserDefined)
	}

	pub fn is_built_in(task_type: &str) -> bool {
		BUILT_IN_TASKS.contains(task_type)
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_status")]
pub enum TaskStatus {
	#[sea_orm(string_value = "IN_PROGRESS")]
	InProgress,
	#[sea_orm(string_value = "CANCELED")]
	Canceled,
	#[sea_orm(string_value = "FAILED")]
	Failed,
	#[sea_orm(string_value = "FAILED_WITH_TERMINAL_ERROR")]
	FailedWithTerminalError,
	#[sea_orm(string_value = "COMPLETED")]
	Completed,
	#[sea_orm(string_value = "COMPLETED_WITH_ERRORS")]
	CompletedWithErrors,
	#[sea_orm(string_value = "SCHEDULED")]
	Scheduled,
	#[sea_orm(string_value = "TIMED_OUT")]
	TimedOut,
	#[sea_orm(string_value = "SKIPPED")]
	Skipped,
}

impl TaskStatus {
	pub fn is_terminal(&self) -> bool {
		matches!(
			self,
			TaskStatus::Canceled
				| TaskStatus::Failed
				| TaskStatus::FailedWithTerminalError
				| TaskStatus::Completed
				| TaskStatus::CompletedWithErrors
				| TaskStatus::TimedOut
		)
	}

	pub fn is_successful(&self) -> bool {
		matches!(
			self,
			TaskStatus::Completed | TaskStatus::CompletedWithErrors | TaskStatus::Skipped
		)
	}

	pub fn is_retriable(&self) -> bool {
		matches!(self, TaskStatus::InProgress | TaskStatus::Failed | TaskStatus::TimedOut)
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_termination_status")]
pub enum TaskTerminationStatus {
	#[sea_orm(string_value = "COMPLETED")]
	Completed,
	#[sea_orm(string_value = "FAILED")]
	Failed,
	#[sea_orm(string_value = "TERMINATED")]
	Terminated,
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "fork_type")]
pub enum ForkType {
	#[sea_orm(string_value = "DIFFERENT_TASK")]
	DifferentTask,
	#[sea_orm(string_value = "SAME_TASK")]
	SameTask,
	#[sea_orm(string_value = "SAME_TASK_SUB_WORKFLOW")]
	SameTaskSubWorkflow,
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "timeout_policy")]
pub enum TimeoutPolicy {
	#[sea_orm(string_value = "RETRY")]
	Retry,
	#[sea_orm(string_value = "TIMED_OUT_WF")]
	TimeOutWf,
	#[sea_orm(string_value = "ALERT_ONLY")]
	AlertOnly,
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "retry_logic")]
pub enum RetryLogic {
	#[sea_orm(string_value = "FIXED")]
	Fixed,
	#[sea_orm(string_value = "EXPONENTIAL_BACKOFF")]
	ExponentialBackoff,
	#[sea_orm(string_value = "LINEAR_BACKOFF")]
	LinearBackoff,
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "operation_type")]
pub enum OperationType {
	#[sea_orm(string_value = "SELECT")]
	Select,
	#[sea_orm(string_value = "INSERT")]
	Insert,
	#[sea_orm(string_value = "Update")]
	Update,
	#[sea_orm(string_value = "DELETE")]
	Delete,
}
