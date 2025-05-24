use super::{Error, Result};
use crate::{
	// interface::{ListAccessors, ObjectAccessors, ValueAccessors},
	ListAccessorTrait,
	ObjectAccessorTrait,
	ValueAccessorTrait,
};
use crate::{prelude::Name, proto::Value};
use async_graphql::Upload;
use indexmap::IndexMap;
use std::borrow::Cow;

/// A value accessor
#[derive(Clone, Debug)]
pub struct ValueAccessor<'a>(pub(crate) &'a Value);

impl<'a> ValueAccessorTrait<'a> for ValueAccessor<'a> {
	type Value = Value;
	type Error = Error;
	type ObjectAccessor = ObjectAccessor<'a>;
	type ListAccessor = ListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ProtoValueAccessor"
	}

	// fn get_accessor<'b>(&'b self) -> ValueAccessors<'b> {
	// 	todo!()
	// }

	/// Returns `true` if the value is null, otherwise returns `false`
	#[inline]
	fn is_null(&self) -> bool {
		matches!(self.0, Value::Null)
	}

	/// Returns the boolean
	fn boolean(&self) -> Result<bool, Self::Error> {
		match self.0 {
			Value::Boolean(b) => Ok(*b),
			_ => Err(Error::new("internal: not a boolean")),
		}
	}

	/// Returns the enum name
	fn enum_name(&self) -> Result<&str, Self::Error> {
		match self.0 {
			Value::Enum((s, _)) => Ok(s.as_str()),
			_ => Err(Error::new("internal: not an enum name")),
		}
	}

	/// Returns the number as `i32`
	fn i32(&self) -> Result<i32> {
		if let Value::Int32(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an i32"))
	}

	/// Returns the number as `i64`
	fn i64(&self) -> Result<i64, Self::Error> {
		if let Value::Int64(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an i64"))
	}

	/// Returns the number as `u32`
	fn u32(&self) -> Result<u32> {
		if let Value::UInt32(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an u32"))
	}

	/// Returns the number as `u64`
	fn u64(&self) -> Result<u64, Self::Error> {
		if let Value::UInt64(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an u64"))
	}

	/// Returns the number as `i32`
	fn si32(&self) -> Result<i32> {
		if let Value::SInt32(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an signed i32"))
	}

	/// Returns the number as `u64`
	fn si64(&self) -> Result<i64> {
		if let Value::SInt64(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an signed i64"))
	}

	/// Returns the number as `f32`
	fn f32(&self) -> Result<f32, Self::Error> {
		if let Value::Float(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not a float"))
	}

	/// Returns the number as `f64`
	fn f64(&self) -> Result<f64, Self::Error> {
		if let Value::Double(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not a double"))
	}

	/// Returns the string value
	fn string(&self) -> Result<&str, Self::Error> {
		if let Value::String(value) = self.0 {
			Ok(value)
		} else {
			Err(Error::new("internal: not a string"))
		}
	}

	/// Returns the object accessor
	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error> {
		if let Value::Message(obj) = self.0 {
			Ok(ObjectAccessor(Cow::Borrowed(obj)))
		} else {
			Err(Error::new("internal: not an object"))
		}
	}

	/// Returns the list accessor
	fn list(&self) -> Result<Self::ListAccessor, Self::Error> {
		if let Value::List(list) = self.0 {
			Ok(ListAccessor(list))
		} else {
			Err(Error::new("internal: not a list"))
		}
	}

	/// Returns a reference to the underlying `Value`
	#[inline]
	fn as_value(&self) -> Self::Value {
		self.0.clone()
	}

	fn upload(&self) -> Result<Upload, Self::Error> {
		panic!(
			"ProtoValueAccessor::upload() is not a vailid function use GraphQLValueAccessor::upload() instead"
		)
	}

	fn deserialize<T: serde::de::DeserializeOwned>(&self) -> std::result::Result<T, Self::Error> {
		panic!(
			"ProtoValueAccessor::deserialize() is not a vailid function use GraphQLValueAccessor::deserialize() instead"
		)
	}
}

/// A object accessor
#[derive(Clone, Debug)]
pub struct ObjectAccessor<'a>(pub(crate) Cow<'a, IndexMap<Name, Value>>);

impl<'a> ObjectAccessorTrait<'a> for ObjectAccessor<'a> {
	type Value = Value;
	type Error = Error;
	type ValueAccessor = ValueAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ProtoObjectAccessor"
	}

	// fn get_accessor(self) -> ObjectAccessors<'a> {
	// 	ObjectAccessors::proto(self)
	// }

	/// Return a reference to the value stored for `key`, if it is present,
	/// else `None`.
	#[inline]
	fn get(&'a self, name: &str) -> Option<Self::ValueAccessor> {
		self.0.get(name).map(ValueAccessor)
	}

	/// Like [`ObjectAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	fn try_get(&'a self, name: &str) -> Result<Self::ValueAccessor, Self::Error> {
		self.0
			.get(name)
			.map(ValueAccessor)
			.ok_or_else(|| Error::new(format!("internal: key \"{}\" not found", name)))
	}

	/// Return an iterator over the key-value pairs of the object, in their
	/// order
	#[inline]
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Name, Self::ValueAccessor)> + 'a> {
		Box::new(self.0.iter().map(|(name, value)| (name, ValueAccessor(value))))
	}

	/// Return an iterator over the keys of the object, in their order
	#[inline]
	fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Name> + 'a> {
		Box::new(self.0.keys())
	}

	/// Return an iterator over the values of the object, in their order
	#[inline]
	fn values(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a> {
		Box::new(self.0.values().map(ValueAccessor))
	}

	/// Returns the number of elements in the object
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns `true` if the object has no members
	#[must_use]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns a reference to the underlying IndexMap
	#[inline]
	fn as_index_map(&self) -> IndexMap<Name, Self::Value> {
		self.0.clone().into_owned()
	}
}

/// A list accessor
#[derive(Clone, Debug)]
pub struct ListAccessor<'a>(pub(crate) &'a [Value]);

impl<'a> ListAccessorTrait<'a> for ListAccessor<'a> {
	type Value = Value;
	type Error = Error;
	type ValueAccessor = ValueAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ProtoListAccessor"
	}

	// fn get_accessor<'b>(&'b self) -> ListAccessors<'b> {
	// 	todo!()
	// }

	/// Returns the number of elements in the list
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns `true` if the list has a length of 0
	#[inline]
	fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Returns an iterator over the list
	#[inline]
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a> {
		Box::new(self.0.iter().map(ValueAccessor))
	}

	/// Returns a reference to an element depending on the index
	#[inline]
	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor> {
		self.0.get(idx).map(ValueAccessor)
	}

	/// Like [`ListAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error> {
		self.get(idx).ok_or_else(|| Error::new(format!("internal: index \"{}\" not found", idx)))
	}

	/// Returns a new ListAccessor that represents a slice of the original
	#[inline]
	fn as_slice(&self, start: usize, end: usize) -> Result<Self, Self::Error> {
		if start <= end && end <= self.len() {
			Ok(ListAccessor(&self.0[start..end]))
		} else {
			Err(Error::new("internal: invalid slice indices"))
		}
	}

	/// Returns a reference to the underlying `&[Value]`
	#[inline]
	fn as_values_slice(&self) -> Vec<Self::Value> {
		self.0.to_vec()
	}
}
