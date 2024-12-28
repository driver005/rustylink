//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "product_images")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub product_id: String,
	#[sea_orm(primary_key, auto_increment = false)]
	pub image_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::image::Entity",
		from = "Column::ImageId",
		to = "super::image::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	Image,
	#[sea_orm(
		belongs_to = "super::product::Entity",
		from = "Column::ProductId",
		to = "super::product::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	Product,
}

impl Related<super::image::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Image.def()
	}
}

impl Related<super::product::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Product.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::image::Entity")]
	Image,
	#[sea_orm(entity = "super::product::Entity")]
	Product,
}
