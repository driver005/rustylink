use binary::proto::{DecoderError, EncoderError};
use std::{
	any::Any,
	fmt::{self, Debug, Display, Formatter},
	sync::Arc,
};

/// An error with a message and optional extensions.
#[derive(Clone)]
pub struct Error {
	/// The error message.
	pub message: String,
	/// The source of the error.
	pub source: Option<Arc<dyn Any + Send + Sync>>,
	/// Prost decode error.
	pub decode_error: Option<DecoderError>,
	/// Prost encoder error.
	pub encoder_error: Option<EncoderError>,
}

impl Debug for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Error")
			.field("message", &self.message)
			.field("decode_error", &self.decode_error)
			.finish()
	}
}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		self.message.eq(&other.message) && self.decode_error.eq(&other.decode_error)
	}
}

impl Error {
	/// Create an error from the given error message.
	pub fn new(message: impl Into<String>) -> Self {
		Self {
			message: message.into(),
			source: None,
			decode_error: None,
			encoder_error: None,
		}
	}

	/// Create an error with a type that implements `Display`, and it will also
	/// set the `source` of the error to this value.
	pub fn new_with_source(source: impl Display + Send + Sync + 'static) -> Self {
		Self {
			message: source.to_string(),
			source: Some(Arc::new(source)),
			decode_error: None,
			encoder_error: None,
		}
	}

	pub fn new_decoder(err: DecoderError) -> Self {
		Self {
			message: err.to_string(),
			source: None,
			decode_error: Some(err),
			encoder_error: None,
		}
	}

	pub fn new_encoder(err: EncoderError) -> Self {
		Self {
			message: err.to_string(),
			source: None,
			decode_error: None,
			encoder_error: Some(err),
		}
	}

	// /// Convert the error to a server error.
	// #[must_use]
	// pub fn into_server_error(self, pos: Pos) -> ServerError {
	// 	ServerError {
	// 		message: self.message,
	// 		source: self.source,
	// 		locations: vec![pos],
	// 		path: Vec::new(),
	// 		extensions: self.extensions,
	// 	}
	// }
}

impl<T: Display + Send + Sync> From<T> for Error {
	fn from(e: T) -> Self {
		Self {
			message: e.to_string(),
			source: None,
			decode_error: None,
			encoder_error: None,
		}
	}
}

// impl From<DecoderError> for Error {
// 	fn from(err: DecoderError) -> Self {
// 		Self {
// 			message: err.to_string(),
// 			source: None,
// 			decode_error: Some(err),
// 			encoder_error: None,
// 		}
// 	}
// }

// impl From<EncoderError> for Error {
// 	fn from(err: EncoderError) -> Self {
// 		Self {
// 			message: err.to_string(),
// 			source: None,
// 			decode_error: None,
// 			encoder_error: Some(err),
// 		}
// 	}
// }

/// An alias for `Result<T, Error>`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
