use crate::{
	// interface::{ListAccessors, ObjectAccessors, ValueAccessors},
	ListAccessorTrait,
	ObjectAccessorTrait,
	SeaographyError,
	Value,
	ValueAccessorTrait,
};
use juniper::ScalarValue;
use ordered_float::OrderedFloat;
use std::{borrow::Cow, collections::BTreeMap};

/// A value accessor
#[derive(Clone, Debug)]
pub struct ValueAccessor<'a>(pub(crate) &'a Value);

impl<'a> ValueAccessorTrait<'a> for ValueAccessor<'a> {
	type Value = Value;
	type Error = SeaographyError;
	type ObjectAccessor = ObjectAccessor<'a>;
	type ListAccessor = ListAccessor<'a>;

	fn type_name(&self) -> &'static str {
		"ValueAccessor"
	}

	/// Returns `true` if the value is null, otherwise returns `false`
	#[inline]
	fn is_null(&self) -> bool {
		matches!(self.0, Value::Null)
	}

	/// Returns a reference to the underlying `Value`
	#[inline]
	fn as_value(&self) -> Self::Value {
		self.0.clone()
	}

	fn int8(&self) -> Result<i8, Self::Error> {
		match self.0 {
			Value::Int8(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i8")),
		}
	}

	fn int16(&self) -> Result<i16, Self::Error> {
		match self.0 {
			Value::Int16(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i16")),
		}
	}

	fn int32(&self) -> Result<i32, Self::Error> {
		match self.0 {
			Value::Int32(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i32")),
		}
	}

	fn int64(&self) -> Result<i64, Self::Error> {
		match self.0 {
			Value::Int64(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i64")),
		}
	}

	fn int128(&self) -> Result<i128, Self::Error> {
		match self.0 {
			Value::Int128(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a i128")),
		}
	}

	fn intsize(&self) -> Result<isize, Self::Error> {
		match self.0 {
			Value::Intsize(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a isize")),
		}
	}

	fn uint8(&self) -> Result<u8, Self::Error> {
		match self.0 {
			Value::UInt8(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u8")),
		}
	}

	fn uint16(&self) -> Result<u16, Self::Error> {
		match self.0 {
			Value::UInt16(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u16")),
		}
	}

	fn uint32(&self) -> Result<u32, Self::Error> {
		match self.0 {
			Value::UInt32(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u32")),
		}
	}

	fn uint64(&self) -> Result<u64, Self::Error> {
		match self.0 {
			Value::UInt64(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u64")),
		}
	}

	fn uint128(&self) -> Result<u128, Self::Error> {
		match self.0 {
			Value::UInt128(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a u128")),
		}
	}

	fn uintsize(&self) -> Result<usize, Self::Error> {
		match self.0 {
			Value::UIntsize(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a usize")),
		}
	}

	fn float32(&self) -> Result<OrderedFloat<f32>, Self::Error> {
		match self.0 {
			Value::Float32(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a f32")),
		}
	}

	fn float64(&self) -> Result<OrderedFloat<f64>, Self::Error> {
		match self.0 {
			Value::Float64(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a f64")),
		}
	}

	fn bool(&self) -> Result<bool, Self::Error> {
		match self.0 {
			Value::Bool(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a bool")),
		}
	}

	fn char(&self) -> Result<char, Self::Error> {
		match self.0 {
			Value::Char(b) => Ok(*b),
			_ => Err(SeaographyError::new("internal: not a char")),
		}
	}

	fn string(&self) -> Result<&str, Self::Error> {
		match self.0 {
			Value::String(b) => Ok(b),
			_ => Err(SeaographyError::new("internal: not a String")),
		}
	}

	// Returns the object accessor
	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error> {
		if let Value::Map(obj) = self.0 {
			Ok(ObjectAccessor(Cow::Borrowed(obj)))
		} else {
			Err(SeaographyError::new("internal: not an object"))
		}
	}

	/// Returns the list accessor
	fn list(&self) -> Result<Self::ListAccessor, Self::Error> {
		if let Value::List(list) = self.0 {
			Ok(ListAccessor(list))
		} else {
			Err(SeaographyError::new("internal: not a list"))
		}
	}

	fn option(&self) -> Result<Option<Self>, Self::Error> {
		match self.0 {
			Value::Option(b) => match &**b {
				Some(v) => Ok(Some(ValueAccessor(v))),
				None => Ok(None),
			},
			_ => Err(SeaographyError::new("internal: not a option")),
		}
	}

	fn variable(&self) -> Result<(Self, Self), Self::Error> {
		match self.0 {
			Value::Var(b) => Ok((ValueAccessor(&b.0), ValueAccessor(&b.1))),
			_ => Err(SeaographyError::new("internal: not a variable")),
		}
	}

	fn enum_name(&self) -> Result<&str, Self::Error> {
		match self.0 {
			Value::Var(b) => match b.0.as_str() {
				Some(v) => Ok(v),
				None => Err(SeaographyError::new("internal: var not an enum")),
			},
			_ => Err(SeaographyError::new("internal: not a enum")),
		}
	}
}

/// A object accessor
#[derive(Clone, Debug)]
pub struct ObjectAccessor<'a>(pub(crate) Cow<'a, BTreeMap<Value, Value>>);

impl<'a> ObjectAccessorTrait<'a> for ObjectAccessor<'a> {
	type Value = Value;
	type Error = SeaographyError;
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
	fn get<T: Into<Self::Value>>(&'a self, name: T) -> Option<Self::ValueAccessor> {
		self.0.get(&name.into()).map(ValueAccessor)
	}

	/// Like [`ObjectAccessor::get`], returns `Err` if the index does not exist
	#[inline]
	fn try_get<T: Into<Self::Value>>(
		&'a self,
		name: T,
	) -> Result<Self::ValueAccessor, Self::Error> {
		self.0
			.get(&name.into())
			.map(ValueAccessor)
			.ok_or_else(|| SeaographyError::new(format!("internal: key not found")))
	}

	/// Return an iterator over the key-value pairs of the object, in their
	/// order
	#[inline]
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Self::Value, Self::ValueAccessor)> + 'a> {
		Box::new(self.0.iter().map(|(name, value)| (name, ValueAccessor(value))))
	}

	/// Return an iterator over the keys of the object, in their order
	#[inline]
	fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Self::Value> + 'a> {
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
	fn as_index_map(&self) -> BTreeMap<Self::Value, Self::Value> {
		self.0.clone().into_owned()
	}
}

/// A list accessor
#[derive(Clone, Debug)]
pub struct ListAccessor<'a>(pub(crate) &'a [Value]);

impl<'a> ListAccessorTrait<'a> for ListAccessor<'a> {
	type Value = Value;
	type Error = SeaographyError;
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
		self.get(idx)
			.ok_or_else(|| SeaographyError::new(format!("internal: index \"{}\" not found", idx)))
	}

	/// Returns a new ListAccessor that represents a slice of the original
	#[inline]
	fn as_slice(&self, start: usize, end: usize) -> Result<Self, Self::Error> {
		if start <= end && end <= self.len() {
			Ok(ListAccessor(&self.0[start..end]))
		} else {
			Err(SeaographyError::new("internal: invalid slice indices"))
		}
	}

	/// Returns a reference to the underlying `&[Value]`
	#[inline]
	fn as_values_slice(&self) -> Vec<Self::Value> {
		self.0.to_vec()
	}
}
