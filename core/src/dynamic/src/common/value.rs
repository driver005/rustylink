use binary::proto::EncoderLit;
use juniper::{LookAheadValue, ScalarValue, Value as JuniperValue};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap, fmt};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
	// Signed integers
	Int8(i8),
	Int16(i16),
	Int32(i32),
	Int64(i64),
	Int128(i128),
	Intsize(isize),

	// Unsigned integers
	UInt8(u8),
	UInt16(u16),
	UInt32(u32),
	UInt64(u64),
	UInt128(u128),
	UIntsize(usize),

	// Floating point
	Float32(OrderedFloat<f32>),
	Float64(OrderedFloat<f64>),

	// Boolean
	Bool(bool),

	// String
	Char(char),
	String(String),

	// Compound types
	List(Vec<Value>),
	Map(BTreeMap<Value, Value>),

	// Optional
	Option(Box<Option<Value>>),

	// Var
	Var(Box<(Value, Value)>),

	Null,
}

impl Value {
	pub fn type_name(&self) -> &'static str {
		match self {
			Value::Int8(_) => "i8",
			Value::Int16(_) => "i16",
			Value::Int32(_) => "i32",
			Value::Int64(_) => "i64",
			Value::Int128(_) => "i128",
			Value::Intsize(_) => "isize",
			Value::UInt8(_) => "u8",
			Value::UInt16(_) => "u16",
			Value::UInt32(_) => "u32",
			Value::UInt64(_) => "u64",
			Value::UInt128(_) => "u128",
			Value::UIntsize(_) => "usize",
			Value::Float32(_) => "f32",
			Value::Float64(_) => "f64",
			Value::Bool(_) => "bool",
			Value::Char(_) => "char",
			Value::String(_) => "string",
			Value::List(_) => "list",
			Value::Map(_) => "map",
			Value::Option(_) => "option",
			Value::Var(_) => "var",
			Value::Null => "null",
		}
	}

	pub fn is_repeated(&self) -> bool {
		matches!(self, Value::List(_))
	}

	pub fn as_var(&self) -> Option<(Value, Value)> {
		match self {
			Value::Var(data) => Some(*data.to_owned()),
			_ => None,
		}
	}
}

impl ScalarValue for Value {
	fn as_int(&self) -> Option<i32> {
		match self {
			Value::Int8(data) => Some((*data).into()),
			Value::Int16(data) => Some((*data).into()),
			Value::Int32(data) => Some(*data),
			Value::Int64(data) => (*data).try_into().ok(),
			Value::Int128(data) => (*data).try_into().ok(),
			Value::Intsize(data) => (*data).try_into().ok(),
			Value::UInt8(data) => Some((*data).into()),
			Value::UInt16(data) => Some((*data).into()),
			Value::UInt32(data) => (*data).try_into().ok(),
			Value::UInt64(data) => (*data).try_into().ok(),
			Value::UInt128(data) => (*data).try_into().ok(),
			_ => None,
		}
	}

	fn as_string(&self) -> Option<String> {
		match self {
			Value::String(data) => Some(data.to_owned()),
			Value::Char(data) => Some(data.to_string()),
			_ => None,
		}
	}

	fn into_string(self) -> Option<String> {
		match self {
			Value::String(data) => Some(data),
			Value::Char(data) => Some(data.to_string()),
			_ => None,
		}
	}

	fn as_str(&self) -> Option<&str> {
		match self {
			Value::String(data) => Some(data),
			//Value::Char(data) => Some(data.to_string().as_str()),
			_ => None,
		}
	}

	fn as_float(&self) -> Option<f64> {
		match self {
			Value::Float32(data) => Some((**data).into()),
			Value::Float64(data) => Some(**data),
			_ => None,
		}
	}

	fn as_bool(&self) -> Option<bool> {
		match self {
			Value::Bool(data) => Some(*data),
			_ => None,
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Int8(data) => write!(fmt, "i8: ({:?})", data),
			Value::Int16(data) => write!(fmt, "i16: ({:?})", data),
			Value::Int32(data) => write!(fmt, "i32: ({:?})", data),
			Value::Int64(data) => write!(fmt, "i64: ({:?})", data),
			Value::Int128(data) => write!(fmt, "i128: ({:?})", data),
			Value::Intsize(data) => write!(fmt, "isize: ({:?})", data),
			Value::UInt8(data) => write!(fmt, "u8: ({:?})", data),
			Value::UInt16(data) => write!(fmt, "u16: ({:?})", data),
			Value::UInt32(data) => write!(fmt, "u32: ({:?})", data),
			Value::UInt64(data) => write!(fmt, "u64: ({:?})", data),
			Value::UInt128(data) => write!(fmt, "u128: ({:?})", data),
			Value::UIntsize(data) => write!(fmt, "usize: ({:?})", data),
			Value::Float32(data) => write!(fmt, "f32: ({:?})", **data),
			Value::Float64(data) => write!(fmt, "f32: ({:?})", **data),
			Value::Bool(data) => write!(fmt, "bool: ({:?})", data),
			Value::Char(data) => write!(fmt, "char: ({:?})", data),
			Value::String(data) => write!(fmt, "String: ({:?})", data),
			Value::List(data) => write!(fmt, "list: ({:?})", data),
			Value::Map(data) => write!(fmt, "map: ({:?})", data),
			Value::Option(data) => write!(fmt, "option: ({:?})", data),
			Value::Var(data) => write!(fmt, "var: ({:?})", data),
			Value::Null => write!(fmt, "null"),
		}
	}
}

