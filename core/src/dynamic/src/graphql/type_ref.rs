use super::Registry;
use crate::TypeRefTrait;
use std::{
	borrow::Cow,
	fmt::{self, Display},
};

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
			TypeRef::NonNull(ty) => write!(f, "{}!", ty),
			TypeRef::List(ty) => write!(f, "[{}]", ty),
		}
	}
}

impl TypeRef {
	/// A int scalar type
	pub const INT: &'static str = "int";

	/// A float scalar type
	pub const FLOAT: &'static str = "float";

	/// A string scalar type
	pub const STRING: &'static str = "string";

	/// A boolean scalar type
	pub const BOOLEAN: &'static str = "boolean";

	/// A ID scalar type
	pub const ID: &'static str = "id";

	/// A Upload type
	// pub const UPLOAD: &'static str = "Upload";

	/// Returns the nullable type reference
	///
	/// GraphQL Type: `T`
	#[inline]
	pub fn named(type_name: impl Into<String>) -> TypeRef {
		TypeRef::Named(type_name.into().into())
	}

	/// Returns the non-null type reference
	///
	/// GraphQL Type: `T!`
	#[inline]
	pub fn named_nn(type_name: impl Into<String>) -> TypeRef {
		TypeRef::NonNull(Box::new(TypeRef::Named(type_name.into().into())))
	}

	/// Returns a nullable list of nullable members type reference
	///
	/// GraphQL Type: `[T]`
	#[inline]
	pub fn named_list(type_name: impl Into<String>) -> TypeRef {
		TypeRef::List(Box::new(TypeRef::Named(type_name.into().into())))
	}

	/// Returns a nullable list of non-null members type reference
	///
	/// GraphQL Type: `[T!]`
	#[inline]
	pub fn named_nn_list(type_name: impl Into<String>) -> TypeRef {
		TypeRef::List(Box::new(TypeRef::NonNull(Box::new(TypeRef::Named(type_name.into().into())))))
	}

	/// Returns a non-null list of nullable members type reference
	///
	/// GraphQL Type: `[T]!`
	#[inline]
	pub fn named_list_nn(type_name: impl Into<String>) -> TypeRef {
		TypeRef::NonNull(Box::new(TypeRef::List(Box::new(TypeRef::Named(type_name.into().into())))))
	}

	/// Returns a non-null list of non-null members type reference
	///
	/// GraphQL Type: `[T!]!`
	#[inline]
	pub fn named_nn_list_nn(type_name: impl Into<String>) -> TypeRef {
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
	pub fn type_name(&self) -> &str {
		match self {
			TypeRef::Named(name) => name,
			TypeRef::NonNull(inner) => inner.type_name(),
			TypeRef::List(inner) => inner.type_name(),
		}
	}

	#[inline]
	pub(crate) fn is_nullable(&self) -> bool {
		match self {
			TypeRef::Named(_) => true,
			TypeRef::NonNull(_) => false,
			TypeRef::List(_) => true,
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

impl TypeRefTrait for TypeRef {
	fn named(type_name: impl Into<String>) -> Self {
		TypeRef::named(type_name)
	}

	fn named_nn(type_name: impl Into<String>) -> Self {
		TypeRef::named_nn(type_name)
	}

	fn named_list(type_name: impl Into<String>) -> Self {
		TypeRef::named_list(type_name)
	}

	fn named_nn_list(type_name: impl Into<String>) -> Self {
		TypeRef::named_nn_list(type_name)
	}

	fn named_list_nn(type_name: impl Into<String>) -> Self {
		TypeRef::named_list_nn(type_name)
	}

	fn named_nn_list_nn(type_name: impl Into<String>) -> Self {
		TypeRef::named_nn_list_nn(type_name)
	}

	fn non_null(ty: Box<Self>) -> Self {
		TypeRef::non_null(ty)
	}

	fn type_name(&self) -> &str {
		self.type_name()
	}

	const DOUBLE: &'static str = TypeRef::FLOAT;
	const FLOAT: &'static str = TypeRef::FLOAT;
	const INT32: &'static str = TypeRef::INT;
	const INT64: &'static str = TypeRef::INT;
	const UINT32: &'static str = TypeRef::INT;
	const UINT64: &'static str = TypeRef::INT;
	const SINT32: &'static str = TypeRef::INT;
	const SINT64: &'static str = TypeRef::INT;
	const FIXED32: &'static str = TypeRef::INT;
	const FIXED64: &'static str = TypeRef::INT;
	const SFIXED32: &'static str = TypeRef::INT;
	const SFIXED64: &'static str = TypeRef::INT;
	const BOOL: &'static str = TypeRef::BOOLEAN;
	const STRING: &'static str = TypeRef::STRING;
	const BYTES: &'static str = TypeRef::STRING;
	const ID: &'static str = TypeRef::ID;
	// const UPLOAD: &'static str = TypeRef::UPLOAD;
}
