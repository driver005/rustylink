//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "batch_job")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	#[sea_orm(column_type = "Text")]
	pub r#type: String,
	pub created_by: Option<String>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub context: Option<Json>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub result: Option<Json>,
	pub dry_run: bool,
	pub created_at: DateTimeWithTimeZone,
	pub pre_processed_at: Option<DateTimeWithTimeZone>,
	pub confirmed_at: Option<DateTimeWithTimeZone>,
	pub processing_at: Option<DateTimeWithTimeZone>,
	pub completed_at: Option<DateTimeWithTimeZone>,
	pub failed_at: Option<DateTimeWithTimeZone>,
	pub canceled_at: Option<DateTimeWithTimeZone>,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::CreatedBy",
		to = "super::user::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	User,
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::user::Entity")]
	User,
}
