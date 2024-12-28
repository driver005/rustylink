//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::DraftOrderStatusEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "draft_order")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub status: DraftOrderStatusEnum,
	pub display_id: i32,
	#[sea_orm(unique)]
	pub cart_id: Option<String>,
	#[sea_orm(unique)]
	pub order_id: Option<String>,
	pub canceled_at: Option<DateTimeWithTimeZone>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub completed_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub idempotency_key: Option<String>,
	pub no_notification_order: Option<bool>,
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
		belongs_to = "super::order::Entity",
		from = "Column::OrderId",
		to = "super::order::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Order,
}

impl Related<super::cart::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Cart.def()
	}
}

impl Related<super::order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Order.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::cart::Entity")]
	Cart,
	#[sea_orm(entity = "super::order::Entity")]
	Order,
}