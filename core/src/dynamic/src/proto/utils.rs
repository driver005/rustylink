use crate::{
	ObjectAccessor, SeaResult, SeaographyError, Value,
	proto::{TYPE_REGISTRY, Type},
};

use super::{Field, TypeRef};
use bytes::{Buf, BufMut, Bytes};
use juniper::ScalarValue;
use prost::{
	DecodeError, EncodeError,
	encoding::{DecodeContext, WireType},
};
use prost_types::{DescriptorProto, FileDescriptorProto, MessageOptions, OneofDescriptorProto};
use std::{borrow::Cow, collections::BTreeMap};
// CONSTS
pub const PCKNAME: &str = "apy";

pub(crate) fn descriptor(
	file: &mut FileDescriptorProto,
	name: Option<String>,
	fields: &BTreeMap<String, Field>,
	oneof: bool,
	deprecated: bool,
) {
	let mut descriptor = DescriptorProto::default();
	descriptor.name = name;

	let mut oneof_decl = vec![];

	for (_, field) in fields.iter() {
		if oneof {
			oneof_decl.push(OneofDescriptorProto {
				name: Some(field.name.clone()),
				..Default::default()
			});
		};

		descriptor.field.push(field.field_descriptor(oneof));
	}

	if !oneof_decl.is_empty() {
		descriptor.oneof_decl = oneof_decl;
	}

	descriptor.options = Some(MessageOptions {
		deprecated: Some(deprecated),
		..Default::default()
	});

	file.message_type.push(descriptor);
}

pub(crate) fn well_known_types() -> FileDescriptorProto {
	let mut file = FileDescriptorProto::default();
	file.name = Some("google/protobuf/wrappers.proto".to_string());
	file.package = Some("google.protobuf".to_string());

	for name in [
		"StringValue",
		"Int32Value",
		"Int64Value",
		"UInt32Value",
		"UInt64Value",
		"BoolValue",
		"FloatValue",
		"DoubleValue",
	] {
		descriptor(&mut file, Some(name.to_string()), &BTreeMap::new(), false, false);
	}

	file
}

pub(crate) fn from_bytes<B>(
	name: &str,
	type_ref: &str,
	is_repeated: bool,
	ctx: DecodeContext,
	buf: &mut B,
	wire_type: WireType,
	arguments: &mut BTreeMap<Value, Value>,
) -> Result<(), DecodeError>
where
	B: Buf,
{
	match (type_ref, is_repeated) {
		(TypeRef::DOUBLE, false) => {
			let mut value = 0.0;
			prost::encoding::double::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::DOUBLE, true) => {
			let mut value = Vec::new();
			prost::encoding::double::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::FLOAT, false) => {
			let mut value = 0.0;
			prost::encoding::float::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::FLOAT, true) => {
			let mut value = Vec::new();
			prost::encoding::float::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::INT32, false) => {
			let mut value = 0;
			prost::encoding::int32::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::INT32, true) => {
			let mut value = Vec::new();
			prost::encoding::int32::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::INT64, false) => {
			let mut value = 0;
			prost::encoding::int64::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::INT64, true) => {
			let mut value = Vec::new();
			prost::encoding::int64::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::UINT32, false) => {
			let mut value = 0;
			prost::encoding::uint32::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::UINT32, true) => {
			let mut value = Vec::new();
			prost::encoding::uint32::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::UINT64, false) => {
			let mut value = 0;
			prost::encoding::uint64::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::UINT64, true) => {
			let mut value = Vec::new();
			prost::encoding::uint64::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SINT32, false) => {
			let mut value = 0;
			prost::encoding::sint32::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SINT32, true) => {
			let mut value = Vec::new();
			prost::encoding::sint32::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SINT64, false) => {
			let mut value = 0;
			prost::encoding::sint64::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SINT64, true) => {
			let mut value = Vec::new();
			prost::encoding::sint64::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::FIXED32, false) => {
			let mut value = 0;
			prost::encoding::fixed32::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::FIXED32, true) => {
			let mut value = Vec::new();
			prost::encoding::fixed32::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::FIXED64, false) => {
			let mut value = 0;
			prost::encoding::fixed64::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::FIXED64, true) => {
			let mut value = Vec::new();
			prost::encoding::fixed64::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SFIXED32, false) => {
			let mut value = 0;
			prost::encoding::sfixed32::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SFIXED32, true) => {
			let mut value = Vec::new();
			prost::encoding::sfixed32::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SFIXED64, false) => {
			let mut value = 0;
			prost::encoding::sfixed64::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::SFIXED64, true) => {
			let mut value = Vec::new();
			prost::encoding::sfixed64::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::BOOL, false) => {
			let mut value = false;
			prost::encoding::bool::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::BOOL, true) => {
			let mut value = Vec::new();
			prost::encoding::bool::merge_repeated(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::STRING, false) => {
			let mut value = String::new();
			prost::encoding::string::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.into());
		}
		(TypeRef::STRING, true) => {
			let mut value = Vec::new();
			prost::encoding::string::merge_repeated(wire_type, &mut value, buf, ctx)?;
			let key = Value::from(name);
			if let Some(arg) = arguments.get_mut(&key) {
				match arg {
					Value::List(values) => {
						values.extend(value.into_iter().map(Into::into));
					}
					_ => panic!(),
				};
			} else {
				arguments.insert(key, value.into());
			}
		}
		(TypeRef::BYTES, false) => {
			let mut value = Bytes::new();
			prost::encoding::bytes::merge(wire_type, &mut value, buf, ctx)?;
			arguments.insert(Value::from(name), value.to_vec().into());
		}
		(TypeRef::BYTES, true) => {
			let mut value: Vec<Bytes> = Vec::new();
			prost::encoding::bytes::merge_repeated(wire_type, &mut value, buf, ctx)?;
			let key = Value::from(name);
			if let Some(arg) = arguments.get_mut(&key) {
				match arg {
					Value::List(values) => {
						values.extend(value.into_iter().map(|b| b.to_vec().into()))
					}
					_ => panic!(),
				};
			} else {
				arguments.insert(
					key,
					value.into_iter().map(|b| b.to_vec()).collect::<Vec<Vec<u8>>>().into(),
				);
			}
		}
		(name, _) => {
			return Err(DecodeError::new(format!(
				"Unsupported type or wire type combination: {}",
				name
			)));
		}
	};
	Ok(())
}

