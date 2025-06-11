use crate::{SeaResult, SeaographyError, Value};
use juniper::ScalarValue;
use ordered_float::OrderedFloat;
use std::{borrow::Cow, collections::BTreeMap};

/// A value accessor
#[derive(Clone, Debug)]
pub struct ValueAccessor<'a>(pub(crate) &'a Value);

impl<'a> ValueAccessor<'a> {
	pub fn type_name(&self) -> &'static str {
		"ValueAccessor"
	}

	/// Returns `true` if the value is null, otherwise returns `false`
	#[inline]
	pub fn is_null(&self) -> bool {
		matches!(self.0, Value::Null)
	}

	/// Returns a reference to the underlying `Value`
	#[inline]
	pub fn as_value(&self) -> Value {
		self.0.clone()
	}

	pub fn int8(&self) -> SeaResult<i8> {
		match self.0 {
			Value::Int8(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i8")),
		}
	}

	pub fn int16(&self) -> SeaResult<i16> {
		match self.0 {
			Value::Int16(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i16")),
		}
	}

	pub fn int32(&self) -> SeaResult<i32> {
		match self.0 {
			Value::Int32(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i32")),
		}
	}

	pub fn int64(&self) -> SeaResult<i64> {
		match self.0 {
			Value::Int64(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i64")),
		}
	}

	pub fn int128(&self) -> SeaResult<i128> {
		match self.0 {
			Value::Int128(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i128")),
		}
	}

	pub fn intsize(&self) -> SeaResult<isize> {
		match self.0 {
			Value::Intsize(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a isize")),
		}
	}

	pub fn uint8(&self) -> SeaResult<u8> {
		match self.0 {
			Value::UInt8(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u8")),
		}
	}

	pub fn uint16(&self) -> SeaResult<u16> {
		match self.0 {
			Value::UInt16(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u16")),
		}
	}

	pub fn uint32(&self) -> SeaResult<u32> {
		match self.0 {
			Value::UInt32(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u32")),
		}
	}

	pub fn uint64(&self) -> SeaResult<u64> {
		match self.0 {
			Value::UInt64(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u64")),
		}
	}

	pub fn uint128(&self) -> SeaResult<u128> {
		match self.0 {
			Value::UInt128(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u128")),
		}
	}

	pub fn uintsize(&self) -> SeaResult<usize> {
		match self.0 {
			Value::UIntsize(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a usize")),
		}
	}

	pub fn float32(&self) -> SeaResult<OrderedFloat<f32>> {
		match self.0 {
			Value::Float32(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a f32")),
		}
	}

	pub fn float64(&self) -> SeaResult<OrderedFloat<f64>> {
		match self.0 {
			Value::Float64(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a f64")),
		}
	}

	pub fn bool(&self) -> SeaResult<bool> {
		match self.0 {
			Value::Bool(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a bool")),
		}
	}

	pub fn char(&self) -> SeaResult<char> {
		match self.0 {
			Value::Char(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a char")),
		}
	}

	pub fn string(&self) -> SeaResult<&str> {
		match self.0 {
			Value::String(b) => Ok(b),
			_ => Err(SeaographyError::new("internal: not a String")),
		}
	}

	// Returns the object accessor
	pub fn object(&self) -> SeaResult<ObjectAccessor<'a>> {
		if let Value::Map(obj) = self.0 {
			Ok(ObjectAccessor(Cow::Borrowed(obj)))
		} else {
			Err(SeaographyError::new("internal: not an object"))
		}
	}

	/// Returns the list accessor
	pub fn list(&self) -> SeaResult<ListAccessor<'a>> {
		if let Value::List(list) = self.0 {
			Ok(ListAccessor(list))
		} else {
			Err(SeaographyError::new("internal: not a list"))
		}
	}

	pub fn option(&self) -> SeaResult<Option<Self>> {
		match self.0 {
			Value::Option(b) => match &**b {
				Some(v) => Ok(Some(ValueAccessor(v))),
				None => Ok(None),
			},
			_ => Err(SeaographyError::new("internal: not a option")),
		}
	}

	pub fn variable(&self) -> SeaResult<(Self, Self)> {
		match self.0 {
			Value::Var(b) => Ok((ValueAccessor(&b.0), ValueAccessor(&b.1))),
			_ => Err(SeaographyError::new("internal: not a variable")),
		}
	}

	pub fn enum_name(&self) -> SeaResult<&str> {
		match self.0 {
			Value::Map(b) => {
				for val in b.values() {
					if val.as_str().is_some() {
						return Ok(val.as_str().unwrap());
					}
				}
				Err(SeaographyError::new("internal: enum name not a string"))
			}
			value => {
				Err(SeaographyError::new(format!("internal: expected a enum, got {:?}", value)))
			}
		}
	}
}

/// A object accessor
#[derive(Clone, Debug)]
pub struct ObjectAccessor<'a>(pub(crate) Cow<'a, BTreeMap<Value, Value>>);

impl<'a> ObjectAccessor<'a> {
	pub fn insert(&mut self, key: Value, value: Value) {
		self.0.to_mut().insert(key, value);
	}

	pub fn type_name(&self) -> &'static str {
		"ObjectAccessor"
	}

	/// Return a reference to the value stored for `key`, if it is present,
	/// else `None`.
	#[inline]
	pub fn get<T: Into<Value>>(&'a self, name: T) -> Option<ValueAccessor<'a>> {
		self.0.get(&name.into()).map(ValueAccessor)
	}

	/// Like [`ObjectAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	pub fn try_get<T: Into<Value>>(&'a self, name: T) -> SeaResult<ValueAccessor<'a>> {
		self.0
			.get(&name.into())
			.map(ValueAccessor)
			.ok_or_else(|| SeaographyError::new(format!("internal: key not found")))
	}

	/// Return an iterator over the key-value pairs of the object, in their
	/// order
	#[inline]
	pub fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Value, ValueAccessor<'a>)> + 'a> {
		Box::new(self.0.iter().map(|(name, value)| (name, ValueAccessor(value))))
	}

	/// Return an iterator over the keys of the object, in their order
	#[inline]
	pub fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Value> + 'a> {
		Box::new(self.0.keys())
	}

	/// Return an iterator over the values of the object, in their order
	#[inline]
	pub fn values(&'a self) -> Box<dyn Iterator<Item = ValueAccessor<'a>> + 'a> {
		Box::new(self.0.values().map(ValueAccessor))
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
	pub fn as_index_map(&self) -> BTreeMap<Value, Value> {
		self.0.clone().into_owned()
	}
}

/// A list accessor
#[derive(Clone, Debug)]
pub struct ListAccessor<'a>(pub(crate) &'a [Value]);

impl<'a> ListAccessor<'a> {
	pub fn type_name(&self) -> &'static str {
		"ListAccessor"
	}

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
	pub fn to_iter(&'a self) -> Box<dyn Iterator<Item = ValueAccessor<'a>> + 'a> {
		Box::new(self.0.iter().map(ValueAccessor))
	}

	/// Returns a reference to an element depending on the index
	#[inline]
	pub fn get(&'a self, idx: usize) -> Option<ValueAccessor<'a>> {
		self.0.get(idx).map(ValueAccessor)
	}

	/// Like [`ListAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	pub fn try_get(&'a self, idx: usize) -> SeaResult<ValueAccessor<'a>> {
		self.get(idx)
			.ok_or_else(|| SeaographyError::new(format!("internal: index \"{}\" not found", idx)))
	}

	/// Returns a new ListAccessor that represents a slice of the original
	#[inline]
	pub fn as_slice(&self, start: usize, end: usize) -> SeaResult<Self> {
		if start <= end && end <= self.len() {
			Ok(ListAccessor(&self.0[start..end]))
		} else {
			Err(SeaographyError::new("internal: invalid slice indices"))
		}
	}

	/// Returns a reference to the underlying `&[Value]`
	#[inline]
	pub fn as_values_slice(&self) -> Vec<Value> {
		self.0.to_vec()
	}
}
