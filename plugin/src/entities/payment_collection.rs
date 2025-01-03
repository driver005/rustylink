//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use super::sea_orm_active_enums::PaymentCollectionStatusEnum;
use super::sea_orm_active_enums::PaymentCollectionTypeEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "payment_collection")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
	pub deleted_at: Option<DateTimeWithTimeZone>,
	pub r#type: PaymentCollectionTypeEnum,
	pub status: PaymentCollectionStatusEnum,
	#[sea_orm(column_type = "Text", nullable)]
	pub description: Option<String>,
	pub amount: i32,
	pub authorized_amount: Option<i32>,
	pub region_id: String,
	pub currency_code: String,
	#[sea_orm(column_type = "JsonBinary", nullable)]
	pub metadata: Option<Json>,
	pub created_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::order_edit::Entity")]
	OrderEdit,
	#[sea_orm(has_many = "super::payment_collection_payments::Entity")]
	PaymentCollectionPayments,
	#[sea_orm(has_many = "super::payment_collection_sessions::Entity")]
	PaymentCollectionSessions,
	#[sea_orm(
		belongs_to = "super::region::Entity",
		from = "Column::RegionId",
		to = "super::region::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Region,
}

impl Related<super::order_edit::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::OrderEdit.def()
	}
}

impl Related<super::payment_collection_payments::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PaymentCollectionPayments.def()
	}
}

impl Related<super::payment_collection_sessions::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PaymentCollectionSessions.def()
	}
}

impl Related<super::region::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Region.def()
	}
}

impl Related<super::payment::Entity> for Entity {
	fn to() -> RelationDef {
		super::payment_collection_payments::Relation::Payment.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::payment_collection_payments::Relation::PaymentCollection.def().rev())
	}
}

impl Related<super::payment_session::Entity> for Entity {
	fn to() -> RelationDef {
		super::payment_collection_sessions::Relation::PaymentSession.def()
	}
	fn via() -> Option<RelationDef> {
		Some(super::payment_collection_sessions::Relation::PaymentCollection.def().rev())
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
	#[sea_orm(entity = "super::order_edit::Entity")]
	OrderEdit,
	#[sea_orm(entity = "super::payment_collection_payments::Entity")]
	PaymentCollectionPayments,
	#[sea_orm(entity = "super::payment_collection_sessions::Entity")]
	PaymentCollectionSessions,
	#[sea_orm(entity = "super::region::Entity")]
	Region,
	#[sea_orm(entity = "super::payment::Entity")]
	Payment,
	#[sea_orm(entity = "super::payment_session::Entity")]
	PaymentSession,
}
