//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::ShippingOptionPriceTypeEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "shipping_option")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub name: String,
	pub region_id: String,
	pub profile_id: String,
	pub provider_id: String,
	pub price_type: ShippingOptionPriceTypeEnum,
	pub amount: Option<i32>,
	pub is_return: bool,
	#[sea_orm(column_type = "JsonBinary")]
	pub data: Json,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub admin_only: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::custom_shipping_option::Entity")]
	CustomShippingOption,
	#[sea_orm(
		belongs_to = "super::fulfillment_provider::Entity",
		from = "Column::ProviderId",
		to = "super::fulfillment_provider::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	FulfillmentProvider,
	#[sea_orm(
		belongs_to = "super::region::Entity",
		from = "Column::RegionId",
		to = "super::region::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Region,
	#[sea_orm(has_many = "super::shipping_method::Entity")]
	ShippingMethod,
	#[sea_orm(has_many = "super::shipping_option_requirement::Entity")]
	ShippingOptionRequirement,
	#[sea_orm(
		belongs_to = "super::shipping_profile::Entity",
		from = "Column::ProfileId",
		to = "super::shipping_profile::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	ShippingProfile,
	#[sea_orm(has_many = "super::shipping_tax_rate::Entity")]
	ShippingTaxRate,
}

impl Related<super::custom_shipping_option::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::CustomShippingOption.def()
	}
}

impl Related<super::fulfillment_provider::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::FulfillmentProvider.def()
	}
}

impl Related<super::region::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Region.def()
	}
}

impl Related<super::shipping_method::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingMethod.def()
	}
}

impl Related<super::shipping_option_requirement::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingOptionRequirement.def()
	}
}

impl Related<super::shipping_profile::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingProfile.def()
	}
}

impl Related<super::shipping_tax_rate::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ShippingTaxRate.def()
	}
}

impl Related<super::tax_rate::Entity> for Entity {
	fn to() -> RelationDef {
		super::shipping_tax_rate::Relation::TaxRate.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::shipping_tax_rate::Relation::ShippingOption.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::custom_shipping_option::Entity")]
	CustomShippingOption,
	#[sea_orm(entity = "super::fulfillment_provider::Entity")]
	FulfillmentProvider,
	#[sea_orm(entity = "super::region::Entity")]
	Region,
	#[sea_orm(entity = "super::shipping_method::Entity")]
	ShippingMethod,
	#[sea_orm(entity = "super::shipping_option_requirement::Entity")]
	ShippingOptionRequirement,
	#[sea_orm(entity = "super::shipping_profile::Entity")]
	ShippingProfile,
	#[sea_orm(entity = "super::shipping_tax_rate::Entity")]
	ShippingTaxRate,
	#[sea_orm(entity = "super::tax_rate::Entity")]
	TaxRate,
}
