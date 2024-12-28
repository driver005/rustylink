//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "money_amount")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub currency_code: String,
	pub amount: i32,
	pub region_id: Option<String>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	pub min_quantity: Option<i32>,
	pub max_quantity: Option<i32>,
	pub price_list_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::currency::Entity",
		from = "Column::CurrencyCode",
		to = "super::currency::Column::Code",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Currency,
	#[sea_orm(
		belongs_to = "super::price_list::Entity",
		from = "Column::PriceListId",
		to = "super::price_list::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	PriceList,
	#[sea_orm(
		belongs_to = "super::region::Entity",
		from = "Column::RegionId",
		to = "super::region::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Region,
}

impl Related<super::currency::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Currency.def()
	}
}

impl Related<super::price_list::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PriceList.def()
	}
}

impl Related<super::region::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Region.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::currency::Entity")]
	Currency,
	#[sea_orm(entity = "super::price_list::Entity")]
	PriceList,
	#[sea_orm(entity = "super::region::Entity")]
	Region,
}