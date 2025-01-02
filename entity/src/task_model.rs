//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::TaskStatus;
use super::sea_orm_active_enums::TaskType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
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
	pub scheduled_time: DateTimeWithTimeZone,
	pub start_time: DateTimeWithTimeZone,
	pub end_time: Option<DateTimeWithTimeZone>,
	pub update_time: Option<DateTimeWithTimeZone>,
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
	pub input_message: Option<Json>,
	pub output_message: Option<Json>,
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
	pub task_config_id: Option<Uuid>,
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::buissness_rule::Entity")]
	BuissnessRule,
	#[sea_orm(has_many = "super::do_while::Entity")]
	DoWhile,
	#[sea_orm(has_many = "super::dynamic::Entity")]
	Dynamic,
	#[sea_orm(has_many = "super::dynamic_fork::Entity")]
	DynamicFork,
	#[sea_orm(has_many = "super::event::Entity")]
	Event,
	#[sea_orm(has_many = "super::fork::Entity")]
	Fork,
	#[sea_orm(has_many = "super::get_signed_jwt::Entity")]
	GetSignedJwt,
	#[sea_orm(has_many = "super::get_workflow::Entity")]
	GetWorkflow,
	#[sea_orm(has_many = "super::http::Entity")]
	Http,
	#[sea_orm(has_many = "super::human::Entity")]
	Human,
	#[sea_orm(has_many = "super::inline::Entity")]
	Inline,
	#[sea_orm(has_many = "super::join::Entity")]
	Join,
	#[sea_orm(has_many = "super::json_transform::Entity")]
	JsonTransform,
	#[sea_orm(has_many = "super::set_variable::Entity")]
	SetVariable,
	#[sea_orm(has_many = "super::simple::Entity")]
	Simple,
	#[sea_orm(has_many = "super::sql::Entity")]
	Sql,
	#[sea_orm(has_many = "super::start_workflow::Entity")]
	StartWorkflow,
	#[sea_orm(has_many = "super::sub_workflow::Entity")]
	SubWorkflow,
	#[sea_orm(has_many = "super::switch::Entity")]
	Switch,
	#[sea_orm(
		belongs_to = "super::task_config::Entity",
		from = "Column::TaskConfigId",
		to = "super::task_config::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskConfig,
	#[sea_orm(
		belongs_to = "super::task_definition::Entity",
		from = "Column::TaskDefName",
		to = "super::task_definition::Column::Name",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskDefinition,
	#[sea_orm(has_many = "super::task_execution_log::Entity")]
	TaskExecutionLog,
	#[sea_orm(has_many = "super::terminate_task::Entity")]
	TerminateTask,
	#[sea_orm(has_many = "super::terminate_workflow::Entity")]
	TerminateWorkflow,
	#[sea_orm(has_many = "super::update_secret::Entity")]
	UpdateSecret,
	#[sea_orm(has_many = "super::update_task::Entity")]
	UpdateTask,
	#[sea_orm(has_many = "super::wait::Entity")]
	Wait,
	#[sea_orm(has_many = "super::wait_for_webhook::Entity")]
	WaitForWebhook,
}

impl Related<super::buissness_rule::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::BuissnessRule.def()
	}
}

impl Related<super::do_while::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DoWhile.def()
	}
}

impl Related<super::dynamic::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Dynamic.def()
	}
}

impl Related<super::dynamic_fork::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DynamicFork.def()
	}
}

impl Related<super::event::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Event.def()
	}
}

impl Related<super::fork::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Fork.def()
	}
}

impl Related<super::get_signed_jwt::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GetSignedJwt.def()
	}
}

impl Related<super::get_workflow::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GetWorkflow.def()
	}
}

impl Related<super::http::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Http.def()
	}
}

impl Related<super::human::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Human.def()
	}
}

impl Related<super::inline::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Inline.def()
	}
}

impl Related<super::join::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Join.def()
	}
}

impl Related<super::json_transform::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::JsonTransform.def()
	}
}

impl Related<super::set_variable::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SetVariable.def()
	}
}

impl Related<super::simple::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Simple.def()
	}
}

impl Related<super::sql::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Sql.def()
	}
}

impl Related<super::start_workflow::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::StartWorkflow.def()
	}
}

impl Related<super::sub_workflow::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SubWorkflow.def()
	}
}

impl Related<super::switch::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Switch.def()
	}
}

impl Related<super::task_config::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskConfig.def()
	}
}

impl Related<super::task_definition::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskDefinition.def()
	}
}

impl Related<super::task_execution_log::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskExecutionLog.def()
	}
}

impl Related<super::terminate_task::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TerminateTask.def()
	}
}

impl Related<super::terminate_workflow::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TerminateWorkflow.def()
	}
}

impl Related<super::update_secret::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::UpdateSecret.def()
	}
}

impl Related<super::update_task::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::UpdateTask.def()
	}
}

impl Related<super::wait::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Wait.def()
	}
}

impl Related<super::wait_for_webhook::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::WaitForWebhook.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}