use chrono::{DateTime, Utc};
use metadata::{Error, Result, RetryLogic, SchemaDef, TimeoutPolicy};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "task_definition")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
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
	pub rate_limit_per_frequency: Option<i32>,
	pub rate_limit_frequency_in_seconds: Option<i32>,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "crate::model::Entity")]
	Taskmodel,
}

impl Related<crate::model::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Taskmodel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
	pub const ONE_HOUR: i64 = 60 * 60;

	// Constructor
	pub fn new(name: String) -> Self {
		Self {
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

	pub async fn save(self, context: &mut Context) -> Result<()> {
		let existing = Entity::find()
			.filter(Column::Name.eq(self.name.clone()))
			.one(&context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		if existing.is_some() {
			return Err(Error::Conflict(format!("Task definition already exists: {}", self.name)));
		}

		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}
}

pub type TaskDefinition = Model;
