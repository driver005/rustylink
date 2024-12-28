use super::{BoxFieldFuture, Error, Field, FieldValue, ObjectAccessor, Result, Value};
use crate::{prelude::Name, Context, ProtobufField, ProtobufKind, Registry, SchemaError};
use binary::proto::Decoder;
use indexmap::IndexMap;

/// A Protobuf object type
#[derive(Debug)]
pub struct Message {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: IndexMap<String, Field>,
	pub(crate) oneof: bool,
	deprecated: bool,
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
			// arguments: Default::default(),
		}
	}

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
	pub fn oneof(self) -> Self {
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

	pub(crate) fn decode(
		&self,
		decoder: &mut Decoder,
		mut buf: Vec<u8>,
	) -> Result<IndexMap<Name, Value>> {
		let mut arguments = IndexMap::new();
		let mut dst = vec![];
		decoder.decode(&mut buf, &mut dst)?;

		for (tag, _, byt) in dst.drain(..) {
			match self.field_by_tag(tag) {
				Some(field) => {
					field.decode(decoder, byt, tag, &mut arguments)?;
				}
				None => {
					return Err(Error::new(format!(
						"Message `{}` has no field with Tag `{}`",
						self.type_name(),
						tag,
					)))
				}
			}
		}

		Ok(arguments)
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a Context<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Vec<BoxFieldFuture<'a>> {
		self.fields.iter().map(|(_, field)| field.collect(ctx, arguments, parent_value)).collect()
	}

	pub(crate) fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
		let mut fields = IndexMap::new();

		for field in self.fields.values() {
			fields.insert(
				field.name.to_string(),
				ProtobufField {
					name: field.name.to_string(),
					description: field.description.clone(),
					field_type: field.ty.to_proto(),
					tag: field.tag,
					label: None,
				},
			);
		}

		registry.types.proto.insert(
			self.name.to_string(),
			ProtobufKind::Message {
				name: self.name.to_string(),
				description: self.description.clone(),
				fields,
				oneof_groups: Default::default(),
				visible: None,
				rust_typename: None,
			},
		);

		Ok(())
	}
}
