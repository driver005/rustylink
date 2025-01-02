use crate::{Context, TaskStorage};
use chrono::{DateTime, Utc};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
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
	pub task_config_id: Option<Uuid>,
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
	// pub buissness_rule_id: Option<Uuid>,
	// pub do_while_id: Option<Uuid>,
	// pub dynamic_id: Option<Uuid>,
	// pub dynamic_fork_id: Option<Uuid>,
	// pub event_id: Option<Uuid>,
	// pub fork_id: Option<Uuid>,
	// pub get_signed_jwt_id: Option<Uuid>,
	// pub http_id: Option<Uuid>,
	// pub inline_id: Option<Uuid>,
	// pub join_id: Option<Uuid>,
	// pub json_transform_id: Option<Uuid>,
	// pub set_variable_id: Option<Uuid>,
	// pub simple_id: Option<Uuid>,
	// pub sql_task_id: Option<Uuid>,
	// pub get_workflow_id: Option<Uuid>,
	// pub start_workflow_id: Option<Uuid>,
	// pub sub_workflow_id: Option<Uuid>,
	// pub switch_id: Option<Uuid>,
	// pub task_update_id: Option<Uuid>,
	// pub terminate_task_id: Option<Uuid>,
	// pub terminate_workflow_id: Option<Uuid>,
	// pub human_id: Option<Uuid>,
	// pub update_secret_id: Option<Uuid>,
	// pub wait_id: Option<Uuid>,
	// pub wait_for_webhook_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "crate::data::definition::Entity",
		from = "Column::TaskDefName",
		to = "crate::data::definition::Column::Name",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskDefinition,
	#[sea_orm(
		belongs_to = "crate::data::config::Entity",
		from = "Column::TaskConfigId",
		to = "crate::data::config::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskConfig,
	#[sea_orm(has_one = "crate::system::buissness::Entity")]
	BuissnessRule,
	#[sea_orm(has_one = "crate::operation::do_while::Entity")]
	DoWhile,
	#[sea_orm(has_one = "crate::operation::dynamic::Entity")]
	Dynamic,
	#[sea_orm(has_one = "crate::operation::fork::dynamic::Entity")]
	DynamicFork,
	#[sea_orm(has_one = "crate::system::event::Entity")]
	Event,
	#[sea_orm(has_one = "crate::operation::fork::fork::Entity")]
	Fork,
	#[sea_orm(has_one = "crate::system::jwt::Entity")]
	GetSignedJwt,
	#[sea_orm(has_one = "crate::system::http::Entity")]
	Http,
	#[sea_orm(has_one = "crate::system::inline::Entity")]
	Inline,
	#[sea_orm(has_one = "crate::operation::join::Entity")]
	Join,
	#[sea_orm(has_one = "crate::system::transform::json::Entity")]
	JsonTransform,
	#[sea_orm(has_one = "crate::operation::variable::Entity")]
	SetVariable,
	#[sea_orm(has_one = "crate::operation::simple::Entity")]
	Simple,
	#[sea_orm(has_one = "crate::system::sql::Entity")]
	SqlTask,
	#[sea_orm(has_one = "crate::operation::workflow::get::Entity")]
	GetWorkflow,
	#[sea_orm(has_one = "crate::operation::start::Entity")]
	StartWorkflow,
	#[sea_orm(has_one = "crate::operation::sub::Entity")]
	SubWorkflow,
	#[sea_orm(has_one = "crate::operation::switch::Entity")]
	Switch,
	#[sea_orm(has_one = "crate::operation::task::update::Entity")]
	TaskUpdate,
	#[sea_orm(has_one = "crate::operation::task::terminate::Entity")]
	TerminateTask,
	#[sea_orm(has_one = "crate::operation::workflow::terminate::Entity")]
	TerminateWorkflow,
	#[sea_orm(has_one = "crate::operation::human::Entity")]
	Human,
	#[sea_orm(has_one = "crate::system::secret::Entity")]
	UpdateSecret,
	#[sea_orm(has_one = "crate::operation::wait::Entity")]
	Wait,
	#[sea_orm(has_one = "crate::system::webhook::wait::Entity")]
	WaitForWebhook,
	#[sea_orm(has_many = "crate::data::log::Entity")]
	TaskExecutionLog,
}

impl Related<crate::data::definition::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskDefinition.def()
	}
}
impl Related<crate::data::config::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskConfig.def()
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
impl Related<crate::data::log::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskExecutionLog.def()
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
			subworkflow_changed: false,
			wait_timeout: None,
			task_config_id: None,
		}
	}
}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskStorage for Model {
	type Entity = Entity;
	type Model = Self;
	type PrimaryKey = Uuid;
	type ActiveModel = ActiveModel;

	async fn insert(self, context: &Context) -> Result<Self::Model> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn update(self, context: &Context) -> Result<Self::Model> {
		ActiveModel::update(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn save(self, context: &Context) -> Result<Self::ActiveModel> {
		ActiveModel::save(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn delete(self, context: &Context) -> Result<()> {
		ActiveModel::delete(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}

	fn find() -> Select<Self::Entity> {
		Entity::find()
	}

	async fn find_by_id(context: &Context, task_id: Self::PrimaryKey) -> Result<Self::Model> {
		let task = Entity::find_by_id(task_id)
			.one(&context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		if let Some(m) = task {
			Ok(m)
		} else {
			Err(Error::NotFound(format!("Could not find task model with id: {}", task_id)))
		}
	}
}

pub type TaskModel = Model;
