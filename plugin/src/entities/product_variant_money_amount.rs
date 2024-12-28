//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "product_variant_money_amount")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	#[sea_orm(column_type = "Text", unique)]
	pub money_amount_id: String,
	#[sea_orm(column_type = "Text")]
	pub variant_id: String,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}
