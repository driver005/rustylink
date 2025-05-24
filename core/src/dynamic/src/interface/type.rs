use super::Object;
use crate::{
	EnumTrait, TypeRefTrait,
	prelude::{GraphQLEnum, GraphQLType, GraphQLTypeRef, ProtoEnum, ProtoType, ProtoTypeRef},
};

/// A type
pub enum Type<T, E>
where
	T: TypeRefTrait,
	E: EnumTrait,
{
	/// Object
	Object(Object<T>),
	/// Enum
	Enum(E),
}

impl Type<GraphQLTypeRef, GraphQLEnum> {
	pub(crate) fn register(self) -> GraphQLType {
		match self {
			Type::Object(o) => o.to_type(),
			Type::Enum(e) => e.into(),
		}
	}
}
impl Type<ProtoTypeRef, ProtoEnum> {
	pub(crate) fn registet(self) -> ProtoType {
		match self {
			Type::Object(o) => o.to_type(),
			Type::Enum(e) => e.into(),
		}
	}
}

impl<T, E> From<Object<T>> for Type<T, E>
where
	T: TypeRefTrait,
	E: EnumTrait,
{
	#[inline]
	fn from(obj: Object<T>) -> Self {
		Type::Object(obj)
	}
}

impl<T, E> From<E> for Type<T, E>
where
	T: TypeRefTrait,
	E: EnumTrait,
{
	#[inline]
	fn from(e: E) -> Self {
		Type::Enum(e)
	}
}