pub(crate) fn to_bytes<B>(buf: &mut B, value: &Value, tag: u32, type_ref: &str) -> SeaResult<usize>
where
	B: BufMut,
{
	match value {
		Value::Int32(int) => match type_ref {
			TypeRef::INT32 => {
				prost::encoding::int32::encode(tag, int, buf);
				Ok(prost::encoding::int32::encoded_len(tag, int))
			}
			TypeRef::SINT32 => {
				prost::encoding::sint32::encode(tag, int, buf);
				Ok(prost::encoding::sint32::encoded_len(tag, int))
			}
			TypeRef::SFIXED32 => {
				prost::encoding::sfixed32::encode(tag, int, buf);
				Ok(prost::encoding::sfixed32::encoded_len(tag, int))
			}
			_ => Err(SeaographyError::new("TypeRef needs to be of type INT32, SINT32 or SFIXED32")),
		},
		Value::Int64(int) => match type_ref {
			TypeRef::INT64 => {
				prost::encoding::int64::encode(tag, int, buf);
				Ok(prost::encoding::int64::encoded_len(tag, int))
			}
			TypeRef::SINT64 => {
				prost::encoding::sint64::encode(tag, int, buf);
				Ok(prost::encoding::sint64::encoded_len(tag, int))
			}
			TypeRef::SFIXED64 => {
				prost::encoding::sfixed64::encode(tag, int, buf);
				Ok(prost::encoding::sfixed64::encoded_len(tag, int))
			}
			_ => Err(SeaographyError::new("TypeRef needs to be of type INT64, SINT64 or SFIXED64")),
		},
		Value::UInt32(int) => match type_ref {
			TypeRef::UINT32 => {
				prost::encoding::uint32::encode(tag, int, buf);
				Ok(prost::encoding::uint32::encoded_len(tag, int))
			}
			TypeRef::FIXED32 => {
				prost::encoding::fixed32::encode(tag, int, buf);
				Ok(prost::encoding::fixed32::encoded_len(tag, int))
			}
			name => Err(SeaographyError::new(format!(
				"TypeRef is of type `{}` but needs to be of type UINT32 or FIXED32",
				name
			))),
		},
		Value::UInt64(int) => match type_ref {
			TypeRef::UINT64 => {
				prost::encoding::uint64::encode(tag, int, buf);
				Ok(prost::encoding::uint64::encoded_len(tag, int))
			}
			TypeRef::FIXED64 => {
				prost::encoding::fixed64::encode(tag, int, buf);
				Ok(prost::encoding::fixed64::encoded_len(tag, int))
			}
			name => Err(SeaographyError::new(format!(
				"TypeRef is of type `{}` but needs to be of type UINT64 or FIXED64",
				name
			))),
		},
		Value::Float32(ordered_float) => match type_ref {
			TypeRef::FLOAT => {
				prost::encoding::float::encode(tag, ordered_float, buf);
				Ok(prost::encoding::float::encoded_len(tag, ordered_float))
			}
			name => Err(SeaographyError::new(format!(
				"TypeRef is of type `{}` but needs to be of type FLOAT",
				name
			))),
		},
		Value::Float64(ordered_float) => match type_ref {
			TypeRef::DOUBLE => {
				prost::encoding::double::encode(tag, ordered_float, buf);
				Ok(prost::encoding::double::encoded_len(tag, ordered_float))
			}
			name => Err(SeaographyError::new(format!(
				"TypeRef is of type `{}` but needs to be of type DOUBLE",
				name
			))),
		},
		Value::Bool(bool) => match type_ref {
			TypeRef::BOOL => {
				prost::encoding::bool::encode(tag, bool, buf);
				Ok(prost::encoding::bool::encoded_len(tag, bool))
			}
			name => Err(SeaographyError::new(format!(
				"TypeRef is of type `{}` but needs to be of type BOOL",
				name
			))),
		},
		Value::String(string) => match type_ref {
			TypeRef::STRING => {
				prost::encoding::string::encode(tag, string, buf);
				Ok(prost::encoding::string::encoded_len(tag, string))
			}
			name => Err(SeaographyError::new(format!(
				"TypeRef is of type `{}` but needs to be of type STRING",
				name
			))),
		},
		Value::Null => Ok(0),
		_ => Err(SeaographyError::new(format!("Value type `{}` not supported", value.type_name()))),
	}
}
