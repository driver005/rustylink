//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "custom_shipping_option")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub price: i32,
	pub shipping_option_id: String,
	pub cart_id: Option<String>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::cart::Entity",
		from = "Column::CartId",
		to = "super::cart::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Cart,
	#[sea_orm(
		belongs_to = "super::shipping_option::Entity",
		from = "Column::ShippingOptionId",
		to = "super::shipping_option::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	ShippingOption,
}

impl Related<super::cart::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Cart.def()
	}
}

impl Related<super::shipping_option::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingOption.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::cart::Entity")]
	Cart,
	#[sea_orm(entity = "super::shipping_option::Entity")]
	ShippingOption,
}