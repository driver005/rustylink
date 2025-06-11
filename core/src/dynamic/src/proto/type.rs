use super::{Enum, Field, Message, TypeRef, from_bytes};
use crate::{
	BoxFieldFutureByte, ContextBase, FieldValue, ObjectAccessor, SeaResult, SeaographyError, Value,
};
use bytes::{Buf, BufMut, Bytes, BytesMut, buf};
use once_cell::sync::Lazy;
use prost::DecodeError;
use prost::encoding::{DecodeContext, WireType, decode_key, merge_loop};
use prost_types::FileDescriptorProto;
use prost_types::field_descriptor_proto::Type as FieldType;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub static TYPE_REGISTRY: Lazy<TypeRegistry> = Lazy::new(TypeRegistry::new);

#[derive(Debug)]
pub struct TypeRegistry {
	types: RwLock<BTreeMap<String, Arc<Type>>>,
}

impl TypeRegistry {
	pub fn new() -> Self {
		Self {
			types: RwLock::new(BTreeMap::new()),
		}
	}
	pub fn clear(&self) {
		let mut types = match self.types.write() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		types.clear();
	}

	pub fn add(&self, name: String, ty: Type) {
		let mut types = match self.types.write() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		if types.insert(name.clone(), Arc::new(ty)).is_some() {
			panic!("Type `{}` could not be added.", name)
		}
	}

	pub fn get(&self, name: &str) -> Option<Arc<Type>> {
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};

		types.get(name).map(|ty| ty.clone())
	}

	pub fn get_filtered<F>(&self, mut f: F) -> BTreeMap<String, Arc<Type>>
	where
		F: FnMut(&str, &Arc<Type>) -> bool,
	{
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		types
			.iter()
			.filter(|(name, ty)| f(name, ty))
			.map(|(name, ty)| (name.clone(), ty.clone()))
			.collect()
	}

	pub fn all(&self) -> BTreeMap<String, Arc<Type>> {
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};

		types.iter().map(|(name, ty)| (name.clone(), ty.clone())).collect()
	}

	pub fn print(&self) {
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		println!("types: {:?}", types)
	}
}

/// A GraphQL type
#[derive(Debug)]
pub enum Type {
	/// Message
	Message(Message),
	/// Enum
	Enum(Enum),
}

impl Type {
	pub(crate) fn type_name(&self) -> &str {
		match self {
			Type::Message(m) => m.type_name(),
			Type::Enum(e) => e.type_name(),
		}
	}

	#[inline]
	pub(crate) fn as_message(&self) -> Option<&Message> {
		if let Type::Message(obj) = self {
			Some(obj)
		} else {
			None
		}
	}

	#[inline]
	pub(crate) fn as_enum(&self) -> Option<&Enum> {
		if let Type::Enum(obj) = self {
			Some(obj)
		} else {
			None
		}
	}

	#[inline]
	pub(crate) fn field_type(&self) -> FieldType {
		match self {
			Type::Message(_) => FieldType::Message,
			Type::Enum(_) => FieldType::Enum,
		}
	}

	pub fn field_by_name(&self, name: &str) -> Option<&str> {
		match self {
			Type::Message(message) => {
				match message.fields.iter().find(|(_, field)| field.type_name() == name) {
					Some((_, field)) => Some(field.type_name()),
					None => None,
				}
			}
			Type::Enum(en) => {
				match en.enum_values.iter().find(|(_, field)| field.type_name() == name) {
					Some((_, field)) => Some(field.type_name()),
					None => None,
				}
			}
		}
	}

	pub(crate) fn decode<B>(
		&self,
		name: &str,
		tag: u32,
		buf: &mut B,
		wire_type: WireType,
		ctx: DecodeContext,
		arguments: &mut BTreeMap<Value, Value>,
	) -> Result<(), DecodeError>
	where
		B: Buf,
	{
		if let Some(message) = self.as_message() {
			match message.field_by_name(name) {
				Some(field) => match field.argument_by_tag(tag) {
					Some(arg) => arg.decode(buf, ctx, wire_type, arguments),
					None => Err(DecodeError::new(format!(
						"Message `{}` has no argument with tag `{}`",
						name, tag
					))),
				},
				None => Err(DecodeError::new(format!(
					"Message `{}` has no field with name `{}`",
					message.type_name(),
					name,
				))),
			}
		} else {
			Err(DecodeError::new(format!(
				"Type `{}` needs to be of type `Message`",
				self.type_name()
			)))
		}
	}

	pub(crate) async fn encode<'a>(
		&self,
		ctx: &'a ContextBase,
		accessor: &'a ObjectAccessor<'a>,
		name: &'a str,
	) -> SeaResult<(usize, BytesMut)> {
		if let Some(message) = self.as_message() {
			match message.field_by_name(name) {
				Some(field) => {
					let res = field.collect(ctx, accessor, None, 0, true).await?;

					Ok(res)
				}
				None => Err(SeaographyError::new(format!(
					"Message `{}` has no field with name `{}`",
					message.type_name(),
					name,
				))),
			}
		} else {
			Err(SeaographyError::new(format!(
				"Type `{}` needs to be of type `Message`",
				self.type_name()
			)))
		}
	}

	pub fn to_value<B>(&self, buf: &mut B, value: &Value, tag: u32) -> SeaResult<usize>
	where
		B: BufMut,
	{
		match self {
			Type::Enum(en) => en.to_value(buf, value, tag),
			name => panic!("Type `{}` is not of type Scalar or Enum", name.type_name()),
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
		recursion: u32,
	) -> Vec<BoxFieldFutureByte<'a, BytesMut>> {
		if recursion == 5 {
			return Vec::new();
		}
		match self {
			Type::Message(m) => m.collect(ctx, arguments, parent_value, recursion),
			Type::Enum(e) => Vec::from([e.collect()]),
		}
	}

	pub(crate) fn register(&self, file: &mut FileDescriptorProto, is_service: bool) {
		match self {
			Type::Message(m) => m.register(file, is_service),
			Type::Enum(e) => e.register(file),
		};
	}
}

impl From<Message> for Type {
	#[inline]
	fn from(obj: Message) -> Self {
		Type::Message(obj)
	}
}

impl From<Enum> for Type {
	#[inline]
	fn from(e: Enum) -> Self {
		Type::Enum(e)
	}
}
