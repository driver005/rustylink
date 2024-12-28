//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::OrderFulfillmentStatusEnum;
use super::sea_orm_active_enums::OrderPaymentStatusEnum;
use super::sea_orm_active_enums::OrderStatusEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "order")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub status: OrderStatusEnum,
	pub fulfillment_status: OrderFulfillmentStatusEnum,
	pub payment_status: OrderPaymentStatusEnum,
	pub display_id: i32,
	#[sea_orm(unique)]
	pub cart_id: Option<String>,
	pub customer_id: String,
	pub email: String,
	pub billing_address_id: Option<String>,
	pub shipping_address_id: Option<String>,
	pub region_id: String,
	pub currency_code: String,
	#[sea_orm(column_type = "Float", nullable)]
	pub tax_rate: Option<f32>,
	pub canceled_at: Option<DateTimeWithTimeZone>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub idempotency_key: Option<String>,
	#[sea_orm(unique)]
	pub draft_order_id: Option<String>,
	pub no_notification: Option<bool>,
	pub external_id: Option<String>,
	pub sales_channel_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::address::Entity",
		from = "Column::ShippingAddressId",
		to = "super::address::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Address2,
	#[sea_orm(
		belongs_to = "super::address::Entity",
		from = "Column::BillingAddressId",
		to = "super::address::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Address1,
	#[sea_orm(
		belongs_to = "super::cart::Entity",
		from = "Column::CartId",
		to = "super::cart::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Cart,
	#[sea_orm(has_many = "super::claim_order::Entity")]
	ClaimOrder,
	#[sea_orm(
		belongs_to = "super::currency::Entity",
		from = "Column::CurrencyCode",
		to = "super::currency::Column::Code",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Currency,
	#[sea_orm(
		belongs_to = "super::customer::Entity",
		from = "Column::CustomerId",
		to = "super::customer::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Customer,
	#[sea_orm(
		belongs_to = "super::draft_order::Entity",
		from = "Column::DraftOrderId",
		to = "super::draft_order::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	DraftOrder,
	#[sea_orm(has_many = "super::fulfillment::Entity")]
	Fulfillment,
	#[sea_orm(has_many = "super::gift_card::Entity")]
	GiftCard,
	#[sea_orm(has_many = "super::gift_card_transaction::Entity")]
	GiftCardTransaction,
	#[sea_orm(has_many = "super::line_item::Entity")]
	LineItem,
	#[sea_orm(has_many = "super::order_discounts::Entity")]
	OrderDiscounts,
	#[sea_orm(has_many = "super::order_edit::Entity")]
	OrderEdit,
	#[sea_orm(has_many = "super::order_gift_cards::Entity")]
	OrderGiftCards,
	#[sea_orm(has_many = "super::payment::Entity")]
	Payment,
	#[sea_orm(has_many = "super::refund::Entity")]
	Refund,
	#[sea_orm(
		belongs_to = "super::region::Entity",
		from = "Column::RegionId",
		to = "super::region::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Region,
	#[sea_orm(has_many = "super::r#return::Entity")]
	Return,
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
	#[sea_orm(has_many = "super::swap::Entity")]
	Swap,
}

impl Related<super::cart::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Cart.def()
	}
}

impl Related<super::claim_order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ClaimOrder.def()
	}
}

impl Related<super::currency::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Currency.def()
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

impl Related<super::fulfillment::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Fulfillment.def()
	}
}

impl Related<super::gift_card_transaction::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GiftCardTransaction.def()
	}
}

impl Related<super::line_item::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::LineItem.def()
	}
}

impl Related<super::order_discounts::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::OrderDiscounts.def()
	}
}

impl Related<super::order_edit::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::OrderEdit.def()
	}
}

impl Related<super::order_gift_cards::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::OrderGiftCards.def()
	}
}

impl Related<super::payment::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Payment.def()
	}
}

impl Related<super::refund::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Refund.def()
	}
}

impl Related<super::region::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Region.def()
	}
}

impl Related<super::r#return::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Return.def()
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
		super::order_discounts::Relation::Discount.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::order_discounts::Relation::Order.def().rev())
	}
}

impl Related<super::gift_card::Entity> for Entity {
	fn to() -> RelationDef {
		super::order_gift_cards::Relation::GiftCard.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::order_gift_cards::Relation::Order.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::address::Entity", def = "Relation::Address2.def()")]
	Address2,
	#[sea_orm(entity = "super::address::Entity", def = "Relation::Address1.def()")]
	Address1,
	#[sea_orm(entity = "super::cart::Entity")]
	Cart,
	#[sea_orm(entity = "super::claim_order::Entity")]
	ClaimOrder,
	#[sea_orm(entity = "super::currency::Entity")]
	Currency,
	#[sea_orm(entity = "super::customer::Entity")]
	Customer,
	#[sea_orm(entity = "super::draft_order::Entity")]
	DraftOrder,
	#[sea_orm(entity = "super::fulfillment::Entity")]
	Fulfillment,
	#[sea_orm(entity = "super::gift_card_transaction::Entity")]
	GiftCard,
	#[sea_orm(entity = "super::line_item::Entity")]
	GiftCardTransaction,
	#[sea_orm(entity = "super::order_discounts::Entity")]
	LineItem,
	#[sea_orm(entity = "super::order_edit::Entity")]
	OrderDiscounts,
	#[sea_orm(entity = "super::order_gift_cards::Entity")]
	OrderEdit,
	#[sea_orm(entity = "super::payment::Entity")]
	OrderGiftCards,
	#[sea_orm(entity = "super::refund::Entity")]
	Payment,
	#[sea_orm(entity = "super::region::Entity")]
	Refund,
	#[sea_orm(entity = "super::r#return::Entity")]
	Region,
	#[sea_orm(entity = "super::sales_channel::Entity")]
	Return,
	#[sea_orm(entity = "super::shipping_method::Entity")]
	SalesChannel,
	#[sea_orm(entity = "super::swap::Entity")]
	ShippingMethod,
	#[sea_orm(entity = "super::discount::Entity")]
	Swap,
	#[sea_orm(entity = "super::gift_card::Entity")]
	Discount,
}