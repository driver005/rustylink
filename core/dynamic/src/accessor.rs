use crate::prelude::{
	GraphQLError, GraphQLListAccessor, GraphQLObjectAccessor, GraphQLValue, GraphQLValueAccessor,
	Name, ProtoError, ProtoListAccessor, ProtoObjectAccessor, ProtoValue, ProtoValueAccessor,
};
use async_graphql::Upload;
use indexmap::IndexMap;

pub enum ValueAccessors<'a> {
	GraphQL(&'a GraphQLValueAccessor<'a>),
	Proto(&'a ProtoValueAccessor<'a>),
}

pub trait ValueAccessor<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ObjectAccessor: ObjectAccessor<'a>;
	type ListAccessor: ListAccessor<'a>;

	fn type_name(&self) -> &'static str;
	fn get_accessor<'b>(&'b self) -> ValueAccessors<'b>;

	fn is_null(&self) -> bool;
	fn boolean(&self) -> Result<bool, Self::Error>;
	fn enum_name(&self) -> Result<&str, Self::Error>;
	fn i64(&self) -> Result<i64, Self::Error>;
	fn u64(&self) -> Result<u64, Self::Error>;
	fn f32(&self) -> Result<f32, Self::Error>;
	fn f64(&self) -> Result<f64, Self::Error>;
	fn string(&self) -> Result<&str, Self::Error>;
	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error>;
	fn list(&self) -> Result<Self::ListAccessor, Self::Error>;
	fn as_value(&self) -> &Self::Value;
	fn upload(&self) -> Result<Upload, Self::Error>;
}

pub enum ObjectAccessors<'a> {
	GraphQL(&'a GraphQLObjectAccessor<'a>),
	Proto(&'a ProtoObjectAccessor<'a>),
}

pub trait ObjectAccessor<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor: ValueAccessor<'a>;

	fn type_name(&self) -> &'static str;
	fn get_accessor<'b>(&'b self) -> ObjectAccessors<'b>;

	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor>;
	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error>;
	fn to_iter(&'a self) -> impl Iterator<Item = (&Name, Self::ValueAccessor)>;
	fn keys(&self) -> impl Iterator<Item = &Name>;
	fn values(&'a self) -> impl Iterator<Item = Self::ValueAccessor>;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn as_index_map(&self) -> &IndexMap<Name, Self::Value>;
}

pub enum ListAccessors<'a> {
	GraphQL(&'a GraphQLListAccessor<'a>),
	Proto(&'a ProtoListAccessor<'a>),
}

pub trait ListAccessor<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor: ValueAccessor<'a>;
	type ListAccessor: ListAccessor<'a>;

	fn type_name(&self) -> &'static str;
	fn get_accessor<'b>(&'b self) -> ListAccessors<'b>;

	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn to_iter(&'a self) -> impl Iterator<Item = Self::ValueAccessor>;
	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor>;
	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error>;
	fn as_slice(&self, start: usize, end: usize) -> Result<Self::ListAccessor, Self::Error>;
	fn as_values_slice(&self) -> &[Self::Value];
}

impl<'a> ValueAccessor<'a> for GraphQLValueAccessor<'a> {
	type Value = GraphQLValue;
	type Error = GraphQLError;
	type ObjectAccessor = GraphQLObjectAccessor<'a>;
	type ListAccessor = GraphQLListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLValueAccessor"
	}

	fn get_accessor<'b>(&'b self) -> ValueAccessors<'b> {
		ValueAccessors::GraphQL(self)
	}

	fn is_null(&self) -> bool {
		self.is_null()
	}

	fn boolean(&self) -> Result<bool, Self::Error> {
		self.boolean()
	}

	fn enum_name(&self) -> Result<&str, Self::Error> {
		self.enum_name()
	}

	fn i64(&self) -> Result<i64, Self::Error> {
		self.i64()
	}

	fn u64(&self) -> Result<u64, Self::Error> {
		self.u64()
	}

	fn f32(&self) -> Result<f32, Self::Error> {
		self.f32()
	}

	fn f64(&self) -> Result<f64, Self::Error> {
		self.f64()
	}

	fn string(&self) -> Result<&str, Self::Error> {
		self.string()
	}

	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error> {
		self.object()
	}

	fn list(&self) -> Result<Self::ListAccessor, Self::Error> {
		self.list()
	}

	fn as_value(&self) -> &Self::Value {
		self.as_value()
	}

	fn upload(&self) -> Result<Upload, Self::Error> {
		self.upload()
	}
}

