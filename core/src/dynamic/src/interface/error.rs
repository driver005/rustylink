use std::fmt::Display;

use crate::{
	ErrorTrait,
	prelude::{GraphQLError, ProtoError},
};

#[derive(Debug)]
pub struct Error {
	pub message: String,
}

impl ErrorTrait for Error {
	fn new(message: impl Into<String>) -> Self {
		Self::new(message)
	}

	fn to<T>(value: T) -> Self
	where
		T: Display + Send + Sync + 'static,
	{
		Self {
			message: value.to_string(),
		}
	}
}

impl Error {
	pub fn new(message: impl Into<String>) -> Self {
		Self {
			message: message.into(),
		}
	}
	pub(crate) fn to_graphql(self) -> GraphQLError {
		GraphQLError::new(self.message)
	}

	pub(crate) fn to_proto(self) -> ProtoError {
		ProtoError::new(self.message)
	}
}

// impl<T: Display + Send + Sync> From<T> for Error {
// 	fn from(e: T) -> Self {
// 		Self {
// 			message: e.to_string(),
// 		}
// 	}
// }

/// An alias for `Result<T, Error>`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
