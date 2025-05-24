use super::{
	utils::{
		dependency_strs_to_token, separate_by_comma, sort_by_line_pos_and_name,
		SnakeCaseWithUnderscores, FILE_HEADER_COMMENT,
	},
	Field, LinePosition, NameString, NamedValue, RenderContext, StructuredSchema, TypeDef,
	ValueTypeDef,
};
use crate::{graphql::RendererConfig, OutputFile};
use anyhow::Result;
use heck::CamelCase;
use macros::{LinePosition, NameString};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{collections::HashMap, collections::HashSet};

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct Interface {
	pub name: String,
	//TODO(tacogips) concrete_type_names  always be empty?
	pub concrete_type_names: Vec<String>,
	pub fields: Vec<Field>,
	pub description: Option<String>,
	pub line_pos: usize,
}

impl Interface {
	fn token(
		&self,
		schema: &StructuredSchema,
		_render_config: &RendererConfig,
		interface_type_and_impl_types: &HashMap<String, Vec<String>>,
	) -> Result<(TokenStream, Vec<TokenStream>)> {
		let interface_name = format_ident!("{}", self.name);

		let context = RenderContext {
			parent: TypeDef::Interface(self),
		};

		let mut interface_field_tokens = Vec::<TokenStream>::new();
		let mut all_dependency_tokens = Vec::<TokenStream>::new();

		let render_context = RenderContext {
			parent: TypeDef::Interface(self),
		};

		for interface_field in self.fields.iter() {
			let field_name = &interface_field.name.to_snake_case_with_underscores();
			let field_type =
				interface_field.typ.token(&schema, &render_context)?.to_string().replace(" ", "");

			let field_token = quote! {field(name = #field_name, ty = #field_type )};
			interface_field_tokens.push(field_token);

			let mut dependencies = interface_field.typ.dependency(schema, &context)?;
			all_dependency_tokens.append(&mut dependencies);
		}

		let mut interface_memer_tokens = Vec::<TokenStream>::new();
		if let Some(impl_types) = interface_type_and_impl_types.get(&self.name) {
			let mut impl_types = impl_types.clone();
			impl_types.sort();
			for member in impl_types {
				let mut interface_member = InterfaceMember::new(schema, &context, &member)?;
				interface_memer_tokens.push(interface_member.member);

				all_dependency_tokens.append(&mut interface_member.dependencies);
			}
		}

		let interface_fields_token = separate_by_comma(interface_field_tokens);
		let interface_memer_tokens = separate_by_comma(interface_memer_tokens);
		let interface_def = quote! {

			#[derive(Interface)]
			#[graphql(#interface_fields_token)]
			#[derive(Debug, Clone)]
			pub enum #interface_name{
				#interface_memer_tokens
			}

		};
		Ok((interface_def, all_dependency_tokens))
	}

	pub(crate) fn write(
		structured_schema: &StructuredSchema,
		render_config: &RendererConfig,
		output_dir: &str,
	) -> Result<bool> {
		let mut interfaces: Vec<&Self> =
			structured_schema.definitions.interfaces.values().into_iter().collect();
		if interfaces.is_empty() {
			return Ok(false);
		}
		interfaces.sort_by(sort_by_line_pos_and_name);

		let mut all_dependencies = HashSet::<String>::new();
		let mut interface_defs = Vec::<String>::new();

		let interface_and_impl_types = find_implment_types_by_interface_type(&structured_schema);

		for each_obj in interfaces {
			let (interface_token, dependencies) =
				each_obj.token(&structured_schema, render_config, &interface_and_impl_types)?;

			interface_defs.push(interface_token.to_string());

			for each_dep in dependencies.into_iter() {
				all_dependencies.insert(each_dep.to_string());
			}
		}

		let mut dest_file =
			OutputFile::new("interfaces.rs", FILE_HEADER_COMMENT.to_string(), output_dir);

		dest_file.add_content("use sdk::prelude::*;");

		let dependencies_token = dependency_strs_to_token(all_dependencies);

		dest_file.add_content(&dependencies_token.to_string());

		for each_obj_def in interface_defs {
			dest_file.add_content(&each_obj_def);
		}

		dest_file.create()?;

		Ok(true)
	}
}

struct InterfaceMember {
	pub member: TokenStream,
	pub dependencies: Vec<TokenStream>,
}

impl InterfaceMember {
	pub(crate) fn new(
		schema: &StructuredSchema,
		render_context: &RenderContext,
		member: &str,
	) -> Result<Self> {
		let member_type_name = format_ident!("{}", member);
		let member_enum_name = format_ident!("{}", member.to_camel_case());

		//TODO(tacogips) this conversion of interface member type to ValueTypeDef might be a bit hack-y?
		let member_type = ValueTypeDef::Named(NamedValue {
			value_type_name: member.to_string(),
			is_nullable: false,
		});
		let dependencies = member_type.dependency(schema, render_context)?;

		let member = quote! { #member_enum_name (#member_type_name) };

		Ok(InterfaceMember {
			member,
			dependencies,
		})
	}
}

fn find_implment_types_by_interface_type(
	structured_schema: &StructuredSchema,
) -> HashMap<String, Vec<String>> {
	let mut result = HashMap::<String, Vec<String>>::new();
	for each_obj in structured_schema.definitions.objects.values() {
		for interface_type in each_obj.impl_interface_name.iter() {
			let impl_types = result.entry(interface_type.to_string()).or_insert(vec![]);
			impl_types.push(each_obj.name.to_string());
		}
	}
	result
}
