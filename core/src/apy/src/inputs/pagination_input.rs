use super::{CursorInput, OffsetInput, PageInput};
use crate::{BuilderContext, CursorInputBuilder, OffsetInputBuilder, PageInputBuilder};
use dynamic::prelude::*;

/// used to hold information about which pagination
/// strategy will be applied on the query
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaginationInput {
	pub cursor: Option<CursorInput>,
	pub page: Option<PageInput>,
	pub offset: Option<OffsetInput>,
}

/// The configuration structure for PaginationInputBuilder
pub struct PaginationInputConfig {
	/// name of the object
	pub type_name: String,
	/// name for 'cursor' field
	pub cursor: String,
	/// name for 'page' field
	pub page: String,
	/// name for 'offset' field
	pub offset: String,
}

impl std::default::Default for PaginationInputConfig {
	fn default() -> Self {
		PaginationInputConfig {
			type_name: "PaginationInput".into(),
			cursor: "cursor".into(),
			page: "page".into(),
			offset: "offset".into(),
		}
	}
}

pub struct PaginationInputBuilder {
	pub context: &'static BuilderContext,
}

impl PaginationInputBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.pagination_input.type_name.clone()
	}

	/// used to get pagination input message
	pub fn input_object(&self) -> Object {
		Object::new(&self.context.pagination_input.type_name, IO::Input)
			.field(Field::input(
				&self.context.pagination_input.cursor,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named(&self.context.cursor_input.type_name),
					ProtoTypeRef::named(&self.context.cursor_input.type_name),
				),
			))
			.field(Field::input(
				&self.context.pagination_input.page,
				2u32,
				TypeRef::new(
					GraphQLTypeRef::named(&self.context.page_input.type_name),
					ProtoTypeRef::named(&self.context.page_input.type_name),
				),
			))
			.field(Field::input(
				&self.context.pagination_input.offset,
				3u32,
				TypeRef::new(
					GraphQLTypeRef::named(&self.context.offset_input.type_name),
					ProtoTypeRef::named(&self.context.offset_input.type_name),
				),
			))
			.oneof()
	}
	/// used to parse query input to pagination information structure
	pub fn parse_object<'a>(&self, value: Option<ValueAccessors<'a>>) -> PaginationInput {
		if value.is_none() {
			return PaginationInput {
				cursor: None,
				offset: None,
				page: None,
			};
		}

		let binding = value.unwrap();
		let object = match binding.object() {
			Ok(obj) => obj,
			Err(_) => {
				return PaginationInput {
					cursor: None,
					offset: None,
					page: None,
				}
			}
		};

		let cursor_input_builder = CursorInputBuilder {
			context: self.context,
		};
		let page_input_builder = PageInputBuilder {
			context: self.context,
		};
		let offset_input_builder = OffsetInputBuilder {
			context: self.context,
		};

		let cursor = if let Some(cursor) = object.get(&self.context.pagination_input.cursor) {
			let object = cursor.object().unwrap();
			Some(cursor_input_builder.parse_object(&object))
		} else {
			None
		};

		let page = if let Some(page) = object.get(&self.context.pagination_input.page) {
			let object = page.object().unwrap();
			Some(page_input_builder.parse_object(&object))
		} else {
			None
		};

		let offset = if let Some(offset) = object.get(&self.context.pagination_input.offset) {
			let object = offset.object().unwrap();
			Some(offset_input_builder.parse_object(&object))
		} else {
			None
		};

		PaginationInput {
			cursor,
			page,
			offset,
		}
	}
}
