//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "wait")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub until: Option<DateTimeWithTimeZone>,
	pub duration: Option<String>,
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
