use crate::{
	graphql::{RendererConfig, ResolverSetting},
	OutputFile,
};

use super::{
	dependency_strs_to_token, sort_by_line_pos_and_name, InputField, InputFieldsInfo, LinePosition,
	NameString, RenderContext, StructuredSchema, TypeDef, FILE_HEADER_COMMENT,
	{separate_by_comma, separate_by_space, to_rust_docs_token},
};
use anyhow::Result;
use macros::{LinePosition, NameString};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct InputObject {
	pub name: String,
	pub fields: Vec<InputField>,
	pub description: Option<String>,
	pub line_pos: usize,
}

impl InputObject {
	fn query_builder(&self, schema: &StructuredSchema) -> Result<TokenStream> {
		let mut to_query = Vec::new();

		for field in self.fields.iter() {
			let mut type_needed = false;
			let field_name = field.name();
			let fields = match field.typ.element_value_type_def(&schema.definitions)? {
				TypeDef::Primitive(_) => {
					quote! {None}
				}
				TypeDef::Object(object) => {
					if field.typ.nullable() {
						type_needed = true;
						if field.typ.is_list() {
							quote! {match #field_name.get(0) {
								Some(d) => Some(d.to_query()),
								None => None,
							}}
						} else {
							quote! {Some(#field_name.to_query())}
						}
					} else {
						let name = format_ident!("{}", object.name);
						quote! {Some(#name::to_query())}
					}
				}
				TypeDef::Enum(_) => {
					quote! {None}
				}
				TypeDef::InputObject(input_object) => {
					if field.typ.nullable() {
						type_needed = true;
						if field.typ.is_list() {
							quote! {match #field_name.get(0) {
								Some(d) => Some(d.to_query()),
								None => None,
							}}
						} else {
							quote! {Some(#field_name.to_query())}
						}
					} else {
						let name = format_ident!("{}", input_object.name);
						quote! {Some(#name::to_query())}
					}
				}
				TypeDef::Scalar(_) => {
					quote! {None}
				}
				TypeDef::Union(_) => {
					quote! {None}
				}
				TypeDef::Interface(interface) => {
					if field.typ.nullable() {
						type_needed = true;
						if field.typ.is_list() {
							quote! {match #field_name.get(0) {
								Some(d) => Some(d.to_query()),
								None => None,
							}}
						} else {
							quote! {Some(#field_name.to_query())}
						}
					} else {
						let name = format_ident!("{}", interface.name);
						quote! {Some(#name::to_query())}
					}
				}
				TypeDef::AsyncGraphqlPreserved(_) => quote! {None},
			};

			let name = &field.name;

			let selection = quote! {
				query.push(SelectionSet {
					operation: #name,
					alias: None,
					fields: #fields,
					arguments: None,
					is_union: false,
				});
			};

			if field.typ.nullable() {
				if type_needed {
					to_query.push(quote! {
						if let Some(#field_name) = &self.#field_name {
							#selection
						}
					});
				} else {
					to_query.push(quote! {
						if self.#field_name.is_some() {
							#selection
						}
					});
				}
			} else {
				to_query.push(selection);
			}
		}

		Ok(quote! {
			pub fn to_query(&self) -> Vec<SelectionSet> {
				let mut query = Vec::new();

				#(#to_query)*

				query
			}
		})
	}

	fn token(
		&self,
		schema: &StructuredSchema,
		render_config: &RendererConfig,
		resolver_setting: &HashMap<String, HashMap<String, &ResolverSetting>>,
	) -> Result<(TokenStream, Vec<TokenStream>)> {
		let object_name = format_ident!("{}", self.name);
		let comment = match &self.description {
			Some(desc_token) => to_rust_docs_token(desc_token),
			None => quote! {},
		};

		let context = RenderContext {
			parent: TypeDef::InputObject(self),
		};

		let field_resolver = resolver_setting.get(&self.name);

		let InputFieldsInfo {
			members,
			mut methods,
			dependencies,
		} = InputFieldsInfo::new(self.fields.iter().collect(), schema, &context, field_resolver)?;

		let members = separate_by_comma(members);

		methods.push(self.query_builder(schema)?);

		let methods = separate_by_space(methods);

		let methods = match render_config.no_object_impl {
			true => quote! {},
			false => quote! {
				impl #object_name {
					#methods
				}
			},
		};

		let additional_attributes = match &render_config.additional_attributes {
			Some(attributes) => format!("{},", attributes).parse::<TokenStream>().unwrap(),
			None => TokenStream::new(),
		};

		let object_def = quote! {
			#comment
			#[derive(#additional_attributes Debug, Clone)]
			pub struct #object_name{
				#members
			}

			#methods
		};
		Ok((object_def, dependencies))
	}

	pub(crate) fn write(
		structured_schema: &StructuredSchema,
		render_config: &RendererConfig,
		output_dir: &str,
	) -> Result<bool> {
		let mut input_objects: Vec<&Self> =
			structured_schema.definitions.input_objects.values().into_iter().collect();
		if input_objects.is_empty() {
			return Ok(false);
		}

		input_objects.sort_by(sort_by_line_pos_and_name);

		let mut all_dependencies = HashSet::<String>::new();
		let mut object_defs = Vec::<String>::new();

		let resolver_setting = render_config.resolver_setting();

		for each_obj in input_objects {
			let (object_token, dependencies) =
				each_obj.token(&structured_schema, render_config, &resolver_setting)?;

			object_defs.push(object_token.to_string());

			for each_dep in dependencies.into_iter() {
				all_dependencies.insert(each_dep.to_string());
			}
		}

		let mut dest_file =
			OutputFile::new("input_objects.rs", FILE_HEADER_COMMENT.to_string(), output_dir);

		dest_file.add_content("use sdk::prelude::*;");

		let dependencies_token = dependency_strs_to_token(all_dependencies);

		dest_file.add_content(&dependencies_token.to_string());

		for each_obj_def in object_defs {
			dest_file.add_content(&each_obj_def);
		}

		dest_file.create()?;

		Ok(true)
	}
}

#[cfg(test)]
mod test {

	use super::*;
	use crate::graphql::{parse_schema, RendererConfig};

	#[test]
	pub fn parse_input_1() {
		let config = RendererConfig::default();
		let schema = r#"
        input SampleInput {
          id: String
          rec:[Int],
        }
        "#;

		let structured_schema = parse_schema(schema, &config).unwrap();

		let resolver_setting = config.resolver_setting();

		let mut input_objects: Vec<&InputObject> =
			structured_schema.definitions.input_objects.values().into_iter().collect();
		assert_eq!(1, input_objects.len());
		let input_object = input_objects.remove(0);
		let (object_token, _dependencies) =
			input_object.token(&structured_schema, &config, &resolver_setting).unwrap();

		let expected = r#"
    #[derive(InputObject)]
    pub struct SampleInput{
        pub id:Option<String>,
        pub rec:Option<Vec<Option<i64>>>}
"#;
		assert_eq!(
			object_token.to_string().replace(" ", ""),
			expected.to_string().replace("\n", "").replace(" ", "")
		);
	}

	#[test]
	pub fn parse_input_2() {
		let config = RendererConfig::default();
		let schema = r#"
        input SampleInput {
          id: String
          rec:[Int]!,
        }
        "#;

		let structured_schema = parse_schema(schema, &RendererConfig::default()).unwrap();
		let resolver_setting = config.resolver_setting();

		let mut input_objects: Vec<&InputObject> =
			structured_schema.definitions.input_objects.values().into_iter().collect();
		assert_eq!(1, input_objects.len());
		let input_object = input_objects.remove(0);
		let (object_token, _dependencies) =
			input_object.token(&structured_schema, &config, &resolver_setting).unwrap();

		let expected = r#"
    #[derive(InputObject)]
    pub struct SampleInput{
        pub id:Option<String>,
        pub rec:Vec<Option<i64>>}
"#;
		assert_eq!(
			object_token.to_string().replace(" ", ""),
			expected.to_string().replace("\n", "").replace(" ", "")
		);
	}

	#[test]
	pub fn parse_input_3() {
		let config = RendererConfig::default();
		let schema = r#"
        input SampleInput {
          id: String
          rec:[Int!]!,
        }
        "#;

		let structured_schema = parse_schema(schema, &config).unwrap();
		let resolver_setting = config.resolver_setting();

		let mut input_objects: Vec<&InputObject> =
			structured_schema.definitions.input_objects.values().into_iter().collect();
		assert_eq!(1, input_objects.len());
		let input_object = input_objects.remove(0);
		let (object_token, _dependencies) =
			input_object.token(&structured_schema, &config, &resolver_setting).unwrap();

		let expected = r#"
    #[derive(InputObject)]
    pub struct SampleInput{
        pub id:Option<String>,
        pub rec:Vec<i64>}
"#;
		assert_eq!(
			object_token.to_string().replace(" ", ""),
			expected.to_string().replace("\n", "").replace(" ", "")
		);
	}
}
