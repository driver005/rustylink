use crate::prelude::{GraphQLEnum, GraphQLEnumItem, ProtoEnum, ProtoEnumItem};
use crate::traits::EnumTrait;

pub struct EnumItems {
	graphql: GraphQLEnumItem,
	proto: ProtoEnumItem,
}

impl EnumItems {
	pub fn new(graphql: GraphQLEnumItem, proto: ProtoEnumItem) -> Self {
		Self {
			graphql,
			proto,
		}
	}
}

pub struct Enum {
	graphql: GraphQLEnum,
	proto: ProtoEnum,
}

impl EnumTrait for Enum {
	type Item = EnumItems;

	/// Create a GraphqL enum type
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			graphql: GraphQLEnum::new(&name),
			proto: ProtoEnum::new(&name),
		}
	}

	/// Add an item
	#[inline]
	fn item(mut self, item: impl Into<Self::Item>) -> Self {
		let item = item.into();

		self.graphql = self.graphql.item(item.graphql);
		self.proto = self.proto.item(item.proto);

		self
	}

	/// Add items
	fn items(mut self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self {
		for item in fields {
			let item = item.into();
			self = self.item(item);
		}

		self
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> String {
		format!("{}_{}", self.graphql.type_name(), self.proto.type_name())
	}
}
