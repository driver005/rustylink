use crate::TypeRefTrait;
use prost_types::field_descriptor_proto::{Label, Type};
use std::{
	borrow::Cow,
	fmt::{self, Display},
};

use super::TYPE_REGISTRY;

/// A type reference
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeRef {
	/// Named type
	Named(Cow<'static, str>),
	/// Non-null type
	NonNull(Box<TypeRef>),
	/// List type
	List(Box<TypeRef>),
}

impl Display for TypeRef {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TypeRef::Named(name) => write!(f, "{}", name),
			TypeRef::NonNull(ty) => write!(f, "{}", ty),
			TypeRef::List(ty) => write!(f, "{}", ty),
		}
	}
}

impl TypeRefTrait for TypeRef {
	/// Returns the nullable type reference
	///
	/// GraphQL Type: `T`
	#[inline]
	fn named(type_name: impl Into<String>) -> TypeRef {
		TypeRef::Named(type_name.into().into())
	}

	/// Returns the non-null type reference
	///
	/// GraphQL Type: `T!`
	#[inline]
	fn named_nn(type_name: impl Into<String>) -> TypeRef {
		TypeRef::NonNull(Box::new(TypeRef::Named(type_name.into().into())))
	}

	/// Returns a nullable list of nullable members type reference
	///
	/// GraphQL Type: `[T]`
	#[inline]
	fn named_list(type_name: impl Into<String>) -> TypeRef {
		TypeRef::List(Box::new(TypeRef::Named(type_name.into().into())))
	}

	/// Returns a nullable list of non-null members type reference
	///
	/// GraphQL Type: `[T!]`
	#[inline]
	fn named_nn_list(type_name: impl Into<String>) -> TypeRef {
		TypeRef::List(Box::new(TypeRef::NonNull(Box::new(TypeRef::Named(type_name.into().into())))))
	}

	/// Returns a non-null list of nullable members type reference
	///
	/// GraphQL Type: `[T]!`
	#[inline]
	fn named_list_nn(type_name: impl Into<String>) -> TypeRef {
		TypeRef::NonNull(Box::new(TypeRef::List(Box::new(TypeRef::Named(type_name.into().into())))))
	}

	/// Returns a non-null list of non-null members type reference
	///
	/// GraphQL Type: `[T!]!`
	#[inline]
	fn named_nn_list_nn(type_name: impl Into<String>) -> TypeRef {
		TypeRef::NonNull(Box::new(TypeRef::List(Box::new(TypeRef::NonNull(Box::new(
			TypeRef::Named(type_name.into().into()),
		))))))
	}

	/// Returns the non-null type reference
	fn non_null(ty: Box<Self>) -> Self {
		TypeRef::NonNull(ty)
	}

	/// Returns the type name
	///
	/// `[Foo!]` -> `Foo`
	#[inline(always)]
	fn type_name(&self) -> &str {
		self.type_name()
	}

	const DOUBLE: &'static str = TypeRef::DOUBLE;
	const FLOAT: &'static str = TypeRef::FLOAT;
	const INT32: &'static str = TypeRef::INT32;
	const INT64: &'static str = TypeRef::INT64;
	const UINT32: &'static str = TypeRef::UINT32;
	const UINT64: &'static str = TypeRef::UINT64;
	const SINT32: &'static str = TypeRef::SINT32;
	const SINT64: &'static str = TypeRef::SINT64;
	const FIXED32: &'static str = TypeRef::FIXED32;
	const FIXED64: &'static str = TypeRef::FIXED64;
	const SFIXED32: &'static str = TypeRef::SFIXED32;
	const SFIXED64: &'static str = TypeRef::SFIXED64;
	const BOOL: &'static str = TypeRef::BOOL;
	const STRING: &'static str = TypeRef::STRING;
	const BYTES: &'static str = TypeRef::BYTES;
	const ID: &'static str = TypeRef::STRING;
	// const UPLOAD: &'static str = TypeRef::BYTES;
}

impl TypeRef {
	/// Protobuf double type
	pub const DOUBLE: &'static str = "double";

	/// Protobuf float type
	pub const FLOAT: &'static str = "float";

	/// Protobuf int32 type
	pub const INT32: &'static str = "int32";

	/// Protobuf int64 type
	pub const INT64: &'static str = "int64";

	/// Protobuf uint32 type
	pub const UINT32: &'static str = "uint32";

	/// Protobuf uint64 type
	pub const UINT64: &'static str = "uint64";

	/// Protobuf sint32 type
	pub const SINT32: &'static str = "sint32";

	/// Protobuf sint64 type
	pub const SINT64: &'static str = "sint64";

	/// Protobuf fixed32 type
	pub const FIXED32: &'static str = "fixed32";

