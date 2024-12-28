//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "store")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub name: String,
	pub default_currency_code: String,
	pub swap_link_template: Option<String>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub payment_link_template: Option<String>,
	pub invite_link_template: Option<String>,
	#[sea_orm(unique)]
	pub default_sales_channel_id: Option<String>,
	pub default_location_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::currency::Entity",
		from = "Column::DefaultCurrencyCode",
		to = "super::currency::Column::Code",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Currency,
	#[sea_orm(
		belongs_to = "super::sales_channel::Entity",
		from = "Column::DefaultSalesChannelId",
		to = "super::sales_channel::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	SalesChannel,
	#[sea_orm(has_many = "super::store_currencies::Entity")]
	StoreCurrencies,
}

impl Related<super::sales_channel::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SalesChannel.def()
	}
}

impl Related<super::store_currencies::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::StoreCurrencies.def()
	}
}

impl Related<super::currency::Entity> for Entity {
	fn to() -> RelationDef {
		super::store_currencies::Relation::Currency.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::store_currencies::Relation::Store.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::sales_channel::Entity")]
	Currency,
	#[sea_orm(entity = "super::store_currencies::Entity")]
	SalesChannel,
	#[sea_orm(entity = "super::currency::Entity")]
	StoreCurrencies,
}
