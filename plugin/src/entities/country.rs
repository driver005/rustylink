//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "country")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	#[sea_orm(unique)]
	pub iso_2: String,
	pub iso_3: String,
	pub num_code: i32,
	pub name: String,
	pub display_name: String,
	pub region_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::address::Entity")]
	Address,
	#[sea_orm(
		belongs_to = "super::region::Entity",
		from = "Column::RegionId",
		to = "super::region::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Region,
}

impl Related<super::address::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Address.def()
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
	#[sea_orm(entity = "super::address::Entity")]
	Address,
	#[sea_orm(entity = "super::region::Entity")]
	Region,
}