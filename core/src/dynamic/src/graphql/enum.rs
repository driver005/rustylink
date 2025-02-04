use crate::EnumTrait;
pub use async_graphql::dynamic::{Enum, EnumItem};

impl EnumTrait for Enum {
	type Item = EnumItem;

	/// Create a GraphqL enum type
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		Enum::new(name)
	}

	/// Add an item
	#[inline]
	fn item(self, item: impl Into<Self::Item>) -> Self {
		self.item(item)
	}

	/// Add items
	fn items(self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self {
		self.items(fields)
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> String {
		self.type_name().to_string()
	}
}
