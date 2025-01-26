use crate::{
	interface::{ListAccessors, ObjectAccessors, ValueAccessors},
	prelude::Name,
};
use async_graphql::Upload;
use indexmap::IndexMap;

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
	fn as_value(&self) -> Self::Value;
	fn upload(&self) -> Result<Upload, Self::Error>;
}

pub trait ObjectAccessorTrait<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor;

	fn type_name(&self) -> &'static str;
	fn get_accessor<'b>(&'b self) -> ObjectAccessors<'b>;

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
	fn to_iter(&'a self) -> impl Iterator<Item = Self::ValueAccessor>;
	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor>;
	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error>;
	fn as_slice(&self, start: usize, end: usize) -> Result<Self::ListAccessor, Self::Error>;
	fn as_values_slice(&self) -> Vec<Self::Value>;
}
