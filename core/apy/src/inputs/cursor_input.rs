use dynamic::prelude::{
	GraphQLInputObject, GraphQLInputValue, GraphQLTypeRef, ObjectAccessor, ProtoField,
	ProtoMessage, ProtoTypeRef, ValueAccessor,
};

use crate::BuilderContext;

/// used to hold information about cursor pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorInput {
	pub cursor: Option<String>,
	pub limit: u64,
}

/// The configuration structure for CursorInputBuilder
pub struct CursorInputConfig {
	/// name of the object
	pub type_name: String,
	/// name for 'cursor' field
	pub cursor: String,
	/// name for 'limit' field
	pub limit: String,
}

impl std::default::Default for CursorInputConfig {
	fn default() -> Self {
		Self {
			type_name: "CursorInput".into(),
			cursor: "cursor".into(),
			limit: "limit".into(),
		}
	}
}

/// This builder produces the cursor pagination options input object
pub struct CursorInputBuilder {
	pub context: &'static BuilderContext,
}

impl CursorInputBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.cursor_input.type_name.clone()
	}

	/// used to get cursor pagination options object
	pub fn input_object(&self) -> GraphQLInputObject {
		GraphQLInputObject::new(&self.context.cursor_input.type_name)
			.field(GraphQLInputValue::new(
				&self.context.cursor_input.cursor,
				GraphQLTypeRef::named(GraphQLTypeRef::STRING),
			))
			.field(GraphQLInputValue::new(
				&self.context.cursor_input.limit,
				GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
			))
	}

	/// used to get cursor pagination options message
	pub fn message(&self) -> ProtoMessage {
		ProtoMessage::new(&self.context.cursor_input.type_name)
			.field(ProtoField::input(
				&self.context.cursor_input.cursor,
				1u32,
				ProtoTypeRef::named(ProtoTypeRef::STRING),
			))
			.field(ProtoField::input(
				&self.context.cursor_input.limit,
				2u32,
				ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
			))
	}

	/// used to parse query input to cursor pagination options struct
	pub fn parse_object<'a, V>(&self, object: &'a V) -> CursorInput
	where
		V: ObjectAccessor<'a>,
	{
		let limit = object.get(&self.context.cursor_input.limit).unwrap().u64().unwrap();

		let cursor = object.get(&self.context.cursor_input.cursor);
		let cursor: Option<String> = cursor.map(|cursor| cursor.string().unwrap().into());

		CursorInput {
			cursor,
			limit,
		}
	}
}
