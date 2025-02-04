use super::{Enum, Object};
use crate::prelude::{GraphQLType, ProtoType};

/// A type
pub enum Type {
	/// Object
	Object(Object),
	/// Enum
	Enum(Enum),
}

impl Type {
	pub(crate) fn register_graphql(self) -> GraphQLType {
		match self {
			Type::Object(o) => o.to_graphql(),
			Type::Enum(e) => e.to_graphql(),
		}
	}

	pub(crate) fn register_proto(self) -> ProtoType {
		match self {
			Type::Object(o) => o.to_proto(),
			Type::Enum(e) => e.to_proto(),
		}
	}
}

impl From<Object> for Type {
	#[inline]
	fn from(obj: Object) -> Self {
		Type::Object(obj)
	}
}

impl From<Enum> for Type {
	#[inline]
	fn from(e: Enum) -> Self {
		Type::Enum(e)
	}
}
