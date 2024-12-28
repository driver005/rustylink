//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "image")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub url: String,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::product_images::Entity")]
	ProductImages,
}

impl Related<super::product_images::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ProductImages.def()
	}
}

impl Related<super::product::Entity> for Entity {
	fn to() -> RelationDef {
		super::product_images::Relation::Product.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::product_images::Relation::Image.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::product_images::Entity")]
	ProductImages,
	#[sea_orm(entity = "super::product::Entity")]
	Product,
}
