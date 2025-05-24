use crate::OutputFile;

use super::{
	dependency_strs_to_token, separate_by_comma, sort_by_line_pos_and_name, LinePosition,
	NameString, NamedValue, RenderContext, StructuredSchema, TypeDef, ValueTypeDef,
	FILE_HEADER_COMMENT,
};
use anyhow::Result;
use heck::CamelCase;
use macros::{LinePosition, NameString};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct Union {
	pub name: String,
	//TODO() rename to concrete_type_names
	pub type_names: Vec<String>,
	pub line_pos: usize,
	pub description: Option<String>,
}

impl Union {
	fn token(&self, schema: &StructuredSchema) -> Result<(TokenStream, Vec<TokenStream>)> {
		let union_name = format_ident!("{}", self.name);

		let context = RenderContext {
			parent: TypeDef::Union(self),
		};

		let UnionFieldsInfo {
			members,
			dependencies,
		} = UnionFieldsInfo::new(schema, &context, self.type_names.iter().collect())?;

		let members = separate_by_comma(members);
		let union_def = quote! {

			#[derive(Union, Debug, Clone)]
			pub enum #union_name {
				#members
			}


		};
		Ok((union_def, dependencies))
	}

	pub(crate) fn write(structured_schema: &StructuredSchema, output_dir: &str) -> Result<bool> {
		let mut unions: Vec<&Self> =
			structured_schema.definitions.unions.values().into_iter().collect();
		if unions.is_empty() {
			return Ok(false);
		}
		unions.sort_by(sort_by_line_pos_and_name);

		let mut all_dependencies = HashSet::<String>::new();
		let mut union_defs = Vec::<String>::new();

		for each_union in unions {
			let (union_token, dependencies) = each_union.token(&structured_schema)?;

			union_defs.push(union_token.to_string());

			for each_dep in dependencies.into_iter() {
				all_dependencies.insert(each_dep.to_string());
			}
		}

		let mut dest_file =
			OutputFile::new("unions.rs", FILE_HEADER_COMMENT.to_string(), output_dir);

		dest_file.add_content("use sdk::prelude::*;");

		let dependencies_token = dependency_strs_to_token(all_dependencies);

		dest_file.add_content(&dependencies_token.to_string());

		for each_obj_def in union_defs {
			dest_file.add_content(&each_obj_def);
		}

		dest_file.create()?;

		Ok(true)
	}
}

pub struct UnionFieldsInfo {
	pub members: Vec<TokenStream>,
	pub dependencies: Vec<TokenStream>,
}

impl UnionFieldsInfo {
	pub(crate) fn new(
		schema: &StructuredSchema,
		context: &RenderContext,
		mut members: Vec<&String>,
	) -> Result<UnionFieldsInfo> {
		members.sort();
		let mut result = UnionFieldsInfo {
			members: Vec::new(),
			dependencies: Vec::new(),
		};
		for member in members.iter() {
			let UnionMember {
				member,
				mut dependencies,
			} = UnionMember::new(schema, context, member)?;

			result.members.push(member);

			result.dependencies.append(&mut dependencies);
		}
		Ok(result)
	}
}

struct UnionMember {
	pub member: TokenStream,
	pub dependencies: Vec<TokenStream>,
}

impl UnionMember {
	fn new(
		schema: &StructuredSchema,
		render_context: &RenderContext,
		member: &str,
	) -> Result<UnionMember> {
		let member_type_name = format_ident!("{}", member);
		let member_enum_name = format_ident!("{}", member.to_camel_case());

		//TODO(tacogips) this conversion of union member type to ValueTypeDef might be a bit hack-y?
		let member_type = ValueTypeDef::Named(NamedValue {
			value_type_name: member.to_string(),
			is_nullable: false,
		});
		let dependencies = member_type.dependency(schema, render_context)?;

		let member = quote! { #member_enum_name (#member_type_name) };

		Ok(UnionMember {
			member,
			dependencies,
		})
	}
}
