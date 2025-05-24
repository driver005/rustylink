use crate::{ValueTrait, prelude::Name};
use indexmap::IndexMap;
use std::{any::Any, borrow::Cow};

use super::FieldValue;

#[derive(Debug, Clone)]
pub enum Value {
	/// `null`.
	Null,
	/// A 32-bit integer.
	Int32(i32),
	/// A 64-bit integer.
	Int64(i64),
	/// An unsigned 32-bit integer.
	UInt32(u32),
	/// An unsigned 64-bit integer.
	UInt64(u64),
	/// A signed 32-bit integer.
	SInt32(i32),
	/// A signed 64-bit integer.
	SInt64(i64),
	/// A 32-bit floating-point number.
	Float(f32),
	/// A 64-bit floating-point number.
	Double(f64),
	/// A boolean.
	Boolean(bool),
	/// A string.
	String(String),
	/// A binary.
	Binary(Vec<u8>),
	/// An enum value represented as a string.
	Enum((Name, i32)),
	/// A list of values.
	List(Vec<Value>),
	/// An object, which is a map of field names to values.
	Message(IndexMap<Name, Value>),
}

impl<'a> ValueTrait<'a> for Value {
	type FieldValue = FieldValue<'a>;
	fn new<T>(val: T) -> Self
	where
		T: From<T>,
		Value: From<T>,
	{
		Self::from(val)
	}

	fn into_field_value(self) -> Self::FieldValue {
		FieldValue::from(self)
	}
}

impl From<()> for Value {
	fn from((): ()) -> Self {
		Value::Null
	}
}

impl From<i8> for Value {
	#[inline]
	fn from(f: i8) -> Self {
		From::from(f as i32)
	}
}

impl From<i16> for Value {
	#[inline]
	fn from(f: i16) -> Self {
		From::from(f as i32)
	}
}

impl From<i32> for Value {
	fn from(value: i32) -> Self {
		Value::Int32(value)
	}
}

impl From<isize> for Value {
	#[inline]
	fn from(f: isize) -> Self {
		From::from(f as i64)
	}
}

impl From<i64> for Value {
	fn from(value: i64) -> Self {
		Value::Int64(value)
	}
}

impl From<u8> for Value {
	#[inline]
	fn from(f: u8) -> Self {
		From::from(f as u32)
	}
}

impl From<u16> for Value {
	#[inline]
	fn from(f: u16) -> Self {
		From::from(f as u32)
	}
}

impl From<u32> for Value {
	fn from(value: u32) -> Self {
		Value::UInt32(value)
	}
}

impl From<usize> for Value {
	#[inline]
	fn from(f: usize) -> Self {
		From::from(f as u64)
	}
}

impl From<u64> for Value {
	fn from(value: u64) -> Self {
		Value::UInt64(value)
	}
}

impl From<f32> for Value {
	fn from(value: f32) -> Self {
		Value::Float(value)
	}
}

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Value::Double(value)
	}
}

impl From<bool> for Value {
	#[inline]
	fn from(value: bool) -> Self {
		Value::Boolean(value)
	}
}

// impl From<Vec<u8>> for Value {
// 	fn from(value: Vec<u8>) -> Self {
// 		Value::Binary(value)
// 	}
// }

// impl From<&[u8]> for Value {
// 	fn from(value: &[u8]) -> Self {
// 		Value::Binary(value.to_vec())
// 	}
// }

impl From<String> for Value {
	#[inline]
	fn from(value: String) -> Self {
		Value::String(value)
	}
}

impl From<&String> for Value {
	#[inline]
	fn from(value: &String) -> Self {
		Value::String(value.clone())
	}
}

impl From<(Name, i32)> for Value {
	fn from(value: (Name, i32)) -> Self {
		Value::Enum(value)
	}
}

impl<'a> From<&'a str> for Value {
	#[inline]
	fn from(value: &'a str) -> Self {
		Value::String(value.into())
	}
}

impl<'a> From<Cow<'a, str>> for Value {
	#[inline]
	fn from(f: Cow<'a, str>) -> Self {
		Value::String(f.into_owned())
	}
}

impl<T: Into<Value>> FromIterator<T> for Value {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Value::List(iter.into_iter().map(Into::into).collect())
	}
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
	fn from(f: &'a [T]) -> Self {
		Value::List(f.iter().cloned().map(Into::into).collect())
	}
}

impl<T: Into<Value>> From<Vec<T>> for Value {
	fn from(f: Vec<T>) -> Self {
		Value::List(f.into_iter().map(Into::into).collect())
	}
}

impl From<IndexMap<Name, Value>> for Value {
	fn from(f: IndexMap<Name, Value>) -> Self {
		Value::Message(f)
	}
}
