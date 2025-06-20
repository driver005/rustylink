use crate::{
	ListAccessorTrait, ObjectAccessorTrait, ValueAccessorTrait,
	prelude::{
		GraphQLListAccessor, GraphQLObjectAccessor, GraphQLValueAccessor, ProtoListAccessor,
		ProtoObjectAccessor, ProtoValueAccessor,
	},
};
use async_graphql::Name;
use indexmap::IndexMap;

use super::{Error, Value};

pub enum ValueAccessors<'a> {
	GraphQL(GraphQLValueAccessor<'a>),
	Proto(ProtoValueAccessor<'a>),
}

impl<'a> ValueAccessors<'a> {
	pub const fn graphql(value: GraphQLValueAccessor<'a>) -> Self {
		Self::GraphQL(value)
	}

	pub const fn proto(value: ProtoValueAccessor<'a>) -> Self {
		Self::Proto(value)
	}
}

impl<'a> ValueAccessorTrait<'a> for ValueAccessors<'a> {
	type Value = Value;
	type Error = Error;
	type ObjectAccessor = ObjectAccessors<'a>;
	type ListAccessor = ListAccessors<'a>;

	fn type_name(&self) -> &'static str {
		"ValueAccessors"
	}

	fn is_null(&self) -> bool {
		match self {
			ValueAccessors::GraphQL(accessor) => accessor.is_null(),
			ValueAccessors::Proto(accessor) => accessor.is_null(),
		}
	}

	fn boolean(&self) -> Result<bool, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.boolean().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.boolean().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn enum_name(&self) -> Result<&str, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.enum_name().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.enum_name().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn i32(&self) -> Result<i32, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.i32().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.i32().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn i64(&self) -> Result<i64, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.i64().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.i64().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn u32(&self) -> Result<u32, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.u32().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.u32().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn u64(&self) -> Result<u64, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.u64().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.u64().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn si32(&self) -> Result<i32, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.si32().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.si32().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn si64(&self) -> Result<i64, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.si64().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.si64().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn f32(&self) -> Result<f32, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.f32().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.f32().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn f64(&self) -> Result<f64, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.f64().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.f64().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn string(&self) -> Result<&str, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.string().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.string().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => match accessor.object() {
				Ok(accessor) => Ok(ObjectAccessors::graphql(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
			ValueAccessors::Proto(accessor) => match accessor.object() {
				Ok(accessor) => Ok(ObjectAccessors::proto(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
		}
	}

	fn list(&self) -> Result<Self::ListAccessor, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => match accessor.list() {
				Ok(accessor) => Ok(ListAccessors::graphql(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
			ValueAccessors::Proto(accessor) => match accessor.list() {
				Ok(accessor) => Ok(ListAccessors::proto(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
		}
	}

	fn as_value(&self) -> Self::Value {
		match self {
			ValueAccessors::GraphQL(accessor) => Value::graphql(accessor.as_value().clone()),
			ValueAccessors::Proto(accessor) => Value::proto(accessor.as_value().clone()),
		}
	}

	fn upload(&self) -> Result<async_graphql::Upload, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.upload().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.upload().map_err(|err| Error::new(err.message))
			}
		}
	}

	fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, Self::Error> {
		match self {
			ValueAccessors::GraphQL(accessor) => {
				accessor.deserialize().map_err(|err| Error::new(err.message))
			}
			ValueAccessors::Proto(accessor) => {
				accessor.deserialize().map_err(|err| Error::new(err.message))
			}
		}
	}
}

pub enum ObjectAccessors<'a> {
	GraphQL(GraphQLObjectAccessor<'a>),
	Proto(ProtoObjectAccessor<'a>),
}

impl<'a> ObjectAccessors<'a> {
	pub fn graphql(value: GraphQLObjectAccessor<'a>) -> Self {
		Self::GraphQL(value)
	}

	pub fn proto(value: ProtoObjectAccessor<'a>) -> Self {
		Self::Proto(value)
	}

	pub fn to_graphql(&'a self) -> &'a GraphQLObjectAccessor<'a> {
		match self {
			ObjectAccessors::GraphQL(object_accessor) => object_accessor,
			ObjectAccessors::Proto(_) => panic!("could not get GraphQLObjectAccessor"),
		}
	}

	pub fn to_proto(&'a self) -> &'a ProtoObjectAccessor<'a> {
		match self {
			ObjectAccessors::GraphQL(_) => {
				panic!("could not get ProtoObjectAccessor")
			}
			ObjectAccessors::Proto(object_accessor) => object_accessor,
		}
	}
}

impl<'a> ObjectAccessorTrait<'a> for ObjectAccessors<'a> {
	type Value = Value;
	type Error = Error;
	type ValueAccessor = ValueAccessors<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLObjectAccessor"
	}

	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor> {
		match self {
			ObjectAccessors::GraphQL(accessor) => accessor.get(name).map(ValueAccessors::graphql),
			ObjectAccessors::Proto(accessor) => accessor.get(name).map(ValueAccessors::proto),
		}
	}

	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error> {
		match self {
			ObjectAccessors::GraphQL(accessor) => match accessor.try_get(name) {
				Ok(accessor) => Ok(ValueAccessors::graphql(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
			ObjectAccessors::Proto(accessor) => match accessor.try_get(name) {
				Ok(accessor) => Ok(ValueAccessors::proto(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
		}
	}

	fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Name, Self::ValueAccessor)> + 'a> {
		match self {
			ObjectAccessors::GraphQL(accessor) => Box::new(
				accessor.to_iter().map(|(name, data)| (name, ValueAccessors::graphql(data))),
			),
			ObjectAccessors::Proto(accessor) => {
				Box::new(accessor.to_iter().map(|(name, data)| (name, ValueAccessors::proto(data))))
			}
		}
	}

	fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Name> + 'a> {
		match self {
			ObjectAccessors::GraphQL(accessor) => Box::new(accessor.keys()),
			ObjectAccessors::Proto(accessor) => Box::new(accessor.keys()),
		}
	}

	fn values(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a> {
		match self {
			ObjectAccessors::GraphQL(accessor) => {
				Box::new(accessor.values().map(ValueAccessors::graphql))
			}
			ObjectAccessors::Proto(accessor) => {
				Box::new(accessor.values().map(ValueAccessors::proto))
			}
		}
	}

	fn len(&self) -> usize {
		match self {
			ObjectAccessors::GraphQL(accessor) => accessor.len(),
			ObjectAccessors::Proto(accessor) => accessor.len(),
		}
	}

	fn is_empty(&self) -> bool {
		match self {
			ObjectAccessors::GraphQL(accessor) => accessor.is_empty(),
			ObjectAccessors::Proto(accessor) => accessor.is_empty(),
		}
	}

	fn as_index_map(&self) -> IndexMap<Name, Self::Value> {
		match self {
			ObjectAccessors::GraphQL(accessor) => accessor
				.as_index_map()
				.iter()
				.map(|data| (data.0.clone(), Value::graphql(data.1.clone())))
				.collect(),
			ObjectAccessors::Proto(accessor) => accessor
				.as_index_map()
				.iter()
				.map(|data| (data.0.clone(), Value::proto(data.1.clone())))
				.collect(),
		}
	}
}

pub enum ListAccessors<'a> {
	GraphQL(GraphQLListAccessor<'a>),
	Proto(ProtoListAccessor<'a>),
}

impl<'a> ListAccessors<'a> {
	pub const fn graphql(value: GraphQLListAccessor<'a>) -> Self {
		Self::GraphQL(value)
	}

	pub const fn proto(value: ProtoListAccessor<'a>) -> Self {
		Self::Proto(value)
	}
}

impl<'a> ListAccessorTrait<'a> for ListAccessors<'a> {
	type Value = Value;
	type Error = Error;
	type ValueAccessor = ValueAccessors<'a>;

	fn type_name(&self) -> &'static str {
		"ListAccessor"
	}

	fn len(&self) -> usize {
		match self {
			ListAccessors::GraphQL(accessor) => accessor.len(),
			ListAccessors::Proto(accessor) => accessor.len(),
		}
	}

	fn is_empty(&self) -> bool {
		match self {
			ListAccessors::GraphQL(accessor) => accessor.is_empty(),
			ListAccessors::Proto(accessor) => accessor.is_empty(),
		}
	}

	fn to_iter(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a> {
		match self {
			ListAccessors::GraphQL(accessor) => {
				Box::new(accessor.to_iter().map(ValueAccessors::graphql))
			}
			ListAccessors::Proto(accessor) => {
				Box::new(accessor.to_iter().map(ValueAccessors::proto))
			}
		}
	}

	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor> {
		match self {
			ListAccessors::GraphQL(accessor) => accessor.get(idx).map(ValueAccessors::graphql),
			ListAccessors::Proto(accessor) => accessor.get(idx).map(ValueAccessors::proto),
		}
	}

	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error> {
		match self {
			ListAccessors::GraphQL(accessor) => match accessor.try_get(idx) {
				Ok(accessor) => Ok(ValueAccessors::graphql(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
			ListAccessors::Proto(accessor) => match accessor.try_get(idx) {
				Ok(accessor) => Ok(ValueAccessors::proto(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
		}
	}

	fn as_slice(&self, start: usize, end: usize) -> Result<Self, Self::Error> {
		match self {
			ListAccessors::GraphQL(accessor) => match accessor.as_slice(start, end) {
				Ok(accessor) => Ok(ListAccessors::graphql(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
			ListAccessors::Proto(accessor) => match accessor.as_slice(start, end) {
				Ok(accessor) => Ok(ListAccessors::proto(accessor)),
				Err(err) => Err(Error::new(err.message)),
			},
		}
	}

	fn as_values_slice(&self) -> Vec<Self::Value> {
		match self {
			ListAccessors::GraphQL(accessor) => {
				accessor.as_values_slice().iter().map(|data| Value::graphql(data.clone())).collect()
			}
			ListAccessors::Proto(accessor) => {
				accessor.as_values_slice().iter().map(|data| Value::proto(data.clone())).collect()
			}
		}
	}
}
