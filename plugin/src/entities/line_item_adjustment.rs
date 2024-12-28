//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "line_item_adjustment")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub item_id: String,
	pub description: String,
	pub discount_id: Option<String>,
	pub amount: Decimal,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::discount::Entity",
		from = "Column::DiscountId",
		to = "super::discount::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Discount,
	#[sea_orm(
		belongs_to = "super::line_item::Entity",
		from = "Column::ItemId",
		to = "super::line_item::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	LineItem,
}

impl Related<super::discount::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Discount.def()
	}
}

impl Related<super::line_item::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::LineItem.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::discount::Entity")]
	Discount,
	#[sea_orm(entity = "super::line_item::Entity")]
	LineItem,
}
