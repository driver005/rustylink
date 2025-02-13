//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use crate::plugin::entities::*;
use ::apy::{Builder, BuilderContext};
use ::dynamic::{
	prelude::{Proto, Schema},
	SchemaError,
};
use sea_orm::DatabaseConnection;
lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn builder(database: &DatabaseConnection) -> Builder {
	let mut builder = Builder::new(&CONTEXT, database.clone());
	apy::register_entities!(builder, [user,]);
	builder.register_enumeration::<crate::plugin::entities::sea_orm_active_enums::UserRoleEnum>();

	builder
}

pub fn schema(
	database: &DatabaseConnection,
	depth: Option<usize>,
	complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
	let builder = builder(database);

	let schema = builder.schema_builder();
	let schema = if let Some(depth) = depth {
		schema.limit_depth(depth)
	} else {
		schema
	};
	let schema = if let Some(complexity) = complexity {
		schema.limit_complexity(complexity)
	} else {
		schema
	};
	Ok(schema.data(database.clone()).finish()?)
}

pub fn proto(database: &DatabaseConnection) -> Result<Proto, SchemaError> {
	let builder = builder(database);

	let proto = builder.proto_builder();

	Ok(proto.data(database.clone()).finish()?)
}
