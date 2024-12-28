use super::{BoxFieldFuture, Enum, FieldValue, Message, ObjectAccessor, Result, Scalar, Service};
use crate::{Context, Registry, SchemaError};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

lazy_static! {
	pub(crate) static ref TYPES: Arc<RwLock<IndexMap<String, Arc<Type>>>> =
		Arc::new(RwLock::new(IndexMap::new()));
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
	pub(crate) fn as_service(&self) -> Option<&Service> {
		if let Type::Service(obj) = self {
			Some(obj)
		} else {
			None
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a Context<'a>,
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

	pub(crate) fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
		if registry.types.proto.contains_key(self.name()) {
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
