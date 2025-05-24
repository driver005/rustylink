use super::{Enum, Message, Result, Scalar, Service, TypeRef};
use crate::{
	BoxFieldFuture, ContextBase, EnumTrait, FieldValue, ObjectAccessor, ProtoRegistry, SchemaError,
	SeaResult, SeaographyError, TypeRefTrait, Value,
};
use binary::proto::Decoder;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

lazy_static! {
	pub(crate) static ref TYPES: Arc<RwLock<BTreeMap<String, Arc<Type>>>> =
		Arc::new(RwLock::new(BTreeMap::new()));
}

// Functions to interact with TYPES

pub fn add_type(name: String, ty: Type) {
	let mut types = TYPES.write().unwrap(); // Write access to the RwLock
	types.insert(name, Arc::new(ty));
}

pub fn get_type(name: &str) -> Option<Arc<Type>> {
	let types = TYPES.read().unwrap(); // Read access to the RwLock
	types.get(name).map(|ty| ty.clone())
}

/// A GraphQL type
#[derive(Debug)]
pub enum Type {
	/// Scalar
	Scalar(Scalar),
	/// Service
	Service(Service),
	/// Message
	Message(Message),
	/// Enum
	Enum(Enum),
}

impl Type {
	pub(crate) fn name(&self) -> &str {
		match self {
			Type::Scalar(s) => &s.name,
			Type::Service(s) => &s.name,
			Type::Message(m) => &m.name,
			Type::Enum(e) => &e.name,
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

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Vec<BoxFieldFuture<'a>> {
		match self {
			Type::Message(m) => m.collect(ctx, arguments, parent_value),
			Type::Enum(e) => Vec::from([e.collect(arguments)]),
			Type::Service(s) => s.collect(ctx, arguments, parent_value),
			Type::Scalar(_) => Vec::new(),
		}
	}

	pub(crate) fn register(&self, registry: &mut ProtoRegistry) -> Result<(), SchemaError> {
		if registry.types.contains_key(self.name()) {
			return Err(SchemaError(format!("Type \"{0}\" already exists", self.name())));
		}

		match self {
			Type::Scalar(s) => s.register(registry),
			Type::Service(s) => s.register(registry),
			Type::Message(m) => m.register(registry),
			Type::Enum(e) => e.register(registry),
		}
	}
}

impl From<Service> for Type {
	#[inline]
	fn from(obj: Service) -> Self {
		Type::Service(obj)
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
