use std::any::Any;

use crate::prelude::*;

pub struct DynamicBuilder {
	data: Data,
	pub types: Vec<Type>,
	pub schema_builder: SchemaBuilder,
	pub proto_builder: ProtoBuilder,
}

impl DynamicBuilder {
	pub fn new(schema_builder: SchemaBuilder, proto_builder: ProtoBuilder) -> Self {
		Self {
			data: Default::default(),
			types: Vec::new(),
			schema_builder,
			proto_builder,
		}
	}

	/// Add a global data that can be accessed in the `Proto`. You access it
	/// with `Context::data`.
	#[must_use]
	pub fn data<D: Any + Send + Sync>(mut self, data: D) -> Self {
		self.data.insert(data);
		self
	}

	#[must_use]
	pub fn register(mut self, ty: impl Into<Type>) -> Self {
		let ty = ty.into();
		self.types.push(ty);
		self
	}

	pub fn to_graphql(self) -> SchemaBuilder {
		self.types
			.into_iter()
			.fold(self.schema_builder, |builder, ty| builder.register(ty.register_graphql()))
	}

	pub fn to_proto(mut self) -> ProtoBuilder {
		self.proto_builder.data.merge(self.data);
		self.types
			.into_iter()
			.fold(self.proto_builder, |builder, ty| builder.register(ty.register_proto()))
	}

	pub fn register_schema(mut self, ty: impl Into<GraphQLType>) -> Self {
		self.schema_builder = self.schema_builder.register(ty);
		self
	}

	pub fn register_proto(mut self, ty: impl Into<ProtoType>) -> Self {
		self.proto_builder = self.proto_builder.register(ty);
		self
	}
}
