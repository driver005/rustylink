//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "event")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub sink: String,
	pub async_complete: Option<bool>,
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

#[derive(Copy, Clone, Debug, EnumIter, macros :: DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::task_model::Entity")]
	TaskModel,
}