impl<'a> ValueAccessor<'a> for ProtoValueAccessor<'a> {
	type Value = ProtoValue;
	type Error = ProtoError;
	type ObjectAccessor = ProtoObjectAccessor<'a>;
	type ListAccessor = ProtoListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ProtoValueAccessor"
	}

	fn get_accessor<'b>(&'b self) -> ValueAccessors<'b> {
		ValueAccessors::Proto(self)
	}

	fn is_null(&self) -> bool {
		self.is_null()
	}

	fn boolean(&self) -> Result<bool, Self::Error> {
		self.boolean()
	}

	fn enum_name(&self) -> Result<&str, Self::Error> {
		self.enum_name()
	}

	fn i64(&self) -> Result<i64, Self::Error> {
		self.i64()
	}

	fn u64(&self) -> Result<u64, Self::Error> {
		self.u64()
	}

	fn f32(&self) -> Result<f32, Self::Error> {
		self.f32()
	}

	fn f64(&self) -> Result<f64, Self::Error> {
		self.f64()
	}

	fn string(&self) -> Result<&str, Self::Error> {
		self.string()
	}

	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error> {
		self.object()
	}

	fn list(&self) -> Result<Self::ListAccessor, Self::Error> {
		self.list()
	}

	fn as_value(&self) -> &Self::Value {
		self.as_value()
	}

	fn upload(&self) -> Result<Upload, Self::Error> {
		panic!("ProtoValueAccessor::upload() is not a vailid function use GraphQLValueAccessor::upload() instead")
	}
}

impl<'a> ObjectAccessor<'a> for GraphQLObjectAccessor<'a> {
	type Value = GraphQLValue;
	type Error = GraphQLError;
	type ValueAccessor = GraphQLValueAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLObjectAccessor"
	}

	fn get_accessor<'b>(&'b self) -> ObjectAccessors<'b> {
		ObjectAccessors::GraphQL(self)
	}

	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor> {
		self.get(name)
	}

	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error> {
		self.try_get(name)
	}

	fn to_iter(&'a self) -> impl Iterator<Item = (&Name, Self::ValueAccessor)> {
		self.iter()
	}

	fn keys(&self) -> impl Iterator<Item = &Name> {
		self.keys()
	}

	fn values(&'a self) -> impl Iterator<Item = Self::ValueAccessor> {
		self.values()
	}

	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn as_index_map(&self) -> &IndexMap<Name, Self::Value> {
		self.as_index_map()
	}
}

impl<'a> ObjectAccessor<'a> for ProtoObjectAccessor<'a> {
	type Value = ProtoValue;
	type Error = ProtoError;
	type ValueAccessor = ProtoValueAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ProtoObjectAccessor"
	}

	fn get_accessor<'b>(&'b self) -> ObjectAccessors<'b> {
		ObjectAccessors::Proto(self)
	}

	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor> {
		self.get(name)
	}

	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error> {
		self.try_get(name)
	}

	fn to_iter(&'a self) -> impl Iterator<Item = (&Name, Self::ValueAccessor)> {
		self.iter()
	}

	fn keys(&self) -> impl Iterator<Item = &Name> {
		self.keys()
	}

	fn values(&'a self) -> impl Iterator<Item = Self::ValueAccessor> {
		self.values()
	}

	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn as_index_map(&self) -> &IndexMap<Name, Self::Value> {
		self.as_index_map()
	}
}

impl<'a> ListAccessor<'a> for GraphQLListAccessor<'a> {
	type Value = GraphQLValue;
	type Error = GraphQLError;
	type ValueAccessor = GraphQLValueAccessor<'a>;
	type ListAccessor = GraphQLListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLListAccessor"
	}

	fn get_accessor<'b>(&'b self) -> ListAccessors<'b> {
		ListAccessors::GraphQL(self)
	}

	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn to_iter(&'a self) -> impl Iterator<Item = Self::ValueAccessor> {
		self.iter()
	}

	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor> {
		self.get(idx)
	}

	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error> {
		self.try_get(idx)
	}

	fn as_slice(&self, start: usize, end: usize) -> Result<Self::ListAccessor, Self::Error> {
		self.as_slice(start, end)
	}

	fn as_values_slice(&self) -> &[Self::Value] {
		self.as_values_slice()
	}
}

impl<'a> ListAccessor<'a> for ProtoListAccessor<'a> {
	type Value = ProtoValue;
	type Error = ProtoError;
	type ValueAccessor = ProtoValueAccessor<'a>;
	type ListAccessor = ProtoListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ProtoListAccessor"
	}

	fn get_accessor<'b>(&'b self) -> ListAccessors<'b> {
		ListAccessors::Proto(self)
	}

	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn to_iter(&'a self) -> impl Iterator<Item = Self::ValueAccessor> {
		self.iter()
	}

	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor> {
		self.get(idx)
	}

	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error> {
		self.try_get(idx)
	}

	fn as_slice(&self, start: usize, end: usize) -> Result<Self::ListAccessor, Self::Error> {
		self.as_slice(start, end)
	}

	fn as_values_slice(&self) -> &[Self::Value] {
		self.as_values_slice()
	}
}
