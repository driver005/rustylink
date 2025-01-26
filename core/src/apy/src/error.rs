use dynamic::prelude::{Error as NormalError, GraphQLError, ProtoError};
use thiserror::Error;

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

//TODO: Implement this conversion
impl From<NormalError> for SeaographyError {
	fn from(value: NormalError) -> Self {
		SeaographyError::Custom(value.message)
	}
}

pub type SeaResult<T> = Result<T, SeaographyError>;
