//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::PriceListStatusEnum;
use super::sea_orm_active_enums::PriceListTypeEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "price_list")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub name: String,
	pub description: String,
	pub r#type: PriceListTypeEnum,
	pub status: PriceListStatusEnum,
	pub starts_at: Option<DateTimeWithTimeZone>,
	pub ends_at: Option<DateTimeWithTimeZone>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::money_amount::Entity")]
	MoneyAmount,
	#[sea_orm(has_many = "super::price_list_customer_groups::Entity")]
	PriceListCustomerGroups,
}

impl Related<super::money_amount::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::MoneyAmount.def()
	}
}

impl Related<super::price_list_customer_groups::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PriceListCustomerGroups.def()
	}
}

impl Related<super::customer_group::Entity> for Entity {
	fn to() -> RelationDef {
		super::price_list_customer_groups::Relation::CustomerGroup.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::price_list_customer_groups::Relation::PriceList.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::money_amount::Entity")]
	MoneyAmount,
	#[sea_orm(entity = "super::price_list_customer_groups::Entity")]
	PriceListCustomerGroups,
	#[sea_orm(entity = "super::customer_group::Entity")]
	CustomerGroup,
}
