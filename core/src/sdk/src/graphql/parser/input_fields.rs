use super::{
	get_attribute_from_resolver_settings, maybe_replace_field, node_as_string,
	sort_by_line_pos_and_name, to_rust_docs_token, LinePosition, NameString, RenderContext,
	SnakeCaseWithUnderscores, StructuredSchema, ValueTypeDef, RUST_KEYWORDS,
};
use crate::graphql::{FieldsSetting, ResolverSetting};
use anyhow::Result;
use async_graphql_parser::{types::InputValueDefinition, Positioned};
use macros::{LinePosition, NameString};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;

struct InputMember {
	pub member: TokenStream,
	pub method: Option<TokenStream>,
	pub dependencies: Vec<TokenStream>,
}

pub struct InputFieldsInfo {
	pub members: Vec<TokenStream>,
	pub methods: Vec<TokenStream>,
	pub dependencies: Vec<TokenStream>,
}

impl InputFieldsInfo {
	pub(crate) fn new(
		mut fields: Vec<&InputField>,
		schema: &StructuredSchema,
		context: &RenderContext,
		resolver_settings: Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<InputFieldsInfo> {
		fields.sort_by(sort_by_line_pos_and_name);
		let mut result = InputFieldsInfo {
			members: vec![],
			methods: vec![],
			dependencies: vec![],
		};
		for field in fields.iter() {
			let InputMember {
				member,
				method,
				mut dependencies,
			} = field.convert(schema, context, &resolver_settings)?;

			result.members.push(member);

			if let Some(method) = method {
				result.methods.push(method);
			}

			result.dependencies.append(&mut dependencies);
		}
		Ok(result)
	}
}

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct InputField {
	pub name: String,
	pub description: Option<String>,
	pub typ: ValueTypeDef,
	pub line_pos: usize,
}

impl InputField {
	pub(crate) fn new(
		input_field_def: &Positioned<InputValueDefinition>,
		fields_setting: Option<&FieldsSetting>,
	) -> Self {
		let line_pos = input_field_def.pos.line;
		let input_field_def = input_field_def.node.clone();

		let field_name = node_as_string!(input_field_def.name);
		let field_type = match maybe_replace_field(&field_name, fields_setting).unwrap() {
			Some(replaced_field) => replaced_field,
			None => input_field_def.ty.node,
		};

		Self {
			name: node_as_string!(input_field_def.name),
			description: input_field_def.description.map(|desc| node_as_string!(desc)),
			typ: ValueTypeDef::new(field_type),
			line_pos,
		}
	}

	pub(crate) fn name(&self) -> Ident {
		let field_name = self.name_string().to_snake_case_with_underscores();
		if RUST_KEYWORDS.contains(&field_name.as_ref()) {
			format_ident!("r#{}", field_name)
		} else {
			format_ident!("{}", field_name)
		}
	}

	fn convert(
		&self,
		schema: &StructuredSchema,
		render_context: &RenderContext,
		resolver_settings: &Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<InputMember> {
		let name = self.name();
		let (typ, nullable) = self.typ.token_nullable(&schema, &render_context)?;

		let filed_name = &self.name;
		let field_attribute = quote! {
			#[serde(rename = #filed_name)]
		};

		let attribute = match get_attribute_from_resolver_settings(&self.name, resolver_settings) {
			Some(attribute) => attribute.to_token_stream(),
			None => quote! {},
		};

		let field_rustdoc = match &self.description {
			Some(desc_token) => to_rust_docs_token(desc_token),
			None => quote! {},
		};

		let function_typ = if nullable {
			quote! { Option<#typ > }
		} else {
			quote! { #typ  }
		};

		let member = quote! { #field_attribute pub #name :#function_typ };

		let function_name = format_ident!("set_{}", name);
		let set_value = if nullable {
			quote! {self.#name = Some(#name)}
		} else {
			quote! {self.#name = #name}
		};

		let method = Some(quote! {
			#field_rustdoc
			#attribute
			pub fn #function_name(mut self, #name: #typ) -> Self {
				#set_value;
				self
			}
		});

		let dependencies = self.typ.dependency(schema, render_context)?;

		Ok(InputMember {
			member,
			method,
			dependencies,
		})
	}
}
