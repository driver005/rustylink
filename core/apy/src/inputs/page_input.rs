use dynamic::prelude::{
	GraphQLInputObject, GraphQLInputValue, GraphQLTypeRef, ObjectAccessor, ProtoField,
	ProtoMessage, ProtoTypeRef, ValueAccessor,
};

use crate::BuilderContext;

/// used to hold information about page pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageInput {
	pub page: u64,
	pub limit: u64,
}

/// The configuration structure for PageInputBuilder
pub struct PageInputConfig {
	/// name of the object
	pub type_name: String,
	/// name for 'page' field
	pub page: String,
	/// name for 'limit' field
	pub limit: String,
}

impl std::default::Default for PageInputConfig {
	fn default() -> Self {
		PageInputConfig {
			type_name: "PageInput".into(),
			page: "page".into(),
			limit: "limit".into(),
		}
	}
}

/// This builder produces the page pagination options input object
pub struct PageInputBuilder {
	pub context: &'static BuilderContext,
}

impl PageInputBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.page_input.type_name.clone()
	}

	/// used to get page pagination options object
	pub fn input_object(&self) -> GraphQLInputObject {
		GraphQLInputObject::new(&self.context.page_input.type_name)
			.field(GraphQLInputValue::new(
				&self.context.page_input.limit,
				GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
			))
			.field(GraphQLInputValue::new(
				&self.context.page_input.page,
				GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
			))
	}

	/// used to get page pagination options message
	pub fn message(&self) -> ProtoMessage {
		ProtoMessage::new(&self.context.page_input.type_name)
			.field(ProtoField::input(
				&self.context.page_input.limit,
				1u32,
				ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
			))
			.field(ProtoField::input(
				&self.context.page_input.page,
				2u32,
				ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
			))
	}

	/// used to parse query input to page pagination options struct
	pub fn parse_object<'a, O>(&self, object: &'a O) -> PageInput
	where
		O: ObjectAccessor<'a>,
	{
		let page = object
			.get(&self.context.page_input.page)
			.map_or_else(|| Ok(0), |v| v.u64())
			.unwrap_or(0);
		let limit = object.get(&self.context.page_input.limit).unwrap().u64().unwrap();

		PageInput {
			page,
			limit,
		}
	}
}