//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "buissnessrule")]
pub struct BuissnessRule {
	#[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
	pub id: String,
	#[sea_orm(column_type = "Text")]
	pub task_configuration: String,
	#[sea_orm(column_type = "Text")]
	pub rule_file_location: String,
	#[sea_orm(column_type = "Text")]
	pub execution_strategy: String,
	#[sea_orm(column_type = "JsonBinary")]
	pub input_column: Json,
	#[sea_orm(column_type = "JsonBinary")]
	pub output_column: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::taskconfig::Entity",
		from = "Column::TaskConfiguration",
		to = "super::taskconfig::Column::TaskId",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Taskconfig,
	#[sea_orm(has_many = "super::taskmodel::Entity")]
	Taskmodel,
}

impl Related<super::taskconfig::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Taskconfig.def()
	}
}

impl Related<super::taskmodel::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Taskmodel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