impl From<()> for Value {
	fn from((): ()) -> Self {
		Value::Null
	}
}

impl From<i8> for Value {
	#[inline]
	fn from(value: i8) -> Self {
		Value::Int8(value)
	}
}
impl From<i16> for Value {
	#[inline]
	fn from(value: i16) -> Self {
		Value::Int16(value)
	}
}
impl From<i32> for Value {
	#[inline]
	fn from(value: i32) -> Self {
		Value::Int32(value)
	}
}
impl From<i64> for Value {
	#[inline]
	fn from(value: i64) -> Self {
		Value::Int64(value)
	}
}
impl From<i128> for Value {
	#[inline]
	fn from(value: i128) -> Self {
		Value::Int128(value)
	}
}
impl From<isize> for Value {
	#[inline]
	fn from(value: isize) -> Self {
		Value::Intsize(value)
	}
}
impl From<u8> for Value {
	#[inline]
	fn from(value: u8) -> Self {
		Value::UInt8(value)
	}
}
impl From<u16> for Value {
	#[inline]
	fn from(value: u16) -> Self {
		Value::UInt16(value)
	}
}
impl From<u32> for Value {
	#[inline]
	fn from(value: u32) -> Self {
		Value::UInt32(value)
	}
}
impl From<u64> for Value {
	#[inline]
	fn from(value: u64) -> Self {
		Value::UInt64(value)
	}
}
impl From<u128> for Value {
	#[inline]
	fn from(value: u128) -> Self {
		Value::UInt128(value)
	}
}
impl From<usize> for Value {
	#[inline]
	fn from(value: usize) -> Self {
		Value::UIntsize(value)
	}
}

impl From<bool> for Value {
	#[inline]
	fn from(value: bool) -> Self {
		Value::Bool(value)
	}
}

impl From<f32> for Value {
	fn from(value: f32) -> Self {
		Value::Float32(OrderedFloat(value))
	}
}

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Value::Float64(OrderedFloat(value))
	}
}

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

