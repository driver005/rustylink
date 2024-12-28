//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "dynamicfork")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
	pub id: String,
	#[sea_orm(column_type = "Text")]
	pub r#type: String,
	#[sea_orm(column_type = "Text", nullable)]
	pub dynamic_fork_tasks_param: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub dynamic_fork_tasks_input_param_name: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub fork_task_name: Option<String>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub fork_task_inputs: Option<Json>,
	#[sea_orm(column_type = "Text", nullable)]
	pub fork_task_workflow: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub fork_task_workflow_version: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}