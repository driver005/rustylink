//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::CartTypeEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "cart")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub email: Option<String>,
	pub billing_address_id: Option<String>,
	pub shipping_address_id: Option<String>,
	pub region_id: String,
	pub customer_id: Option<String>,
	#[sea_orm(unique)]
	pub payment_id: Option<String>,
	pub r#type: CartTypeEnum,
	pub completed_at: Option<DateTimeWithTimeZone>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub idempotency_key: Option<String>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub context: Option<Json>,
	pub payment_authorized_at: Option<DateTimeWithTimeZone>,
	pub sales_channel_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::address::Entity",
		from = "Column::BillingAddressId",
		to = "super::address::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Address2,
	#[sea_orm(
		belongs_to = "super::address::Entity",
		from = "Column::ShippingAddressId",
		to = "super::address::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Address1,
	#[sea_orm(has_many = "super::cart_discounts::Entity")]
	CartDiscounts,
	#[sea_orm(has_many = "super::cart_gift_cards::Entity")]
	CartGiftCards,
	#[sea_orm(has_many = "super::custom_shipping_option::Entity")]
	CustomShippingOption,
	#[sea_orm(
		belongs_to = "super::customer::Entity",
		from = "Column::CustomerId",
		to = "super::customer::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Customer,
	#[sea_orm(has_one = "super::draft_order::Entity")]
	DraftOrder,
	#[sea_orm(has_many = "super::line_item::Entity")]
	LineItem,
	#[sea_orm(has_one = "super::order::Entity")]
	Order,
	#[sea_orm(
		belongs_to = "super::payment::Entity",
		from = "Column::PaymentId",
		to = "super::payment::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Payment,
	#[sea_orm(has_many = "super::payment_session::Entity")]
	PaymentSession,
	#[sea_orm(
		belongs_to = "super::region::Entity",
		from = "Column::RegionId",
		to = "super::region::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Region,
	#[sea_orm(
		belongs_to = "super::sales_channel::Entity",
		from = "Column::SalesChannelId",
		to = "super::sales_channel::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	SalesChannel,
	#[sea_orm(has_many = "super::shipping_method::Entity")]
	ShippingMethod,
	#[sea_orm(has_one = "super::swap::Entity")]
	Swap,
}

impl Related<super::cart_discounts::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::CartDiscounts.def()
	}
}

impl Related<super::cart_gift_cards::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::CartGiftCards.def()
	}
}

impl Related<super::custom_shipping_option::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::CustomShippingOption.def()
	}
}

impl Related<super::customer::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Customer.def()
	}
}

impl Related<super::draft_order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DraftOrder.def()
	}
}

impl Related<super::line_item::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::LineItem.def()
	}
}

impl Related<super::order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Order.def()
	}
}

impl Related<super::payment::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Payment.def()
	}
}

impl Related<super::payment_session::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PaymentSession.def()
	}
}

impl Related<super::region::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Region.def()
	}
}

impl Related<super::sales_channel::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SalesChannel.def()
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

impl Related<super::discount::Entity> for Entity {
	fn to() -> RelationDef {
		super::cart_discounts::Relation::Discount.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::cart_discounts::Relation::Cart.def().rev())
	}
}

impl Related<super::gift_card::Entity> for Entity {
	fn to() -> RelationDef {
		super::cart_gift_cards::Relation::GiftCard.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::cart_gift_cards::Relation::Cart.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::address::Entity", def = "Relation::Address2.def()")]
	Address2,
	#[sea_orm(entity = "super::address::Entity", def = "Relation::Address1.def()")]
	Address1,
	#[sea_orm(entity = "super::cart_discounts::Entity")]
	CartDiscounts,
	#[sea_orm(entity = "super::cart_gift_cards::Entity")]
	CartGiftCards,
	#[sea_orm(entity = "super::custom_shipping_option::Entity")]
	CustomShippingOption,
	#[sea_orm(entity = "super::customer::Entity")]
	Customer,
	#[sea_orm(entity = "super::draft_order::Entity")]
	DraftOrder,
	#[sea_orm(entity = "super::line_item::Entity")]
	LineItem,
	#[sea_orm(entity = "super::order::Entity")]
	Order,
	#[sea_orm(entity = "super::payment::Entity")]
	Payment,
	#[sea_orm(entity = "super::payment_session::Entity")]
	PaymentSession,
	#[sea_orm(entity = "super::region::Entity")]
	Region,
	#[sea_orm(entity = "super::sales_channel::Entity")]
	SalesChannel,
	#[sea_orm(entity = "super::shipping_method::Entity")]
	ShippingMethod,
	#[sea_orm(entity = "super::swap::Entity")]
	Swap,
	#[sea_orm(entity = "super::discount::Entity")]
	Discount,
	#[sea_orm(entity = "super::gift_card::Entity")]
	GiftCard,
}