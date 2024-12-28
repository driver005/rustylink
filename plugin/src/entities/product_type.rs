//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "product_type")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub value: String,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::discount_condition_product_type::Entity")]
	DiscountConditionProductType,
	#[sea_orm(has_many = "super::product::Entity")]
	Product,
	#[sea_orm(has_many = "super::product_type_tax_rate::Entity")]
	ProductTypeTaxRate,
}

impl Related<super::discount_condition_product_type::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DiscountConditionProductType.def()
	}
}

impl Related<super::product::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Product.def()
	}
}

impl Related<super::product_type_tax_rate::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ProductTypeTaxRate.def()
	}
}

impl Related<super::discount_condition::Entity> for Entity {
	fn to() -> RelationDef {
		super::discount_condition_product_type::Relation::DiscountCondition.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::discount_condition_product_type::Relation::ProductType.def().rev())
	}
}

impl Related<super::tax_rate::Entity> for Entity {
	fn to() -> RelationDef {
		super::product_type_tax_rate::Relation::TaxRate.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::product_type_tax_rate::Relation::ProductType.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::discount_condition_product_type::Entity")]
	DiscountConditionProductType,
	#[sea_orm(entity = "super::product::Entity")]
	Product,
	#[sea_orm(entity = "super::product_type_tax_rate::Entity")]
	ProductTypeTaxRate,
	#[sea_orm(entity = "super::discount_condition::Entity")]
	DiscountCondition,
	#[sea_orm(entity = "super::tax_rate::Entity")]
	TaxRate,
}
