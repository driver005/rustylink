type Any = serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskModel {
	pub task_id: String,
	pub task_type: TaskType,
	pub status: TaskStatus,
	pub reference_task_name: String,
	pub retry_count: Option<u32>,
	pub seq: i32,
	pub correlation_id: Option<String>,
	pub poll_count: i32,
	pub task_def_name: String,
	pub scheduled_time: DateTime<Utc>,
	pub start_time: DateTime<Utc>,
	pub end_time: Option<DateTime<Utc>>,
	pub update_time: Option<DateTime<Utc>>,
	pub start_delay_in_seconds: u64,
	pub retried_task_id: Option<String>,
	pub retried: bool,
	pub executed: bool,
	pub callback_from_worker: bool,
	pub response_timeout_seconds: Option<u64>,
	pub workflow_instance_id: Option<String>,
	pub workflow_type: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub callback_after_seconds: u64,
	pub worker_id: Option<String>,
	pub workflow_task: Arc<TaskConfig>,
	pub domain: Option<String>,
	pub input_message: Option<Any>,
	pub output_message: Option<Any>,
	pub rate_limit_per_frequency: Option<u32>,
	pub rate_limit_frequency_in_seconds: Option<u32>,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub workflow_priority: u8,
	pub execution_name_space: Option<String>,
	pub isolation_group_id: Option<String>,
	pub iteration: i32,
	pub sub_workflow_id: Option<String>,
	pub subworkflow_changed: bool,
	pub wait_timeout: Option<i64>,

	pub buissness_rule: Option<BuissnessRule>,
	pub do_while: Option<DoWhile>,
	pub dynamic: Option<Dynamic>,
	pub dynamic_fork: Option<DynamicFork>,
	pub event: Option<Event>,
	pub fork: Option<Fork>,
	pub get_signed_jwt: Option<GetSignedJwt>,
	pub http: Option<Http>,
	pub inline: Option<Inline>,
	pub join: Option<Join>,
	pub json_transform: Option<JsonTransform>,
	pub set_variable: Option<SetVariable>,
	pub simple: Option<Simple>,
	pub sql_task: Option<SqlTask>,
	pub start_workflow: Option<StartWorkflow>,
	pub sub_workflow: Option<SubWorkflow>,
	pub switch: Option<Switch>,
	pub task_update: Option<TaskUpdate>,
	pub terminate_task: Option<TerminateTask>,
	pub terminate_workflow: Option<TerminateWorkflow>,
	pub update_secret: Option<UpdateSecret>,
	pub wait: Option<Wait>,
	pub wait_for_webhook: Option<WaitForWebhook>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuissnessRule {
	pub task_configuration: Arc<TaskConfig>,
	pub rule_file_location: String,
	pub execution_strategy: String,
	pub input_column: HashMap<String, serde_json::Value>,
	pub output_column: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoWhile {
	pub task_configuration: Arc<TaskConfig>,
	pub evaluator_type: String,
	pub loop_condition: String,
	pub loop_over: Vec<TaskConfig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dynamic {
	pub task_configuration: Arc<TaskConfig>,
	pub dynamic_task_name_param: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DynamicFork {
	DifferentTask {
		dynamic_fork_tasks_param: String,
		dynamic_fork_tasks_input_param_name: String,
	},
	SameTask {
		fork_task_name: String,
		fork_task_inputs: HashMap<String, serde_json::Value>,
	},
	SameTaskSubWorkflow {
		fork_task_workflow: String,
		fork_task_workflow_version: String,
		fork_task_inputs: HashMap<String, serde_json::Value>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
	pub task_configuration: Arc<TaskConfig>,
	pub sink: String,
	pub async_complete: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fork {
	pub task_configuration: Arc<TaskConfig>,
	pub fork_tasks: Vec<Vec<TaskConfig>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetSignedJwt {
	pub task_configuration: Arc<TaskConfig>,
	pub subject: String,
	pub issuer: String,
	pub private_key: String,
	pub private_key_id: String,
	pub audience: String,
	pub ttl_in_seconds: u64,
	pub scopes: String,
	pub algorithm: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Http {
	pub task_configuration: Arc<TaskConfig>,
	pub uri: String,
	pub method: String,
	pub accept: Option<String>,
	pub content_type: Option<String>,
	pub termination_condition: Option<String>,
	pub polling_interval: Option<u64>,
	pub polling_strategy: Option<String>,
	pub headers: Option<HashMap<String, serde_json::Value>>,
	pub body: Option<HashMap<String, serde_json::Value>>,
	pub encode: Option<bool>,
	pub async_complete: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inline {
	pub task_configuration: Arc<TaskConfig>,
	pub evaluator_type: String,
	pub expression: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Join {
	pub task_configuration: Arc<TaskConfig>,
	pub join_on: Vec<String>,
	pub expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonTransform {
	pub task_configuration: Arc<TaskConfig>,
	pub query_expression: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetVariable {
	pub task_configuration: Arc<TaskConfig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Simple {
	pub task_configuration: Arc<TaskConfig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SqlTask {
	pub task_configuration: Arc<TaskConfig>,
	pub integration_name: String,
	pub statement: String,
	pub operation_type: String,
	pub parameters: Vec<String>,
	pub expected_output_count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartWorkflow {
	pub task_configuration: Arc<TaskConfig>,
	pub name: String,
	pub version: Option<u64>,
	pub correlation_id: Option<String>,
	pub idempotency_key: Option<String>,
	pub idempotency_strategy: Option<IdempotencyStrategy>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubWorkflow {
	pub task_configuration: Arc<TaskConfig>,
	pub name: String,
	pub version: u32,
	pub task_to_domain: Option<HashMap<String, String>>,
	pub idempotency_key: Option<String>,
	pub idempotency_strategy: Option<IdempotencyStrategy>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
	pub task_configuration: Arc<TaskConfig>,
	pub evaluator_type: String,
	pub expression: String,
	pub decision_cases: HashMap<String, Vec<TaskConfig>>,
	pub default_case: Option<Vec<TaskConfig>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskUpdate {
	pub task_configuration: Arc<TaskConfig>,
	pub task_status: TaskStatus,
	pub workflow_id: Option<String>,
	pub task_ref_name: Option<String>,
	pub task_id: Option<String>,
	pub merge_output: Option<bool>,
	pub task_output: Option<HashMap<String, serde_json::Value>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TerminateTask {
	pub task_configuration: Arc<TaskConfig>,
	pub termination_status: TaskTerminationStatus,
	pub termination_reason: Option<String>,
	pub workflow_output: Option<HashMap<String, serde_json::Value>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TerminateWorkflow {
	pub task_configuration: Arc<TaskConfig>,
	pub workflow_id: Vec<String>,
	pub trigger_failure_workflow: bool,
	pub termination_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSecret {
	pub task_configuration: Arc<TaskConfig>,
	pub secret_key: String,
	pub secret_value: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Wait {
	pub task_configuration: Arc<TaskConfig>,
	pub until: Option<DateTime<Utc>>,
	pub duration: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WaitForWebhook {
	pub task_configuration: Arc<TaskConfig>,
	pub matches: HashMap<String, serde_json::Value>,
}
