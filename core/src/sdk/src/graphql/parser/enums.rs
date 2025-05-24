use super::{
	node_as_string,
	utils::{separate_by_comma, sort_by_line_pos_and_name, FILE_HEADER_COMMENT},
	LinePosition, NameString, StructuredSchema,
};
use crate::{
	graphql::{EnumSetting, EnumValueSetting, RendererConfig},
	OutputFile,
};
use anyhow::Result;
use async_graphql_parser::{types::EnumValueDefinition, Positioned};
use heck::CamelCase;
use macros::{LinePosition, NameString};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct Enum {
	pub name: String,
	pub values: Vec<EnumValue>,
	pub line_pos: usize,
	pub description: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct EnumValue {
	pub value_name: String,
	pub description: Option<String>,
}

impl EnumValue {
	pub(crate) fn new(enum_def: &Positioned<EnumValueDefinition>) -> EnumValue {
		let enum_def = enum_def.node.clone();

		if !enum_def.directives.is_empty() {
			log::warn!("directive is not supported yet, {}", node_as_string!(enum_def.value));
		}

		EnumValue {
			value_name: node_as_string!(enum_def.value),
			description: enum_def.description.map(|desc| node_as_string!(desc)),
		}
	}
}

impl Enum {
	fn token(
		&self,
		config: &RendererConfig,
		enum_settings: &HashMap<String, EnumSetting>,
	) -> Result<TokenStream> {
		let enum_name = self.name.to_camel_case();
		let mut graphql_derive = quote! {};

		// TODO(tacogips) using there_is_specific_rename_item is naive implementation. make this concise with macro or something
		let mut there_is_specific_rename_item = false;
		let mut enum_value_settings = HashMap::<String, &EnumValueSetting>::default();
		if let Some(specific_enum_setting) = enum_settings.get(&enum_name) {
			if let Some(specifig_rename_items) = &specific_enum_setting.rename_items {
				there_is_specific_rename_item = true;
				graphql_derive = quote! {
					#[graphql(rename_items = #specifig_rename_items)]
				}
			}

			if let Some(specific_enum_setting_value) = specific_enum_setting.value.as_ref() {
				enum_value_settings = specific_enum_setting_value
					.iter()
					.map(|each| (each.value.to_string(), each))
					.collect();
			}
		}

		let enums_members: Vec<TokenStream> = self
			.values
			.iter()
			.map(|each_enum_value| {
				//each_enum.value_name.parse::<TokenStream>().unwrap()}
				let enum_value_name = each_enum_value.value_name.to_camel_case();
				let each_enum = format_ident!("{}", enum_value_name);

				let enum_attribute = match enum_value_settings.get(&enum_value_name) {
					Some(each_enum_setting) => match &each_enum_setting.rename {
						Some(rename) => {
							quote! {
								#[graphql(name = #rename)]
							}
						}
						None => quote! {},
					},
					None => quote! {},
				};

				quote! {
					#enum_attribute
					#each_enum
				}
			})
			.collect();

		if !there_is_specific_rename_item {
			if let Some(enum_rename_items) = config.enum_rename_items.as_ref() {
				graphql_derive = quote! {
						#[graphql(rename_items = #enum_rename_items)]
				}
			}
		}

		let additional_attributes = match &config.additional_attributes {
			Some(attributes) => format!("{},", attributes).parse::<TokenStream>().unwrap(),
			None => TokenStream::new(),
		};

		let enum_name = format_ident!("{}", enum_name);
		let enum_members = separate_by_comma(enums_members);

		let enum_def = quote! {

			#[derive(#additional_attributes Copy, Clone, Debug, Eq, PartialEq)]
			#graphql_derive
			pub enum #enum_name{
				#enum_members
			}


		};
		Ok(enum_def)
	}

	pub(crate) fn write(
		schema: &StructuredSchema,
		config: &RendererConfig,
		output_dir: &str,
	) -> Result<bool> {
		let mut enums: Vec<&Self> = schema.definitions.enums.values().into_iter().collect();
		if enums.is_empty() {
			return Ok(false);
		}
		enums.sort_by(sort_by_line_pos_and_name);

		let mut enum_defs = Vec::<String>::new();

		let enum_settings = config.enum_settings();

		for each_enum in enums {
			let token = each_enum.token(config, &enum_settings)?;
			enum_defs.push(token.to_string());
		}

		let mut dest_file =
			OutputFile::new("enums.rs", FILE_HEADER_COMMENT.to_string(), output_dir);

		dest_file.add_content("use sdk::prelude::*;");

		for each_obj_def in enum_defs {
			dest_file.add_content(&each_obj_def);
		}

		dest_file.create()?;

		Ok(true)
	}
}
