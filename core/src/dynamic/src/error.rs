use crate::prelude::{GraphQLError, GraphQLSchemaError, ProtoError};
use thiserror::Error;

/// An error can occur when building dynamic schema
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub struct SchemaError(pub String);

impl From<GraphQLSchemaError> for SchemaError {
	fn from(value: GraphQLSchemaError) -> Self {
		SchemaError(value.0)
	}
}

#[derive(Error, Debug)]
pub enum SeaographyError {
	#[error("[async_graphql] {0:?}")]
	AsyncGraphQLError(GraphQLError),
	#[error("[proto] {0:?}")]
	AsyncProtoError(ProtoError),
	#[error("[int conversion] {0}")]
	TryFromIntError(#[from] std::num::TryFromIntError),
	#[error("[parsing] {0}")]
	ParseIntError(#[from] std::num::ParseIntError),
	#[error("[type conversion: {1}] {0}")]
	TypeConversionError(String, String),
	#[error("[array conversion] postgres array can not be nested type of array")]
	NestedArrayConversionError,
	#[error("[custom] {0}")]
	Custom(String),
}

impl From<GraphQLError> for SeaographyError {
	fn from(value: GraphQLError) -> Self {
		SeaographyError::AsyncGraphQLError(value)
	}
}

impl From<ProtoError> for SeaographyError {
	fn from(value: ProtoError) -> Self {
		SeaographyError::AsyncProtoError(value)
	}
}

pub type SeaResult<T> = Result<T, SeaographyError>;
