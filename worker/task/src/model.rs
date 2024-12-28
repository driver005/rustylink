use crate::Context;
use chrono::{DateTime, Utc};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task_model")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub task_id: Uuid,
	pub task_type: TaskType,
	pub status: TaskStatus,
	pub reference_task_name: String,
	pub retry_count: Option<i32>,
	pub seq: i32,
	pub correlation_id: Option<String>,
	pub poll_count: i32,
	pub task_def_name: String,
	pub scheduled_time: DateTime<Utc>,
	pub start_time: DateTime<Utc>,
	pub end_time: Option<DateTime<Utc>>,
	pub update_time: Option<DateTime<Utc>>,
	pub start_delay_in_seconds: i64,
	pub retried_task_id: Option<String>,
	pub retried: bool,
	pub executed: bool,
	pub callback_from_worker: bool,
	pub response_timeout_seconds: Option<i64>,
	pub workflow_instance_id: Option<String>,
	pub workflow_type: Option<String>,
	pub reason_for_incompletion: Option<String>,
	pub callback_after_seconds: i64,
	pub worker_id: Option<String>,
	pub domain: Option<String>,
	pub input_message: Option<serde_json::Value>,
	pub output_message: Option<serde_json::Value>,
	pub rate_limit_per_frequency: Option<i32>,
	pub rate_limit_frequency_in_seconds: Option<i32>,
	pub external_input_payload_storage_path: Option<String>,
	pub external_output_payload_storage_path: Option<String>,
	pub workflow_priority: i32,
	pub execution_name_space: Option<String>,
	pub isolation_group_id: Option<String>,
	pub iteration: i32,
	pub subworkflow_changed: bool,
	pub wait_timeout: Option<i64>,
	pub workflow_task_id: Option<Uuid>,
	pub buissness_rule_id: Option<Uuid>,
	pub do_while_id: Option<Uuid>,
	pub dynamic_id: Option<Uuid>,
	pub dynamic_fork_id: Option<Uuid>,
	pub event_id: Option<Uuid>,
	pub fork_id: Option<Uuid>,
	pub get_signed_jwt_id: Option<Uuid>,
	pub http_id: Option<Uuid>,
	pub inline_id: Option<Uuid>,
	pub join_id: Option<Uuid>,
	pub json_transform_id: Option<Uuid>,
	pub set_variable_id: Option<Uuid>,
	pub simple_id: Option<Uuid>,
	pub sql_task_id: Option<Uuid>,
	pub get_workflow_id: Option<Uuid>,
	pub start_workflow_id: Option<Uuid>,
	pub sub_workflow_id: Option<Uuid>,
	pub switch_id: Option<Uuid>,
	pub task_update_id: Option<Uuid>,
	pub terminate_task_id: Option<Uuid>,
	pub terminate_workflow_id: Option<Uuid>,
	pub human_id: Option<Uuid>,
	pub update_secret_id: Option<Uuid>,
	pub wait_id: Option<Uuid>,
	pub wait_for_webhook_id: Option<Uuid>,
	// pub workflow_task: Arc<TaskConfig>,
	// pub buissness_rule: Option<BuissnessRule>,
	// pub do_while: Option<DoWhile>,
	// pub dynamic: Option<Dynamic>,
	// pub dynamic_fork: Option<DynamicFork>,
	// pub event: Option<Event>,
	// pub fork: Option<Fork>,
	// pub get_signed_jwt: Option<GetSignedJwt>,
	// pub http: Option<Http>,
	// pub inline: Option<Inline>,
	// pub join: Option<Join>,
	// pub json_transform: Option<JsonTransform>,
	// pub set_variable: Option<SetVariable>,
	// pub simple: Option<Simple>,
	// pub sql_task: Option<SqlTask>,
	// pub start_workflow: Option<StartWorkflow>,
	// pub sub_workflow: Option<SubWorkflow>,
	// pub switch: Option<Switch>,
	// pub task_update: Option<UpdateTask>,
	// pub terminate_task: Option<TerminateTask>,
	// pub terminate_workflow: Option<TerminateWorkflow>,
	// pub update_secret: Option<UpdateSecret>,
	// pub wait: Option<Wait>,
	// pub wait_for_webhook: Option<WaitForWebhook>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "crate::definition::Entity",
		from = "Column::TaskDefName",
		to = "crate::definition::Column::Name",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskDefinition,
	#[sea_orm(
		belongs_to = "crate::config::Entity",
		from = "Column::WorkflowTaskId",
		to = "crate::config::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	WorkflowTask,
	#[sea_orm(
		belongs_to = "crate::system::buissness::Entity",
		from = "Column::BuissnessRuleId",
		to = "crate::system::buissness::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	BuissnessRule,
	#[sea_orm(
		belongs_to = "crate::operation::do_while::Entity",
		from = "Column::DoWhileId",
		to = "crate::operation::do_while::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	DoWhile,
	#[sea_orm(
		belongs_to = "crate::operation::dynamic::Entity",
		from = "Column::DynamicId",
		to = "crate::operation::dynamic::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Dynamic,
	#[sea_orm(
		belongs_to = "crate::operation::fork::dynamic::Entity",
		from = "Column::DynamicForkId",
		to = "crate::operation::fork::dynamic::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	DynamicFork,
	#[sea_orm(
		belongs_to = "crate::system::event::Entity",
		from = "Column::EventId",
		to = "crate::system::event::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Event,
	#[sea_orm(
		belongs_to = "crate::operation::fork::fork::Entity",
		from = "Column::ForkId",
		to = "crate::operation::fork::fork::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Fork,
	#[sea_orm(
		belongs_to = "crate::system::jwt::Entity",
		from = "Column::GetSignedJwtId",
		to = "crate::system::jwt::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	GetSignedJwt,
	#[sea_orm(
		belongs_to = "crate::system::http::Entity",
		from = "Column::HttpId",
		to = "crate::system::http::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Http,
	#[sea_orm(
		belongs_to = "crate::system::inline::Entity",
		from = "Column::InlineId",
		to = "crate::system::inline::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Inline,
	#[sea_orm(
		belongs_to = "crate::operation::join::Entity",
		from = "Column::JoinId",
		to = "crate::operation::join::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Join,
	#[sea_orm(
		belongs_to = "crate::system::transform::json::Entity",
		from = "Column::JsonTransformId",
		to = "crate::system::transform::json::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	JsonTransform,
	#[sea_orm(
		belongs_to = "crate::operation::variable::Entity",
		from = "Column::SetVariableId",
		to = "crate::operation::variable::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	SetVariable,
	#[sea_orm(
		belongs_to = "crate::operation::simple::Entity",
		from = "Column::SimpleId",
		to = "crate::operation::simple::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Simple,
	#[sea_orm(
		belongs_to = "crate::system::sql::Entity",
		from = "Column::SqlTaskId",
		to = "crate::system::sql::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	SqlTask,
	#[sea_orm(
		belongs_to = "crate::operation::workflow::get::Entity",
		from = "Column::GetWorkflowId",
		to = "crate::operation::workflow::get::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	GetWorkflow,
	#[sea_orm(
		belongs_to = "crate::operation::start::Entity",
		from = "Column::StartWorkflowId",
		to = "crate::operation::start::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	StartWorkflow,
	#[sea_orm(
		belongs_to = "crate::operation::sub::Entity",
		from = "Column::SubWorkflowId",
		to = "crate::operation::sub::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	SubWorkflow,
	#[sea_orm(
		belongs_to = "crate::operation::switch::Entity",
		from = "Column::SwitchId",
		to = "crate::operation::switch::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Switch,
	#[sea_orm(
		belongs_to = "crate::operation::task::update::Entity",
		from = "Column::TaskUpdateId",
		to = "crate::operation::task::update::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskUpdate,
	#[sea_orm(
		belongs_to = "crate::operation::task::terminate::Entity",
		from = "Column::TerminateTaskId",
		to = "crate::operation::task::terminate::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TerminateTask,
	#[sea_orm(
		belongs_to = "crate::operation::workflow::terminate::Entity",
		from = "Column::TerminateWorkflowId",
		to = "crate::operation::workflow::terminate::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TerminateWorkflow,
	#[sea_orm(
		belongs_to = "crate::operation::human::Entity",
		from = "Column::HumanId",
		to = "crate::operation::human::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Human,
	#[sea_orm(
		belongs_to = "crate::system::secret::Entity",
		from = "Column::UpdateSecretId",
		to = "crate::system::secret::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	UpdateSecret,
	#[sea_orm(
		belongs_to = "crate::operation::wait::Entity",
		from = "Column::WaitId",
		to = "crate::operation::wait::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Wait,
	#[sea_orm(
		belongs_to = "crate::system::webhook::wait::Entity",
		from = "Column::WaitForWebhookId",
		to = "crate::system::webhook::wait::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	WaitForWebhook,
}

