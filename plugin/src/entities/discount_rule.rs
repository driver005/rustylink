//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::DiscountRuleAllocationEnum;
use super::sea_orm_active_enums::DiscountRuleTypeEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "discount_rule")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub description: Option<String>,
	pub r#type: DiscountRuleTypeEnum,
	pub value: i32,
	pub allocation: Option<DiscountRuleAllocationEnum>,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::discount::Entity")]
	Discount,
	#[sea_orm(has_many = "super::discount_condition::Entity")]
	DiscountCondition,
	#[sea_orm(has_many = "super::discount_rule_products::Entity")]
	DiscountRuleProducts,
}

impl Related<super::discount::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Discount.def()
	}
}

impl Related<super::discount_condition::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DiscountCondition.def()
	}
}

impl Related<super::discount_rule_products::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DiscountRuleProducts.def()
	}
}

impl Related<super::product::Entity> for Entity {
	fn to() -> RelationDef {
		super::discount_rule_products::Relation::Product.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::discount_rule_products::Relation::DiscountRule.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::discount::Entity")]
	Discount,
	#[sea_orm(entity = "super::discount_condition::Entity")]
	DiscountCondition,
	#[sea_orm(entity = "super::discount_rule_products::Entity")]
	DiscountRuleProducts,
	#[sea_orm(entity = "super::product::Entity")]
	Product,
}
