use std::sync::Arc;

use crate::Value;

pub trait EnumTrait {
	type Item: EnumItemTrait;

	fn new(name: impl Into<String>) -> Self;

	fn item(self, item: impl Into<Self::Item>) -> Self;

	fn items(self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self;

	fn type_name(&self) -> &str;
}

pub trait EnumItemTrait {
	fn new(name: impl Into<String>) -> Self;

	fn type_name(&self) -> &str;
}

/// Trait for common behaviors of TypeRef
pub trait TypeRefTrait: Sized {
	/// Returns the nullable type reference
	///
	/// GraphQL Type: `T`
	fn named(type_name: impl Into<String>) -> Self;
	/// Returns the non-null type reference
	///
	/// GraphQL Type: `T!`
	fn named_nn(type_name: impl Into<String>) -> Self;
	/// Returns a nullable list of nullable members type reference
	///
	/// GraphQL Type: `[T]`
	fn named_list(type_name: impl Into<String>) -> Self;
	/// Returns a nullable list of non-null members type reference
	///
	/// GraphQL Type: `[T!]`
	fn named_nn_list(type_name: impl Into<String>) -> Self;
	/// Returns a non-null list of nullable members type reference
	///
	/// GraphQL Type: `[T]!`
	fn named_list_nn(type_name: impl Into<String>) -> Self;
	/// Returns a non-null list of non-null members type reference
	///
	/// GraphQL Type: `[T!]!`
	fn named_nn_list_nn(type_name: impl Into<String>) -> Self;
	/// Returns the non-null type reference
	fn non_null(ty: Box<Self>) -> Self;
	/// Returns the type name
	///
	/// `[Foo!]` -> `Foo`
	fn type_name(&self) -> &str;

	const DOUBLE: &'static str;
	const FLOAT: &'static str;
	const INT32: &'static str;
	const INT64: &'static str;
	const UINT32: &'static str;
	const UINT64: &'static str;
	const SINT32: &'static str;
	const SINT64: &'static str;
	const FIXED32: &'static str;
	const FIXED64: &'static str;
	const SFIXED32: &'static str;
	const SFIXED64: &'static str;
	const BOOL: &'static str;
	const STRING: &'static str;
	const BYTES: &'static str;
	const ID: &'static str;
	// const UPLOAD: &'static str;
}

pub trait ErrorTrait {
	fn new(message: impl Into<String>) -> Self;

	/// Implement `From<T>` for each type as needed
	fn to<T>(value: T) -> Self
	where
		T: std::fmt::Display + Send + Sync + 'static;
}

/// A validator for scalar
pub type ScalarValidatorFn = Arc<dyn Fn(&Value) -> bool + Send + Sync>;