impl Related<crate::definition::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskDefinition.def()
	}
}
impl Related<crate::system::buissness::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::BuissnessRule.def()
	}
}
impl Related<crate::operation::do_while::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DoWhile.def()
	}
}
impl Related<crate::operation::dynamic::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Dynamic.def()
	}
}
impl Related<crate::operation::fork::dynamic::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DynamicFork.def()
	}
}
impl Related<crate::system::event::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Event.def()
	}
}
impl Related<crate::operation::fork::fork::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Fork.def()
	}
}
impl Related<crate::system::jwt::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GetSignedJwt.def()
	}
}
impl Related<crate::system::http::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Http.def()
	}
}
impl Related<crate::system::inline::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Inline.def()
	}
}
impl Related<crate::operation::join::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Join.def()
	}
}
impl Related<crate::system::transform::json::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::JsonTransform.def()
	}
}
impl Related<crate::operation::variable::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SetVariable.def()
	}
}
impl Related<crate::operation::simple::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Simple.def()
	}
}
impl Related<crate::system::sql::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SqlTask.def()
	}
}
impl Related<crate::operation::workflow::get::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GetWorkflow.def()
	}
}
impl Related<crate::operation::start::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::StartWorkflow.def()
	}
}
impl Related<crate::operation::sub::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SubWorkflow.def()
	}
}
impl Related<crate::operation::switch::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Switch.def()
	}
}
impl Related<crate::operation::task::update::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskUpdate.def()
	}
}
impl Related<crate::operation::task::terminate::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TerminateTask.def()
	}
}
impl Related<crate::operation::workflow::terminate::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TerminateWorkflow.def()
	}
}
impl Related<crate::system::secret::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::UpdateSecret.def()
	}
}
impl Related<crate::operation::human::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Human.def()
	}
}
impl Related<crate::operation::wait::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Wait.def()
	}
}
impl Related<crate::system::webhook::wait::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::WaitForWebhook.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
	pub fn new(
		task_type: TaskType,
		task_def_name: &str,
		reference_task_name: &str,
		scheduled_time: DateTime<Utc>,
	) -> Self {
		Self {
			task_id: Uuid::new_v4(),
			task_type,
			status: TaskStatus::Scheduled,
			reference_task_name: reference_task_name.to_string(),
			retry_count: None,
			seq: 0,
			correlation_id: None,
			poll_count: 0,
			task_def_name: task_def_name.to_string(),
			scheduled_time,
			start_time: Utc::now(),
			end_time: None,
			update_time: None,
			start_delay_in_seconds: 0,
			retried_task_id: None,
			retried: false,
			executed: false,
			callback_from_worker: true,
			response_timeout_seconds: None,
			workflow_instance_id: None,
			workflow_type: None,
			reason_for_incompletion: None,
			callback_after_seconds: 0,
			worker_id: None,
			domain: None,
			input_message: None,
			output_message: None,
			rate_limit_per_frequency: None,
			rate_limit_frequency_in_seconds: None,
			external_input_payload_storage_path: None,
			external_output_payload_storage_path: None,
			workflow_priority: 0,
			execution_name_space: None,
			isolation_group_id: None,
			iteration: 0,
			sub_workflow_id: None,
			subworkflow_changed: false,
			wait_timeout: None,
			workflow_task_id: None,
			buissness_rule_id: None,
			do_while_id: None,
			dynamic_id: None,
			dynamic_fork_id: None,
			event_id: None,
			fork_id: None,
			get_signed_jwt_id: None,
			http_id: None,
			inline_id: None,
			join_id: None,
			json_transform_id: None,
			set_variable_id: None,
			simple_id: None,
			sql_task_id: None,
			get_workflow_id: None,
			start_workflow_id: None,
			switch_id: None,
			task_update_id: None,
			terminate_task_id: None,
			terminate_workflow_id: None,
			human_id: None,
			update_secret_id: None,
			wait_id: None,
			wait_for_webhook_id: None,
		}
	}

	pub async fn save(self, context: &mut Context) -> Result<()> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}
}

pub type TaskModel = Model;
