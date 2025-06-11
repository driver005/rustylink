use super::{Error, Field, Result, descriptor};
use crate::{BoxFieldFutureByte, ContextBase, FieldValue, ObjectAccessor, Value};
use binary::proto::Decoder;
use bytes::{Buf, BufMut, BytesMut};
use prost::{
	DecodeError,
	encoding::{DecodeContext, WireType, decode_key, merge_loop},
};
use prost_types::{FileDescriptorProto, ServiceDescriptorProto, ServiceOptions};
use std::collections::BTreeMap;

/// A Protobuf object type
#[derive(Debug)]
pub struct Message {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: BTreeMap<String, Field>,
	pub(crate) oneof: bool,
	pub(crate) deprecated: bool,
}

impl Message {
	/// Create a new Protobuf object type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
			deprecated: false,
			oneof: false,
		}
	}

	impl_set_description!();

	/// Add an field to the object
	#[inline]
	pub fn field(mut self, field: Field) -> Self {
		assert!(
			!self.fields.contains_key(&field.name),
			"Field `{}` already exists",
			field.name.as_str()
		);
		self.fields.insert(field.name.clone(), field);
		self
	}

	/// Get an field of the object
	#[inline]
	pub fn get_field(&self, name: &str) -> Option<&Field> {
		self.fields.get(name)
	}

	/// Indicates this Message is a OneOf Message
	pub fn set_oneof(self) -> Self {
		Self {
			oneof: true,
			..self
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	/// Returns the type name
	#[inline]
	pub fn field_len(&self) -> usize {
		self.fields.len()
	}

	pub fn field_by_tag(&self, tag: u32) -> Option<&Field> {
		match self.fields.iter().find(|(_, field)| field.tag == tag) {
			Some((_, field)) => Some(field),
			None => None,
		}
	}

	pub fn field_by_name(&self, name: &str) -> Option<&Field> {
		match self.fields.iter().find(|(_, field)| field.type_name() == name) {
			Some((_, field)) => Some(field),
			None => None,
		}
	}

	pub(crate) fn decode<B>(
		&self,
		buf: &mut B,
		ctx: DecodeContext,
		arguments: &mut BTreeMap<Value, Value>,
	) -> Result<(), DecodeError>
	where
		B: Buf,
	{
		merge_loop(arguments, buf, ctx, |msg, buffer, ctx| {
			let (tag, wire_type) = decode_key(buffer)?;
			match self.field_by_tag(tag) {
				Some(field) => field.decode(buffer, ctx, wire_type, msg),
				None => Err(DecodeError::new(format!(
					"Message `{}` has no field with Tag `{}`",
					self.type_name(),
					tag,
				))),
			}
		})
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
		mut recursion: u32,
	) -> Vec<BoxFieldFutureByte<'a, BytesMut>> {
		self.fields
			.iter()
			.map(|(_, field)| field.collect(ctx, arguments, parent_value, recursion, false))
			.collect()
	}

	pub(crate) fn register(&self, file: &mut FileDescriptorProto, is_service: bool) {
		if is_service {
			let mut service = ServiceDescriptorProto::default();
			service.name = Some(self.name.clone());

			let mut methods = vec![];

			for method in self.fields.values() {
				if !method.arguments.is_empty() {
					let name = format!("Input{}", method.name);
					descriptor(
						file,
						Some(name.clone()),
						&method.arguments,
						false,
						method.deprecated,
					);
					methods.push(method.method_descriptor(Some(name)));
				} else {
					methods.push(method.method_descriptor(None));
				}
			}

			if !methods.is_empty() {
				service.method = methods;
			}

			service.options = Some(ServiceOptions {
				deprecated: Some(self.deprecated),
				..Default::default()
			});

			file.service.push(service);
		} else {
			descriptor(file, Some(self.name.clone()), &self.fields, self.oneof, self.deprecated);
		}
	}
}