	/// Protobuf fixed64 type
	pub const FIXED64: &'static str = "fixed64";

	/// Protobuf sfixed32 type
	pub const SFIXED32: &'static str = "sfixed32";

	/// Protobuf sfixed64 type
	pub const SFIXED64: &'static str = "sfixed64";

	/// Protobuf bool type
	pub const BOOL: &'static str = "bool";

	/// Protobuf string type
	pub const STRING: &'static str = "string";

	/// Protobuf bytes type
	pub const BYTES: &'static str = "bytes";

	/// Returns the type name
	///
	/// `[Foo!]` -> `Foo`
	#[inline(always)]
	pub fn type_name(&self) -> &str {
		match self {
			TypeRef::Named(name) => name,
			TypeRef::NonNull(inner) => inner.type_name(),
			TypeRef::List(inner) => inner.type_name(),
		}
	}

	#[inline]
	pub(crate) fn field_type(&self) -> Type {
		match self {
			TypeRef::Named(name) => match name.as_ref() {
				Self::DOUBLE => Type::Double,
				Self::FLOAT => Type::Float,
				Self::INT32 => Type::Int32,
				Self::INT64 => Type::Int64,
				Self::UINT32 => Type::Uint32,
				Self::UINT64 => Type::Uint64,
				Self::SINT32 => Type::Sint32,
				Self::SINT64 => Type::Sint64,
				Self::FIXED32 => Type::Fixed32,
				Self::FIXED64 => Type::Fixed64,
				Self::SFIXED32 => Type::Sfixed32,
				Self::SFIXED64 => Type::Sfixed64,
				Self::BOOL => Type::Bool,
				Self::STRING => Type::String,
				Self::BYTES => Type::Bytes,
				name => match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.field_type(),
					None => panic!("custom types was not found"),
				},
			},
			TypeRef::NonNull(inner) => inner.field_type(),
			TypeRef::List(inner) => inner.field_type(),
		}
	}

	#[inline]
	pub(crate) fn field_label(&self) -> Label {
		if self.is_repeated() {
			Label::Repeated
		} else if self.is_nullable() {
			Label::Optional
		} else {
			Label::Required
		}
	}

	#[inline]
	pub(crate) fn is_nullable(&self) -> bool {
		match self {
			TypeRef::Named(_) => true,

			//TODO: Change back to false
			TypeRef::NonNull(_) => true,
			TypeRef::List(_) => true,
		}
	}

	#[inline]
	pub(crate) fn is_repeated(&self) -> bool {
		match self {
			TypeRef::Named(_) => false,
			TypeRef::NonNull(inner) => inner.is_repeated(),
			TypeRef::List(_) => true,
		}
	}

	pub(crate) fn is_default(&self) -> bool {
		match self.type_name() {
			TypeRef::DOUBLE
			| TypeRef::FLOAT
			| TypeRef::INT32
			| TypeRef::INT64
			| TypeRef::UINT32
			| TypeRef::UINT64
			| TypeRef::SINT32
			| TypeRef::SINT64
			| TypeRef::FIXED32
			| TypeRef::FIXED64
			| TypeRef::SFIXED32
			| TypeRef::SFIXED64
			| TypeRef::BOOL
			| TypeRef::STRING
			| TypeRef::BYTES => true,
			_ => false,
		}
	}

	pub(crate) fn is_subtype(&self, sub: &TypeRef) -> bool {
		fn is_subtype(cur: &TypeRef, sub: &TypeRef) -> bool {
			match (cur, sub) {
				(TypeRef::NonNull(super_type), TypeRef::NonNull(sub_type)) => {
					is_subtype(&super_type, &sub_type)
				}
				(_, TypeRef::NonNull(sub_type)) => is_subtype(cur, &sub_type),
				(TypeRef::Named(super_type), TypeRef::Named(sub_type)) => super_type == sub_type,
				(TypeRef::List(super_type), TypeRef::List(sub_type)) => {
					is_subtype(super_type, sub_type)
				}
				_ => false,
			}
		}

		is_subtype(self, sub)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create() {
		assert_eq!(TypeRef::named("MyObj").to_string(), "MyObj");
		assert_eq!(TypeRef::named_nn("MyObj").to_string(), "MyObj!");
		assert_eq!(TypeRef::named_list("MyObj").to_string(), "[MyObj]");
		assert_eq!(TypeRef::named_list_nn("MyObj").to_string(), "[MyObj]!");
		assert_eq!(TypeRef::named_nn_list("MyObj").to_string(), "[MyObj!]");
		assert_eq!(TypeRef::named_nn_list_nn("MyObj").to_string(), "[MyObj!]!");
	}
}
