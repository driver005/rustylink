use crate::{
	interface::{ListAccessors, ObjectAccessors, ValueAccessors},
	prelude::Name,
};
use async_graphql::Upload;
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use std::{any::Any, borrow::Cow};

pub trait ValueAccessorTrait<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ObjectAccessor;
	type ListAccessor;

	fn type_name(&self) -> &'static str;
	fn get_accessor<'b>(&'b self) -> ValueAccessors<'b>;

	fn is_null(&self) -> bool;
	fn boolean(&self) -> Result<bool, Self::Error>;
	fn enum_name(&self) -> Result<&str, Self::Error>;
	fn i32(&self) -> Result<i32, Self::Error>;
	fn i64(&self) -> Result<i64, Self::Error>;
	fn u32(&self) -> Result<u32, Self::Error>;
	fn u64(&self) -> Result<u64, Self::Error>;
	fn si32(&self) -> Result<i32, Self::Error>;
	fn si64(&self) -> Result<i64, Self::Error>;
	fn f32(&self) -> Result<f32, Self::Error>;
	fn f64(&self) -> Result<f64, Self::Error>;
	fn string(&self) -> Result<&str, Self::Error>;
	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error>;
	fn list(&self) -> Result<Self::ListAccessor, Self::Error>;
	fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Self::Error>;
	fn as_value(&self) -> Self::Value;
	fn upload(&self) -> Result<Upload, Self::Error>;
}

pub trait ObjectAccessorTrait<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor;

	fn type_name(&self) -> &'static str;
	fn get_accessor(self) -> ObjectAccessors<'a>;

	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor>;
	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error>;
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Name, Self::ValueAccessor)> + 'a>;
	fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Name> + 'a>;
	fn values(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a>;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn as_index_map(&self) -> IndexMap<Name, Self::Value>;
}

pub trait ListAccessorTrait<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor;
	type ListAccessor;

	fn type_name(&self) -> &'static str;
	fn get_accessor<'b>(&'b self) -> ListAccessors<'b>;

	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a>;
	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor>;
	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error>;
	fn as_slice(&self, start: usize, end: usize) -> Result<Self::ListAccessor, Self::Error>;
	fn as_values_slice(&self) -> Vec<Self::Value>;
}

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
	fn owned_any<T: Any + Send + Sync>(obj: T) -> Self;
	fn boxed_any(obj: Box<dyn Any + Send + Sync>) -> Self;
	fn borrowed_any(obj: &'a (dyn Any + Send + Sync)) -> Self;
	fn list<I, T>(values: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<Self>;
	fn with_type(self, ty: impl Into<Cow<'static, str>>) -> Self;
	fn as_value(&self) -> Option<&Self::Value>;
	fn try_to_value(&self) -> Result<&Self::Value, Self::Error>;
	fn as_list(&'a self) -> Option<&'a [Self]>;
	fn try_to_list(&'a self) -> Result<&'a [Self], Self::Error>;
	fn downcast_ref<T: Any>(&self) -> Option<&T>;
	fn try_downcast_ref<T: Any>(&self) -> Result<&T, Self::Error>;
	fn to_val(&self) -> Option<Self::Value>;
}

pub trait ResolverContextDyn<'a> {
	type Context;
	type ObjectAccessor;
	type FieldValue;

	fn ctx(&'a self) -> &'a Self::Context;
	fn args(self) -> Self::ObjectAccessor;
	fn parent_value(&'a self) -> &'a Self::FieldValue;
}
