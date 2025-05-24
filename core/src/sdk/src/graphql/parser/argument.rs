use super::{
	node_as_string, NameString, RenderContext, SnakeCaseWithUnderscores, StructuredSchema,
	ValueTypeDef,
};
use crate::graphql::ResolverArgument;
use anyhow::Result;
use async_graphql_parser::{
	types::{InputValueDefinition, Type},
	Positioned,
};
use macros::NameString;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, NameString, PartialEq)]
pub struct Argument {
	pub name: String,
	pub typ: ValueTypeDef,
	pub description: Option<String>,
	//TODO(tacogips) default value not supported
	//pub default_value: Option<String>,
}

impl Argument {
	pub(crate) fn new(input_def: &Positioned<InputValueDefinition>) -> Self {
		let input_def = input_def.node.clone();
		if let Some(default_value) = input_def.default_value {
			log::warn!(
				"default value of argument is not supported yet. argument:{} {}",
				node_as_string!(input_def.name),
				default_value
			);
		}

		if !input_def.directives.is_empty() {
			log::warn!(
				"directive is not supported yet. argument:{}",
				node_as_string!(input_def.name)
			);
		}

		Self {
			name: node_as_string!(input_def.name),
			typ: ValueTypeDef::new(input_def.ty.node),
			description: input_def.description.map(|desc| node_as_string!(desc)),
		}
	}

	pub(crate) fn new_from_config_arg(arg: &ResolverArgument) -> Self {
		let typ = Type::new(&arg.arg_type)
			.unwrap_or_else(|| panic!("invalid resolver argument type :{:?}", arg));

		Self {
			name: arg.arg_name.clone(),
			typ: ValueTypeDef::new(typ),
			description: arg.arg_description.clone(),
		}
	}

	pub(crate) fn token(
		&self,
		schema: &StructuredSchema,
		render_context: &RenderContext,
		name_prefix: &str,
	) -> Result<TokenStream> {
		let name =
			format_ident!("{}{}", name_prefix, self.name_string().to_snake_case_with_underscores());
		let typ = self.typ.token(&schema, &render_context)?;

		let result = quote! { #name:#typ };

		Ok(result)
	}
}
