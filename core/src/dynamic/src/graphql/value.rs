use crate::ValueTrait;

pub use async_graphql::Value;
use async_graphql::dynamic::FieldValue;

impl<'a> ValueTrait<'a> for Value {
	type FieldValue = FieldValue<'a>;

	fn new<T>(val: T) -> Self
	where
		T: From<T>,
		Value: From<T>,
	{
		Self::from(val)
	}

	fn into_field_value(self) -> Self::FieldValue {
		FieldValue::from(self)
	}
}
