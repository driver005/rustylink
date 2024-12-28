//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "payment_collection_sessions")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub payment_collection_id: String,
	#[sea_orm(primary_key, auto_increment = false)]
	pub payment_session_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::payment_collection::Entity",
		from = "Column::PaymentCollectionId",
		to = "super::payment_collection::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	PaymentCollection,
	#[sea_orm(
		belongs_to = "super::payment_session::Entity",
		from = "Column::PaymentSessionId",
		to = "super::payment_session::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	PaymentSession,
}

impl Related<super::payment_collection::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PaymentCollection.def()
	}
}

impl Related<super::payment_session::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PaymentSession.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::payment_collection::Entity")]
	PaymentCollection,
	#[sea_orm(entity = "super::payment_session::Entity")]
	PaymentSession,
}
