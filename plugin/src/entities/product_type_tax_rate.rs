//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "product_type_tax_rate")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub product_type_id: String,
	#[sea_orm(primary_key, auto_increment = false)]
	pub rate_id: String,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::product_type::Entity",
		from = "Column::ProductTypeId",
		to = "super::product_type::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	ProductType,
	#[sea_orm(
		belongs_to = "super::tax_rate::Entity",
		from = "Column::RateId",
		to = "super::tax_rate::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	TaxRate,
}

impl Related<super::product_type::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ProductType.def()
	}
}

impl Related<super::tax_rate::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::TaxRate.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::product_type::Entity")]
	ProductType,
	#[sea_orm(entity = "super::tax_rate::Entity")]
	TaxRate,
}
