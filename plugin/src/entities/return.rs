//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::ReturnStatusEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "return")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub status: ReturnStatusEnum,
	#[sea_orm(unique)]
	pub swap_id: Option<String>,
	pub order_id: Option<String>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub shipping_data: Option<Json>,
	pub refund_amount: i32,
	pub received_at: Option<DateTimeWithTimeZone>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub idempotency_key: Option<String>,
	#[sea_orm(unique)]
	pub claim_order_id: Option<String>,
	pub no_notification: Option<bool>,
	pub location_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::claim_order::Entity",
		from = "Column::ClaimOrderId",
		to = "super::claim_order::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	ClaimOrder,
	#[sea_orm(
		belongs_to = "super::order::Entity",
		from = "Column::OrderId",
		to = "super::order::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Order,
	#[sea_orm(has_many = "super::return_item::Entity")]
	ReturnItem,
	#[sea_orm(has_one = "super::shipping_method::Entity")]
	ShippingMethod,
	#[sea_orm(
		belongs_to = "super::swap::Entity",
		from = "Column::SwapId",
		to = "super::swap::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Swap,
}

impl Related<super::claim_order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ClaimOrder.def()
	}
}

impl Related<super::order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Order.def()
	}
}

impl Related<super::return_item::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ReturnItem.def()
	}
}

impl Related<super::shipping_method::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingMethod.def()
	}
}

impl Related<super::swap::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Swap.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::claim_order::Entity")]
	ClaimOrder,
	#[sea_orm(entity = "super::order::Entity")]
	Order,
	#[sea_orm(entity = "super::return_item::Entity")]
	ReturnItem,
	#[sea_orm(entity = "super::shipping_method::Entity")]
	ShippingMethod,
	#[sea_orm(entity = "super::swap::Entity")]
	Swap,
}