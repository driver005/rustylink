//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::IdempotencyStrategy;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "start_workflow")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub name: String,
	pub version: Option<i64>,
	pub correlation_id: Option<String>,
	pub idempotency_key: Option<String>,
	pub idempotency_strategy: Option<IdempotencyStrategy>,
	pub task_model_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::task_model::Entity",
		from = "Column::TaskModelId",
		to = "super::task_model::Column::TaskId",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaskModel,
}

impl Related<super::task_model::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaskModel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}