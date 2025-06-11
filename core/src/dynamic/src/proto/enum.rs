use super::{Error, Result, TypeRef, from_bytes, to_bytes};
use crate::{
	BoxFieldFutureByte, EnumItemTrait, EnumTrait, ObjectAccessor, SeaResult, SeaographyError,
	Value, ValueAccessor,
};
use binary::proto::{DecoderLit, Encoder};
use bytes::{Buf, BufMut};
use futures::FutureExt;
use juniper::ScalarValue;
use prost::{
	DecodeError,
	encoding::{DecodeContext, WireType},
};
use prost_types::{
	EnumDescriptorProto, EnumOptions, EnumValueDescriptorProto, EnumValueOptions,
	FileDescriptorProto,
};
use std::{collections::BTreeMap, ops::Add};

/// A GraphQL enum item
#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem {
	pub(crate) name: String,
	pub(crate) tag: Option<u32>,
	pub(crate) description: Option<String>,
	pub(crate) deprecation: bool,
}

impl<T: Into<String>> From<(T, u32)> for EnumItem {
	#[inline]
	fn from((value, tag): (T, u32)) -> Self {
		EnumItem {
			name: value.into(),
			tag: Some(tag),
			description: None,
			deprecation: false,
		}
	}
}

impl EnumItem {
	/// Create a new EnumItem
	#[inline]
	pub fn new<N>(name: N) -> Self
	where
		N: Into<String>,
	{
		Self {
			name: name.into(),
			tag: None,
			description: None,
			deprecation: false,
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	pub fn register(&self) -> EnumValueDescriptorProto {
		let mut enum_value = EnumValueDescriptorProto::default();
		enum_value.name = Some(self.name.clone());
		enum_value.number = Some(self.tag.expect("please ensure EnumItem is registred to a Enum: therfore use Enum::item or Enum::items to register it") as i32);
		enum_value.options = Some(EnumValueOptions {
			deprecated: Some(self.deprecation),
			..Default::default()
		});

		enum_value
	}

	// impl_set_description!();
}

impl EnumItemTrait for EnumItem {
	/// Create a new EnumItem
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		Self::new(name)
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> &str {
		self.type_name()
	}
}

/// A GraphQL enum type
#[derive(Debug)]
pub struct Enum {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) enum_values: BTreeMap<String, EnumItem>,
}

impl Enum {
	/// Create a Proto enum type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			enum_values: Default::default(),
		}
	}

	/// Add an item
	#[inline]
	pub fn item(mut self, item: impl Into<EnumItem>) -> Self {
		let mut field = item.into();
		field.tag = Some(self.enum_values.len().add(1) as u32);
		self.enum_values.insert(field.name.clone(), field);
		self
	}

	/// Add items
	pub fn items(mut self, items: impl IntoIterator<Item = impl Into<EnumItem>>) -> Self {
		for field in items {
			let mut field = field.into();
			field.tag = Some(self.enum_values.len().add(1) as u32);
			self.enum_values.insert(field.name.clone(), field);
		}
		self
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	/// Get an item of the object
	#[inline]
	pub fn get_item(&self, name: &str) -> Option<&EnumItem> {
		self.enum_values.get(name)
	}

	/// Get an item of the object
	#[inline]
	pub fn get_item_by_tag(&self, tag: u32) -> Option<&EnumItem> {
		self.enum_values.values().find(|item| item.tag == Some(tag))
	}

	pub(crate) fn to_value<B>(&self, buf: &mut B, value: &Value, tag: u32) -> SeaResult<usize>
	where
		B: BufMut,
	{
		// if self.inaccessible {
		// 	return Err(SeaographyError::new(format!(
		// 		"enum `{}` is inaccessible",
		// 		self.type_name()
		// 	)));
		// }
		if let Some(name) = value.as_string() {
			if let Some(item) = self.enum_values.get(&name) {
				let val = item.tag.expect("please ensure EnumItem is registred to a Enum: therfore use Enum::item or Enum::items to register it");
				return to_bytes(buf, &Value::Int32(val as i32), tag, TypeRef::INT32);
			}
		}

		Err(SeaographyError::new(format!("enum `{}` has no value of `{}`", self.name, value)))
	}

	pub(crate) fn collect<'a, B>(&'a self) -> BoxFieldFutureByte<'a, B>
	where
		B: BufMut,
	{
		async move {
			return Err(SeaographyError::new(format!(
				"invalid FieldValue for enum `{}`, expected `FieldValue::Value`",
				self.type_name()
			)));
		}
		.boxed()
	}

	pub fn encode(&self, buf: &mut Vec<u8>, value: Value, tag: u32) -> Result<usize> {
		let encoder = Encoder::default();

		match value {
			Value::Var(var) => {
				let val = ValueAccessor(&var.0);
				Ok(encoder.encode((&tag, &val.int32()?), buf)?)
			}
			_ => Ok(0),
		}
	}

	pub fn bytes<B>(
		&self,
		buf: &mut B,
		ctx: DecodeContext,
		wire_type: WireType,
		is_repeated: bool,
		arguments: &mut BTreeMap<Value, Value>,
	) -> Result<(), DecodeError>
	where
		B: Buf,
	{
		let mut argument = BTreeMap::new();
		from_bytes(
			self.type_name(),
			TypeRef::INT32,
			is_repeated,
			ctx,
			buf,
			wire_type,
			&mut argument,
		)?;

		if let Some((key, value)) = argument.first_key_value() {
			match value {
				Value::Int32(number) => {
					if let Some(item) = self.get_item_by_tag(*number as u32) {
						println!("item: {:?}", item.type_name());
						arguments.insert(key.to_owned(), Value::from(item.type_name()));
					}
				}
				_ => unreachable!(),
			}
		}

		Ok(())
	}

	// pub(crate) async fn resolve(&self, value: &FieldValue<'_>) -> Result<Option<Value>> {
	// 	match value.as_value() {
	// 		Some(v) => match v {
	// 			Value::Enum((_, number)) => {
	// 				if !self.fields.contains_key(self.type_name()) {
	// 					return Err(Error::new(format!(
	// 						"internal: invalid item for enum \"{}\"",
	// 						self.name
	// 					)));
	// 				}
	// 				Ok(Some(Value::Enum((Name::new(self.type_name()), *number))))
	// 			}
	// 			Value::Int32(number) => {
	// 				if !self.fields.contains_key(self.type_name()) {
	// 					return Err(Error::new(format!(
	// 						"internal: invalid item for enum \"{}\"",
	// 						self.name
	// 					)));
	// 				}
	// 				Ok(Some(Value::Enum((Name::new(self.type_name()), *number))))
	// 			}
	// 			_ => Err(Error::new(format!(
	// 				"internal: invalid item for enum \"{}\"",
	// 				self.type_name()
	// 			))),
	// 		},
	// 		None => {
	// 			Err(Error::new(format!("internal: invalid item for enum \"{}\"", self.type_name())))
	// 		}
	// 	}
	// }

	pub(crate) fn register(&self, file: &mut FileDescriptorProto) {
		let mut enum_descriptor = EnumDescriptorProto::default();
		enum_descriptor.name = Some(self.name.clone());

		for item in self.enum_values.values() {
			enum_descriptor.value.push(item.register());
		}

		enum_descriptor.options = Some(EnumOptions {
			deprecated: Some(false),
			..Default::default()
		});

		file.enum_type.push(enum_descriptor);
	}
}

impl EnumTrait for Enum {
	type Item = EnumItem;

	/// Create a GraphqL enum type
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		Enum::new(name)
	}

	/// Add an item
	#[inline]
	fn item(self, item: impl Into<Self::Item>) -> Self {
		self.item(item)
	}

	/// Add items
	fn items(self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self {
		self.items(fields)
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> &str {
		self.type_name()
	}
}
