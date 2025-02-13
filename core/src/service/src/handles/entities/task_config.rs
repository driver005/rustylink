//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::EvaluatorType;
use super::sea_orm_active_enums::TaskType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "task_config")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub name: String,
	pub task_reference_name: String,
	pub task_type: TaskType,
	pub description: Option<String>,
	pub optional: bool,
	pub input_parameters: Json,
	pub async_complete: bool,
	pub start_delay: i64,
	pub permissive: bool,
	pub loop_condition: Option<String>,
	pub loop_over: Option<Json>,
	pub dynamic_task_name_param: Option<String>,
	pub dynamic_fork_tasks_param: Option<String>,
	pub dynamic_fork_tasks_input_param_name: Option<String>,
	pub fork_tasks: Option<Json>,
	pub join_on: Option<Vec<String>>,
	pub join_status: Option<String>,
	pub sub_workflow_param: Option<Json>,
	pub decision_cases: Option<Json>,
	pub default_case: Option<Json>,
	pub evaluator_type: Option<EvaluatorType>,
	pub expression: Option<String>,
	pub sink: Option<String>,
	pub trigger_failure_workflow: Option<bool>,
	pub script_expression: Option<String>,
	pub task_definition: Option<String>,
	pub rate_limited: Option<bool>,
	pub default_exclusive_join_task: Option<Vec<String>>,
	pub retry_count: Option<i32>,
	pub on_state_change: Option<Json>,
	pub cache_config: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::task_model::Entity")]
	TaskModel,
}

impl Related<super::task_model::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskModel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, macros :: DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::task_model::Entity")]
	TaskModel,
}
