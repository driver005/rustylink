use crate::BuilderContext;
use dynamic::prelude::*;

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

	/// used to get page pagination options message
	pub fn input_object<Ty>(&self) -> Object<Ty>
	where
		Ty: TypeRefTrait,
	{
		Object::new(&self.context.page_input.type_name, IO::Input)
			.field(Field::input(&self.context.page_input.limit, 1u32, Ty::named_nn(Ty::UINT64)))
			.field(Field::input(&self.context.page_input.page, 2u32, Ty::named_nn(Ty::UINT64)))
	}

	/// used to parse query input to page pagination options struct
	pub fn parse_object<'a>(&self, object: &'a ObjectAccessor<'a>) -> PageInput {
		let page = object
			.get(&self.context.page_input.page)
			.map_or_else(|| Ok(0), |v| v.uint64())
			.unwrap_or(0);
		let limit = object.get(&self.context.page_input.limit).unwrap().uint64().unwrap();

		PageInput {
			page,
			limit,
		}
	}
}
