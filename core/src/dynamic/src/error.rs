use std::sync::Arc;

use crate::prelude::{GraphQLServerError, ProtoError};
use binary::proto::{DecoderError, EncoderError};
use juniper::{FieldError, IntoFieldError};
use thiserror::Error;

/// An error can occur when building dynamic schema
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub struct SchemaError(pub String);

#[derive(Error, Debug)]
pub enum SeaographyError {
	// #[error("[async_graphql] {0:?}")]
	// AsyncGraphQLError(GraphQLError),
	#[error("[async_graphql] {0:?}")]
	AsyncGraphQLError(GraphQLServerError),
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
	#[error("[decoder] {0}")]
	DecoderError(DecoderError),
	#[error("[encoder] {0}")]
	EncoderError(EncoderError),
	#[error("[db] {0}")]
	DB(sea_orm::DbErr),
}

impl<S> IntoFieldError<S> for SeaographyError {
	fn into_field_error(self) -> FieldError<S> {
		FieldError::from(self.to_string())
	}
}

impl SeaographyError {
	pub fn new(message: impl Into<String>) -> Self {
		Self::Custom(message.into())
	}
}

impl From<GraphQLServerError> for SeaographyError {
	fn from(value: GraphQLServerError) -> Self {
		SeaographyError::AsyncGraphQLError(value)
	}
}

impl From<ProtoError> for SeaographyError {
	fn from(value: ProtoError) -> Self {
		SeaographyError::AsyncProtoError(value)
	}
}

impl From<DecoderError> for SeaographyError {
	fn from(value: DecoderError) -> Self {
		SeaographyError::DecoderError(value)
	}
}

impl From<EncoderError> for SeaographyError {
	fn from(value: EncoderError) -> Self {
		SeaographyError::EncoderError(value)
	}
}

impl From<sea_orm::DbErr> for SeaographyError {
	fn from(value: sea_orm::DbErr) -> Self {
		SeaographyError::DB(value)
	}
}

impl From<Arc<sea_orm::DbErr>> for SeaographyError {
	fn from(value: Arc<sea_orm::DbErr>) -> Self {
		match Arc::try_unwrap(value) {
			Ok(inner) => SeaographyError::DB(inner),
			Err(arc) => SeaographyError::new("couldnot unwrap db error"),
		}
	}
}

pub type SeaResult<T> = Result<T, SeaographyError>;
