use crate::BuilderContext;
use dynamic::prelude::*;
use std::sync::Arc;

/// used to hold information about offset pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OffsetInput {
	pub offset: u64,
	pub limit: u64,
}

/// The configuration structure for OffsetInputBuilder
pub struct OffsetInputConfig {
	/// name of the object
	pub type_name: String,
	/// name for 'offset' field
	pub offset: String,
	/// name for 'limit' field
	pub limit: String,
}

impl OffsetInputConfig {
	pub fn type_name(&self) -> Arc<String> {
		Arc::new(self.type_name.clone())
	}
}

impl std::default::Default for OffsetInputConfig {
	fn default() -> Self {
		Self {
			type_name: "OffsetInput".into(),
			offset: "offset".into(),
			limit: "limit".into(),
		}
	}
}

/// This builder produces the offset pagination options input object
pub struct OffsetInputBuilder {
	pub context: &'static BuilderContext,
}

impl OffsetInputBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.offset_input.type_name.clone()
	}

	/// used to get offset pagination options message
	pub fn input_object(&self) -> Object {
		Object::new(&self.context.offset_input.type_name, IO::Input)
			.field(Field::input(
				&self.context.offset_input.limit,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
					ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
				),
			))
			.field(Field::input(
				&self.context.offset_input.offset,
				2u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
					ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
				),
			))
	}

	/// used to parse query input to offset pagination options struct
	pub fn parse_object<'a>(&self, object: &'a ObjectAccessors) -> OffsetInput {
		let offset = object
			.get(&self.context.offset_input.offset)
			.map_or_else(|| Ok(0), |v| v.u64())
			.unwrap();

		let limit = object.get(&self.context.offset_input.limit).unwrap().u64().unwrap();

		OffsetInput {
			offset,
			limit,
		}
	}
}
