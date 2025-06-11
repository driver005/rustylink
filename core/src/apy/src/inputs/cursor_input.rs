use crate::BuilderContext;
use dynamic::prelude::*;

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

	/// used to get cursor pagination options message
	pub fn input_object<Ty>(&self) -> Object<Ty>
	where
		Ty: TypeRefTrait,
	{
		Object::new(&self.context.cursor_input.type_name, IO::Input)
			.field(Field::input(&self.context.cursor_input.cursor, Ty::named(Ty::STRING)))
			.field(Field::input(&self.context.cursor_input.limit, Ty::named_nn(Ty::UINT64)))
	}

	/// used to parse query input to cursor pagination options struct
	pub fn parse_object<'a>(&self, object: &'a ObjectAccessor<'a>) -> SeaResult<CursorInput> {
		let cursor = object.get(&self.context.cursor_input.cursor);
		let cursor = match cursor {
			Some(cursor) => Some(cursor.string()?.to_string()),
			None => None,
		};

		let limit = match object.get(&self.context.cursor_input.limit) {
			Some(value_accessor) => value_accessor,
			None => {
				return Err(SeaographyError::new(format!(
					"{} is a required argument but not provided.",
					self.context.cursor_input.limit
				)));
			}
		}
		.uint64()?;

		Ok(CursorInput {
			cursor,
			limit,
		})
	}
}