impl From<(Value, Value)> for Value {
	fn from(value: (Value, Value)) -> Self {
		Value::Var(Box::new(value))
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

impl From<BTreeMap<Value, Value>> for Value {
	fn from(f: BTreeMap<Value, Value>) -> Self {
		Value::Map(f)
	}
}

// impl From<Value> for Option<()> {
// 	fn from(value: Value) -> Self {
// 		if let Value::Null = value {
// 			Some(())
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<i8> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Int8(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<i16> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Int16(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<i32> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Int32(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<i64> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Int64(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<i128> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Int128(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<isize> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Intsize(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<u8> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::UInt8(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<u16> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::UInt16(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<u32> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::UInt32(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<u64> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::UInt64(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<u128> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::UInt128(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }
// impl From<Value> for Option<usize> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::UIntsize(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<bool> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Bool(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<f32> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Float32(data) = value {
// 			Some(*data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<f64> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Float64(data) = value {
// 			Some(*data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<String> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::String(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<(Value, Value)> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Var(data) = value {
// 			Some(*data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<Vec<Value>> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::List(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }

// impl From<Value> for Option<BTreeMap<Value, Value>> {
// 	#[inline]
// 	fn from(value: Value) -> Self {
// 		if let Value::Map(data) = value {
// 			Some(data)
// 		} else {
// 			None
// 		}
// 	}
// }

impl<'a> From<EncoderLit<'a>> for Value {
	fn from(value: EncoderLit<'a>) -> Self {
		match value {
			EncoderLit::Bytes(data) => {
				Value::List(data.iter().map(|var| Value::UInt8(*var)).collect())
			}
			EncoderLit::Bool(data) => Value::Bool(*data),
			EncoderLit::BoolVec(data) => {
				Value::List(data.iter().map(|var| Value::Bool(*var)).collect())
			}
			EncoderLit::Int32(data) => Value::Int32(*data),
			EncoderLit::Int32Vec(data) => {
				Value::List(data.iter().map(|var| Value::Int32(*var)).collect())
			}
			EncoderLit::Int64(data) => Value::Int64(*data),
			EncoderLit::Int64Vec(data) => {
				Value::List(data.iter().map(|var| Value::Int64(*var)).collect())
			}
			EncoderLit::UInt32(data) => Value::UInt32(*data),
			EncoderLit::UInt32Vec(data) => {
				Value::List(data.iter().map(|var| Value::UInt32(*var)).collect())
			}
			EncoderLit::UInt64(data) => Value::UInt64(*data),
			EncoderLit::UInt64Vec(data) => {
				Value::List(data.iter().map(|var| Value::UInt64(*var)).collect())
			}
			EncoderLit::Float(data) => Value::Float32(OrderedFloat(*data)),
			EncoderLit::FloatVec(data) => {
				Value::List(data.iter().map(|var| Value::Float32(OrderedFloat(*var))).collect())
			}
			EncoderLit::Double(data) => Value::Float64(OrderedFloat(*data)),
			EncoderLit::DoubleVec(data) => {
				Value::List(data.iter().map(|var| Value::Float64(OrderedFloat(*var))).collect())
			}
			EncoderLit::SInt32(data) => Value::Int32(*data),
			EncoderLit::SInt32Vec(data) => {
				Value::List(data.iter().map(|var| Value::Int32(*var)).collect())
			}
			EncoderLit::SInt64(data) => Value::Int64(*data),
			EncoderLit::SInt64Vec(data) => {
				Value::List(data.iter().map(|var| Value::Int64(*var)).collect())
			}
			EncoderLit::Fixed32(data) => Value::UInt32(*data),
			EncoderLit::Fixed32Vec(data) => {
				Value::List(data.iter().map(|var| Value::UInt32(*var)).collect())
			}
			EncoderLit::Fixed64(data) => Value::UInt64(*data),
			EncoderLit::Fixed64Vec(data) => {
				Value::List(data.iter().map(|var| Value::UInt64(*var)).collect())
			}
			EncoderLit::SFixed32(data) => Value::Int32(*data),
			EncoderLit::SFixed32Vec(data) => {
				Value::List(data.iter().map(|var| Value::Int32(*var)).collect())
			}
			EncoderLit::SFixed64(data) => Value::Int64(*data),
			EncoderLit::SFixed64Vec(data) => {
				Value::List(data.iter().map(|var| Value::Int64(*var)).collect())
			}
		}
	}
}

impl From<JuniperValue<Value>> for Value {
	fn from(value: JuniperValue<Value>) -> Self {
		match value {
			JuniperValue::Null => Value::Null,
			JuniperValue::Scalar(s) => s,
			JuniperValue::List(values) => {
				Value::List(values.into_iter().map(Value::from).collect())
			}
			JuniperValue::Object(object) => Value::Map(
				object
					.into_iter()
					.map(|(key, value)| (Value::String(key.to_string()), Value::from(value)))
					.collect(),
			),
		}
	}
}

impl<'a> From<LookAheadValue<'a, Value>> for Value {
	fn from(value: LookAheadValue<'a, Value>) -> Self {
		match value {
			LookAheadValue::Null => Value::Null,
			LookAheadValue::Scalar(s) => s.clone(),
			LookAheadValue::Enum(e) => Value::String(e.to_string()),
			LookAheadValue::List(look_ahead_list) => {
				look_ahead_list.iter().map(|value| Value::from(value.item)).collect()
			}
			LookAheadValue::Object(look_ahead_object) => Value::Map(
				look_ahead_object
					.iter()
					.map(|(key, value)| {
						(Value::String(key.item.to_string()), Value::from(value.item))
					})
					.collect(),
			),
		}
	}
}

// impl From<JuniperValue> for Value {
// 	fn from(value: JuniperValue) -> Self {
// 		match value {
// 			JuniperValue::Null => Value::Null,
// 			JuniperValue::Number(number) => match number.as_i128() {
// 				Some(data) => Value::Int128(data),
// 				None => Value::Null,
// 			},
// 			JuniperValue::String(string) => Value::String(string),
// 			JuniperValue::Boolean(bool) => Value::Bool(bool),
// 			JuniperValue::Binary(bytes) => {
// 				let data = bytes.to_vec();
// 				Value::from(data)
// 			}
// 			JuniperValue::Enum(name) => Value::String(name.to_string()),
// 			JuniperValue::List(values) => {
// 				let mut data = vec![];
// 				for val in values.into_iter() {
// 					data.push(Value::from(val));
// 				}

// 				Value::List(data)
// 			}
// 			JuniperValue::Object(map) => {
// 				let mut data = BTreeMap::new();
// 				for val in map {
// 					data.insert(Value::String(val.0.to_string()), Value::from(val.1));
// 				}

// 				Value::Map(data)
// 			}
// 		}
// 	}
// }
