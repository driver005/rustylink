//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "store_currencies")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub store_id: String,
	#[sea_orm(primary_key, auto_increment = false)]
	pub currency_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::currency::Entity",
		from = "Column::CurrencyCode",
		to = "super::currency::Column::Code",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	Currency,
	#[sea_orm(
		belongs_to = "super::store::Entity",
		from = "Column::StoreId",
		to = "super::store::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	Store,
}

impl Related<super::currency::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Currency.def()
	}
}

impl Related<super::store::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Store.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::currency::Entity")]
	Currency,
	#[sea_orm(entity = "super::store::Entity")]
	Store,
}
