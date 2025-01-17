//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "customer_group")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	#[sea_orm(unique)]
	pub name: String,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::customer_group_customers::Entity")]
	CustomerGroupCustomers,
	#[sea_orm(has_many = "super::discount_condition_customer_group::Entity")]
	DiscountConditionCustomerGroup,
	#[sea_orm(has_many = "super::price_list_customer_groups::Entity")]
	PriceListCustomerGroups,
}

impl Related<super::customer_group_customers::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::CustomerGroupCustomers.def()
	}
}

impl Related<super::discount_condition_customer_group::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DiscountConditionCustomerGroup.def()
	}
}

impl Related<super::price_list_customer_groups::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PriceListCustomerGroups.def()
	}
}

impl Related<super::customer::Entity> for Entity {
	fn to() -> RelationDef {
		super::customer_group_customers::Relation::Customer.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::customer_group_customers::Relation::CustomerGroup.def().rev())
	}
}

impl Related<super::discount_condition::Entity> for Entity {
	fn to() -> RelationDef {
		super::discount_condition_customer_group::Relation::DiscountCondition.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::discount_condition_customer_group::Relation::CustomerGroup.def().rev())
	}
}

impl Related<super::price_list::Entity> for Entity {
	fn to() -> RelationDef {
		super::price_list_customer_groups::Relation::PriceList.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::price_list_customer_groups::Relation::CustomerGroup.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::customer_group_customers::Entity")]
	CustomerGroupCustomers,
	#[sea_orm(entity = "super::discount_condition_customer_group::Entity")]
	DiscountConditionCustomerGroup,
	#[sea_orm(entity = "super::price_list_customer_groups::Entity")]
	PriceListCustomerGroups,
	#[sea_orm(entity = "super::customer::Entity")]
	Customer,
	#[sea_orm(entity = "super::discount_condition::Entity")]
	DiscountCondition,
	#[sea_orm(entity = "super::price_list::Entity")]
	PriceList,
}
