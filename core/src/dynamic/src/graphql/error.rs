use std::{fmt::Display, sync::Arc};

use crate::ErrorTrait;
pub use async_graphql::Error;

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
			source: Some(Arc::new(value)),
			extensions: None,
		}
	}
}
