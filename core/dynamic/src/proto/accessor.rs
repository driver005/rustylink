use super::{Error, Result};
use crate::{prelude::Name, proto::Value};
use indexmap::IndexMap;
use std::borrow::Cow;

/// A value accessor
#[derive(Clone, Debug)]
pub struct ValueAccessor<'a>(&'a Value);

impl<'a> ValueAccessor<'a> {
	/// Returns `true` if the value is null, otherwise returns `false`
	#[inline]
	pub fn is_null(&self) -> bool {
		matches!(self.0, Value::Null)
	}

	/// Returns the boolean
	pub fn boolean(&self) -> Result<bool> {
		match self.0 {
			Value::Boolean(b) => Ok(*b),
			_ => Err(Error::new("internal: not a boolean")),
		}
	}

	/// Returns the enum name
	pub fn enum_name(&self) -> Result<&str> {
		match self.0 {
			Value::Enum((s, _)) => Ok(s.as_str()),
			_ => Err(Error::new("internal: not an enum name")),
		}
	}

	/// Returns the number as `i32`
	pub fn i32(&self) -> Result<i32> {
		if let Value::Int32(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an i32"))
	}

	/// Returns the number as `i64`
	pub fn i64(&self) -> Result<i64> {
		if let Value::Int64(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an i64"))
	}

	/// Returns the number as `u32`
	pub fn u32(&self) -> Result<u32> {
		if let Value::UInt32(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an u32"))
	}

	/// Returns the number as `u64`
	pub fn u64(&self) -> Result<u64> {
		if let Value::UInt64(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an u64"))
	}

	/// Returns the number as `i32`
	pub fn si32(&self) -> Result<i32> {
		if let Value::SInt32(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an signed i32"))
	}

	/// Returns the number as `u64`
	pub fn si64(&self) -> Result<i64> {
		if let Value::SInt64(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not an signed i64"))
	}

	/// Returns the number as `f32`
	pub fn f32(&self) -> Result<f32> {
		if let Value::Float(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not a float"))
	}

	/// Returns the number as `f64`
	pub fn f64(&self) -> Result<f64> {
		if let Value::Double(number) = self.0 {
			return Ok(*number);
		}
		Err(Error::new("internal: not a double"))
	}

	/// Returns the string value
	pub fn string(&self) -> Result<&'a str> {
		if let Value::String(value) = self.0 {
			Ok(value)
		} else {
			Err(Error::new("internal: not a string"))
		}
	}

	/// Returns the object accessor
	pub fn object(&self) -> Result<ObjectAccessor<'a>> {
		if let Value::Message(obj) = self.0 {
			Ok(ObjectAccessor(Cow::Borrowed(obj)))
		} else {
			Err(Error::new("internal: not an object"))
		}
	}

	/// Returns the list accessor
	pub fn list(&self) -> Result<ListAccessor<'a>> {
		if let Value::List(list) = self.0 {
			Ok(ListAccessor(list))
		} else {
			Err(Error::new("internal: not a list"))
		}
	}

	/// Returns a reference to the underlying `Value`
	#[inline]
	pub fn as_value(&self) -> &'a Value {
		self.0
	}
}

/// A object accessor
#[derive(Clone, Debug)]
pub struct ObjectAccessor<'a>(pub(crate) Cow<'a, IndexMap<Name, Value>>);

impl<'a> ObjectAccessor<'a> {
	/// Return a reference to the value stored for `key`, if it is present,
	/// else `None`.
	#[inline]
	pub fn get(&self, name: &str) -> Option<ValueAccessor<'_>> {
		self.0.get(name).map(ValueAccessor)
	}

	/// Like [`ObjectAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	pub fn try_get(&self, name: &str) -> Result<ValueAccessor<'_>> {
		self.0
			.get(name)
			.map(ValueAccessor)
			.ok_or_else(|| Error::new(format!("internal: key \"{}\" not found", name)))
	}

	/// Return an iterator over the key-value pairs of the object, in their
	/// order
	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = (&Name, ValueAccessor<'_>)> + '_ {
		self.0.iter().map(|(name, value)| (name, ValueAccessor(value)))
	}

	/// Return an iterator over the keys of the object, in their order
	#[inline]
	pub fn keys(&self) -> impl Iterator<Item = &Name> + '_ {
		self.0.keys()
	}

	/// Return an iterator over the values of the object, in their order
	#[inline]
	pub fn values(&self) -> impl Iterator<Item = ValueAccessor<'_>> + '_ {
		self.0.values().map(ValueAccessor)
	}

	/// Returns the number of elements in the object
	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns `true` if the object has no members
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns a reference to the underlying IndexMap
	#[inline]
	pub fn as_index_map(&'a self) -> &'a IndexMap<Name, Value> {
		&self.0
	}
}

/// A list accessor
#[derive(Clone, Debug)]
pub struct ListAccessor<'a>(pub(crate) &'a [Value]);

impl<'a> ListAccessor<'a> {
	/// Returns the number of elements in the list
	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns `true` if the list has a length of 0
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Returns an iterator over the list
	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = ValueAccessor<'_>> + '_ {
		self.0.iter().map(ValueAccessor)
	}

	/// Returns a reference to an element depending on the index
	#[inline]
	pub fn get(&self, idx: usize) -> Option<ValueAccessor<'_>> {
		self.0.get(idx).map(ValueAccessor)
	}

	/// Like [`ListAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	pub fn try_get(&self, idx: usize) -> Result<ValueAccessor<'_>> {
		self.get(idx).ok_or_else(|| Error::new(format!("internal: index \"{}\" not found", idx)))
	}

	/// Returns a new ListAccessor that represents a slice of the original
	#[inline]
	pub fn as_slice(&self, start: usize, end: usize) -> Result<ListAccessor<'a>> {
		if start <= end && end <= self.len() {
			Ok(ListAccessor(&self.0[start..end]))
		} else {
			Err(Error::new("internal: invalid slice indices"))
		}
	}

	/// Returns a reference to the underlying `&[Value]`
	#[inline]
	pub fn as_values_slice(&self) -> &'a [Value] {
		self.0
	}
}
