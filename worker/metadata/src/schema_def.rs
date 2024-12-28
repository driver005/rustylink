use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

use crate::Auditable;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct SchemaDef {
	pub auditable: Auditable, // Composition: SchemaDef contains Auditable
	pub name: String,
	pub version: u32,
	pub schema_type: SchemaType,
	pub external_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub enum SchemaType {
	JSON,
	AVRO,
	PROTOBUF,
}

impl Default for SchemaType {
	fn default() -> Self {
		SchemaType::JSON
	}
}
