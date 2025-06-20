use crate::{
	ProtoRegistry, ProtobufKind, ScalarValidatorFn, SchemaError, SeaResult, SeaographyError, Value,
};
use std::{
	fmt::{self, Debug},
	sync::Arc,
};

pub struct Scalar {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) validator: Option<ScalarValidatorFn>,
}

impl Debug for Scalar {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Scalar")
			.field("name", &self.name)
			.field("description", &self.description)
			.finish()
	}
}

impl Scalar {
	/// Create a GraphQL scalar type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			validator: None,
		}
	}

	// impl_set_description!();

	/// Set the validator
	#[inline]
	pub fn validator(self, validator: impl Fn(&Value) -> bool + Send + Sync + 'static) -> Self {
		Self {
			validator: Some(Arc::new(validator)),
			..self
		}
	}

	#[inline]
	pub(crate) fn validate(&self, value: &Value) -> bool {
		match &self.validator {
			Some(validator) => (validator)(value),
			None => true,
		}
	}

	pub(crate) fn to_value(&self, value: &Value) -> SeaResult<Value> {
		if self.validate(&value) {
			return Ok(value.to_owned());
		}
		Err(SeaographyError::new(format!("invalid value `{}` for scalar `{}`", value, self.name)))
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	pub(crate) fn register(&self, registry: &mut ProtoRegistry) -> Result<(), SchemaError> {
		registry.types.insert(
			self.name.clone(),
			ProtobufKind::Scalar {
				name: self.name.clone(),
				description: self.description.clone(),
				is_valid: self.validator.clone(),
				visible: None,
			},
		);
		Ok(())
	}
}
