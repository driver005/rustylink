use async_graphql::{dynamic::FieldValue, Error, Value};

use crate::traits::FieldValueTrait;

impl<'a> FieldValueTrait<'a> for FieldValue<'a> {
	type Value = Value;
	type Error = Error;

	const NULL: Self = Self::NULL;

	const NONE: Option<Self> = Self::NONE;

	fn null() -> Self {
		Self::NULL
	}

	fn none() -> Option<Self> {
		None
	}

	fn value(value: impl Into<Self::Value>) -> Self {
		Self::value(value)
	}

	fn owned_any(obj: impl std::any::Any + Send + Sync) -> Self {
		Self::owned_any(obj)
	}

	fn boxed_any(obj: Box<dyn std::any::Any + Send + Sync>) -> Self {
		Self::boxed_any(obj)
	}

	fn borrowed_any(obj: &'a (dyn std::any::Any + Send + Sync)) -> Self {
		Self::borrowed_any(obj)
	}

	fn list<I, T>(values: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<Self>,
	{
		Self::list(values)
	}

	fn with_type(self, ty: impl Into<std::borrow::Cow<'static, str>>) -> Self {
		self.with_type(ty)
	}

	fn as_value(&self) -> Option<&Self::Value> {
		self.as_value()
	}

	fn try_to_value(&self) -> Result<&Self::Value, Self::Error> {
		self.try_to_value()
	}

	fn as_list(&'a self) -> Option<&'a [Self]> {
		self.as_list()
	}

	fn try_to_list(&'a self) -> Result<&'a [Self], Self::Error> {
		self.try_to_list()
	}

	fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
		self.downcast_ref()
	}

	fn try_downcast_ref<T: std::any::Any>(&self) -> Result<&T, Self::Error> {
		self.try_downcast_ref()
	}

	fn to_val(&self) -> Option<Self::Value> {
		panic!("GraphQLFieldValue::to_val() is not a valid function use ProtoFieldValue::to_val() instead")
	}
}
