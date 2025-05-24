use super::{Error, Name, Upload, Value};
use crate::{
	// interface::{ListAccessors, ObjectAccessors, ValueAccessors},
	ListAccessorTrait,
	ObjectAccessorTrait,
	ValueAccessorTrait,
};
pub use async_graphql::dynamic::{ListAccessor, ObjectAccessor, ValueAccessor};

use indexmap::IndexMap;

impl<'a> ValueAccessorTrait<'a> for ValueAccessor<'a> {
	type Value = Value;
	type Error = Error;
	type ObjectAccessor = ObjectAccessor<'a>;
	type ListAccessor = ListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLValueAccessor"
	}

	// fn get_accessor<'b>(&'b self) -> ValueAccessors<'b> {
	// 	todo!()
	// }

	fn is_null(&self) -> bool {
		self.is_null()
	}

	fn boolean(&self) -> Result<bool, Self::Error> {
		self.boolean()
	}

	fn enum_name(&self) -> Result<&str, Self::Error> {
		self.enum_name()
	}

	fn i32(&self) -> Result<i32, Self::Error> {
		panic!(
			"GraphQLValueAccessor::i32() is not a vailid function use ProtoValueAccessor::i32() instead"
		)
	}

	fn i64(&self) -> Result<i64, Self::Error> {
		self.i64()
	}

	fn u32(&self) -> Result<u32, Self::Error> {
		panic!(
			"GraphQLValueAccessor::u32() is not a vailid function use ProtoValueAccessor::u32() instead"
		)
	}

	fn u64(&self) -> Result<u64, Self::Error> {
		self.u64()
	}

	fn si32(&self) -> Result<i32, Self::Error> {
		panic!(
			"GraphQLValueAccessor::si32() is not a vailid function use ProtoValueAccessor::si32() instead"
		)
	}

	fn si64(&self) -> Result<i64, Self::Error> {
		panic!(
			"GraphQLValueAccessor::si64() is not a vailid function use ProtoValueAccessor::si64() instead"
		)
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

	fn as_value(&self) -> Self::Value {
		self.as_value().clone()
	}

	fn upload(&self) -> Result<Upload, Self::Error> {
		self.upload()
	}

	fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, Self::Error> {
		self.deserialize::<T>()
	}

	fn int8(&self) -> Result<i8, Self::Error> {
		todo!()
	}

	fn int16(&self) -> Result<i16, Self::Error> {
		todo!()
	}

	fn int32(&self) -> Result<i32, Self::Error> {
		todo!()
	}

	fn int64(&self) -> Result<i64, Self::Error> {
		todo!()
	}

	fn int128(&self) -> Result<i128, Self::Error> {
		todo!()
	}

	fn intsize(&self) -> Result<isize, Self::Error> {
		todo!()
	}

	fn uint8(&self) -> Result<u8, Self::Error> {
		todo!()
	}

	fn uint16(&self) -> Result<u16, Self::Error> {
		todo!()
	}

	fn uint32(&self) -> Result<u32, Self::Error> {
		todo!()
	}

	fn uint64(&self) -> Result<u64, Self::Error> {
		todo!()
	}

	fn uint128(&self) -> Result<u128, Self::Error> {
		todo!()
	}

	fn uintsize(&self) -> Result<usize, Self::Error> {
		todo!()
	}

	fn float32(&self) -> Result<ordered_float::OrderedFloat<f32>, Self::Error> {
		todo!()
	}

	fn float64(&self) -> Result<ordered_float::OrderedFloat<f64>, Self::Error> {
		todo!()
	}

	fn bool(&self) -> Result<bool, Self::Error> {
		todo!()
	}

	fn char(&self) -> Result<char, Self::Error> {
		todo!()
	}

	fn option(&self) -> Result<Option<Self>, Self::Error> {
		todo!()
	}

	fn variable(&self) -> Result<(Self, Self), Self::Error> {
		todo!()
	}
}

impl<'a> ObjectAccessorTrait<'a> for ObjectAccessor<'a> {
	type Value = Value;
	type Error = Error;
	type ValueAccessor = ValueAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLObjectAccessor"
	}

	// fn get_accessor(self) -> ObjectAccessors<'a> {
	// 	ObjectAccessors::graphql(self)
	// }

	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor> {
		self.get(name)
	}

	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error> {
		self.try_get(name)
	}

	fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Name, Self::ValueAccessor)> + 'a> {
		Box::new(self.iter())
	}

	fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Name> + 'a> {
		Box::new(self.keys())
	}

	fn values(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a> {
		Box::new(self.values())
	}

	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn as_index_map(&self) -> IndexMap<Name, Self::Value> {
		self.as_index_map().clone()
	}
}

impl<'a> ListAccessorTrait<'a> for ListAccessor<'a> {
	type Value = Value;
	type Error = Error;
	type ValueAccessor = ValueAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"GraphQLListAccessor"
	}

	// fn get_accessor<'b>(&'b self) -> ListAccessors<'b> {
	// 	todo!()
	// }

	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn to_iter(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a> {
		Box::new(self.iter())
	}

	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor> {
		self.get(idx)
	}

	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error> {
		self.try_get(idx)
	}

	fn as_slice(&self, start: usize, end: usize) -> Result<Self, Self::Error> {
		self.as_slice(start, end)
	}

	fn as_values_slice(&self) -> Vec<Self::Value> {
		self.as_values_slice().to_vec()
	}
}
