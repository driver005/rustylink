//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "cart_discounts")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub cart_id: String,
	#[sea_orm(primary_key, auto_increment = false)]
	pub discount_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::cart::Entity",
		from = "Column::CartId",
		to = "super::cart::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	Cart,
	#[sea_orm(
		belongs_to = "super::discount::Entity",
		from = "Column::DiscountId",
		to = "super::discount::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	Discount,
}

impl Related<super::cart::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Cart.def()
	}
}

impl Related<super::discount::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Discount.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::cart::Entity")]
	Cart,
	#[sea_orm(entity = "super::discount::Entity")]
	Discount,
}