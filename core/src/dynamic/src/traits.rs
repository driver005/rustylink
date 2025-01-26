use std::{any::Any, borrow::Cow};

use crate::{Registry, SchemaError};

pub trait EnumTrait {
	type Item;

	fn new(name: impl Into<String>) -> Self;

	fn item(self, item: impl Into<Self::Item>) -> Self;

	fn items(self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self;

	fn type_name(&self) -> String;
}

/// Trait for common behaviors of TypeRef
pub trait TypeRefTrait: Sized {
	fn named(type_name: impl Into<String>) -> Self;
	fn named_nn(type_name: impl Into<String>) -> Self;
	fn named_list(type_name: impl Into<String>) -> Self;
	fn named_nn_list(type_name: impl Into<String>) -> Self;
	fn named_list_nn(type_name: impl Into<String>) -> Self;
	fn named_nn_list_nn(type_name: impl Into<String>) -> Self;
	fn type_name(&self) -> &str;
}

pub trait FieldValueTrait<'a>: Sized {
	type Value;
	type Error;

	const NULL: Self;
	const NONE: Option<Self>;

	fn null() -> Self;
	fn none() -> Option<Self>;
	fn value(value: impl Into<Self::Value>) -> Self;
	fn owned_any(obj: impl Any + Send + Sync) -> Self;
	fn boxed_any(obj: Box<dyn Any + Send + Sync>) -> Self;
	fn borrowed_any(obj: &'a (dyn Any + Send + Sync)) -> Self;
	fn list<I, T>(values: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<Self>;
	fn with_type(self, ty: impl Into<Cow<'static, str>>) -> Self;
	fn as_value(&self) -> Option<&Self::Value>;
	fn try_to_value(&self) -> Result<&Self::Value, Self::Error>;
	fn as_list(&'a self) -> Option<&[Self]>;
	fn try_to_list(&'a self) -> Result<&[Self], Self::Error>;
	fn downcast_ref<T: Any>(&self) -> Option<&T>;
	fn try_downcast_ref<T: Any>(&self) -> Result<&T, Self::Error>;
	fn to_val(&self) -> Option<Self::Value>;
}
