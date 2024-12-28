//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "region")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub name: String,
	pub currency_code: String,
	#[sea_orm(column_type = "Float")]
	pub tax_rate: f32,
	pub tax_code: Option<String>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub gift_cards_taxable: bool,
	pub automatic_taxes: bool,
	pub tax_provider_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::cart::Entity")]
	Cart,
	#[sea_orm(has_many = "super::country::Entity")]
	Country,
	#[sea_orm(
		belongs_to = "super::currency::Entity",
		from = "Column::CurrencyCode",
		to = "super::currency::Column::Code",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Currency,
	#[sea_orm(has_many = "super::discount_regions::Entity")]
	DiscountRegions,
	#[sea_orm(has_many = "super::gift_card::Entity")]
	GiftCard,
	#[sea_orm(has_many = "super::money_amount::Entity")]
	MoneyAmount,
	#[sea_orm(has_many = "super::order::Entity")]
	Order,
	#[sea_orm(has_many = "super::payment_collection::Entity")]
	PaymentCollection,
	#[sea_orm(has_many = "super::region_fulfillment_providers::Entity")]
	RegionFulfillmentProviders,
	#[sea_orm(has_many = "super::region_payment_providers::Entity")]
	RegionPaymentProviders,
	#[sea_orm(has_many = "super::shipping_option::Entity")]
	ShippingOption,
	#[sea_orm(
		belongs_to = "super::tax_provider::Entity",
		from = "Column::TaxProviderId",
		to = "super::tax_provider::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	TaxProvider,
	#[sea_orm(has_many = "super::tax_rate::Entity")]
	TaxRate,
}

impl Related<super::cart::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Cart.def()
	}
}

impl Related<super::country::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Country.def()
	}
}

impl Related<super::currency::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Currency.def()
	}
}

impl Related<super::discount_regions::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DiscountRegions.def()
	}
}

impl Related<super::gift_card::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GiftCard.def()
	}
}

impl Related<super::money_amount::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::MoneyAmount.def()
	}
}

impl Related<super::order::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Order.def()
	}
}

impl Related<super::payment_collection::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PaymentCollection.def()
	}
}

impl Related<super::region_fulfillment_providers::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::RegionFulfillmentProviders.def()
	}
}

impl Related<super::region_payment_providers::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::RegionPaymentProviders.def()
	}
}

impl Related<super::shipping_option::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingOption.def()
	}
}

impl Related<super::tax_provider::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaxProvider.def()
	}
}

impl Related<super::tax_rate::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaxRate.def()
	}
}

impl Related<super::discount::Entity> for Entity {
	fn to() -> RelationDef {
		super::discount_regions::Relation::Discount.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::discount_regions::Relation::Region.def().rev())
	}
}

impl Related<super::fulfillment_provider::Entity> for Entity {
	fn to() -> RelationDef {
		super::region_fulfillment_providers::Relation::FulfillmentProvider.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::region_fulfillment_providers::Relation::Region.def().rev())
	}
}

impl Related<super::payment_provider::Entity> for Entity {
	fn to() -> RelationDef {
		super::region_payment_providers::Relation::PaymentProvider.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::region_payment_providers::Relation::Region.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::cart::Entity")]
	Cart,
	#[sea_orm(entity = "super::country::Entity")]
	Country,
	#[sea_orm(entity = "super::currency::Entity")]
	Currency,
	#[sea_orm(entity = "super::discount_regions::Entity")]
	DiscountRegions,
	#[sea_orm(entity = "super::gift_card::Entity")]
	GiftCard,
	#[sea_orm(entity = "super::money_amount::Entity")]
	MoneyAmount,
	#[sea_orm(entity = "super::order::Entity")]
	Order,
	#[sea_orm(entity = "super::payment_collection::Entity")]
	PaymentCollection,
	#[sea_orm(entity = "super::region_fulfillment_providers::Entity")]
	RegionFulfillmentProviders,
	#[sea_orm(entity = "super::region_payment_providers::Entity")]
	RegionPaymentProviders,
	#[sea_orm(entity = "super::shipping_option::Entity")]
	ShippingOption,
	#[sea_orm(entity = "super::tax_provider::Entity")]
	TaxProvider,
	#[sea_orm(entity = "super::tax_rate::Entity")]
	TaxRate,
	#[sea_orm(entity = "super::discount::Entity")]
	Discount,
	#[sea_orm(entity = "super::fulfillment_provider::Entity")]
	FulfillmentProvider,
	#[sea_orm(entity = "super::payment_provider::Entity")]
	PaymentProvider,
}