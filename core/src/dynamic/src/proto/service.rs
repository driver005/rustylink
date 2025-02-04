use super::{BoxFieldFuture, Error, Field, FieldValue, ObjectAccessor, Result, SchemaError, Value};
use crate::{prelude::Name, Context, ProtobufField, ProtobufKind, ProtobufMethod, Registry};
use binary::proto::Decoder;
use heck::ToUpperCamelCase;
use indexmap::IndexMap;
/// A Protobuf object type
#[derive(Debug)]
pub struct Service {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: IndexMap<String, Field>,
}

impl Service {
	/// Create a new Protobuf object type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
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

	/// Get an method of the object
	#[inline]
	pub fn get_field(&self, name: &str) -> Option<&Field> {
		self.fields.get(name)
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	#[inline]
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
		let mut methods = IndexMap::new();

		for method in self.fields.values() {
			let input_type = if method.arguments.len() == 1 {
				match method.arguments.first() {
					Some((_, field)) => field.ty.to_string(),
					None => "".to_string(),
				}
			} else {
				let mut fields = IndexMap::new();
				let name = format!("{}Input", method.name.to_string().to_upper_camel_case());

				for field in method.arguments.values() {
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
					name.clone(),
					ProtobufKind::Message {
						name: name.clone(),
						description: self.description.clone(),
						fields,
						oneof_groups: Default::default(),
						visible: None,
						rust_typename: None,
					},
				);
				name
			};
			methods.insert(
				method.name.to_string(),
				ProtobufMethod {
					name: method.name.to_string(),
					description: method.description.clone(),
					input_type,
					output_type: method.ty.to_string(),
					client_streaming: false,
					server_streaming: false,
				},
			);
		}

		registry.types.proto.insert(
			self.name.clone(),
			ProtobufKind::Service {
				name: self.name.clone(),
				description: self.description.clone(),
				methods,
				visible: None,
				rust_typename: None,
			},
		);

		Ok(())
	}
}
