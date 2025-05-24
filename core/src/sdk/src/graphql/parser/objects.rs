use super::{
	dependency_strs_to_token, separate_by_comma, separate_by_space, sort_by_line_pos_and_name,
	to_rust_docs_token, Field, FieldsInfo, LinePosition, NameString, RenderContext,
	StructuredSchema, TypeDef, FILE_HEADER_COMMENT,
};
use crate::{
	graphql::{CustomResolvers, HiddenFields, RendererConfig, ResolverSetting},
	OutputFile,
};
use anyhow::Result;
use macros::{LinePosition, NameString};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct Object {
	pub name: String,
	pub fields: Vec<Field>,
	pub description: Option<String>,
	pub line_pos: usize,
	pub impl_interface_name: Vec<String>,
}

impl Object {
	fn query_builder(&self, schema: &StructuredSchema) -> Result<TokenStream> {
		let mut to_query = Vec::new();

		for field in self.fields.iter() {
			let fields = match field.typ.element_value_type_def(&schema.definitions)? {
				TypeDef::Primitive(_) => {
					quote! {None}
				}
				TypeDef::Object(object) => {
					let name = format_ident!("{}", object.name);
					quote! {Some(#name::to_query())}
				}
				TypeDef::Enum(_) => {
					quote! {None}
				}
				TypeDef::InputObject(input_object) => {
					let name = format_ident!("{}", input_object.name);
					quote! {Some(#name::to_query())}
				}
				TypeDef::Scalar(_) => {
					quote! {None}
				}
				TypeDef::Union(_) => {
					quote! {None}
				}
				TypeDef::Interface(interface) => {
					let name = format_ident!("{}", interface.name);
					quote! {Some(#name::to_query())}
				}
				TypeDef::AsyncGraphqlPreserved(_) => quote! {None},
			};

			let name = &field.name;

			to_query.push(quote! {
				SelectionSet {
					operation: #name,
					alias: None,
					fields: #fields,
					arguments: None,
					is_union: false,
				},
			});
		}

		Ok(quote! {
			pub fn to_query() -> Vec<SelectionSet> {
				vec![
					#(#to_query)*
				]
			}
		})
	}

	fn token(
		&self,
		schema: &StructuredSchema,
		render_config: &RendererConfig,
		resolver_setting: &HashMap<String, HashMap<String, &ResolverSetting>>,
		additional_resolvers: &HashMap<String, CustomResolvers>,
		hidden_fields: &HashMap<String, HiddenFields>,
	) -> Result<(TokenStream, Vec<TokenStream>)> {
		let object_name = format_ident!("{}", self.name);
		let comment = match &self.description {
			Some(desc_token) => to_rust_docs_token(desc_token),
			None => quote! {},
		};

		let context = RenderContext {
			parent: TypeDef::Object(self),
		};

		let field_resolver = resolver_setting.get(&self.name);

		let FieldsInfo {
			mut members,
			mut methods,
			mut dependencies,
		} = FieldsInfo::new(
			self.fields.iter().collect(),
			schema,
			render_config,
			&context,
			field_resolver,
		)?;

		if let Some(additional_resolvers) = additional_resolvers.get(&self.name) {
			let mut bodies: Vec<TokenStream> = additional_resolvers
				.bodies
				.iter()
				.map(|e| e.parse::<TokenStream>().unwrap())
				.collect();

			let mut usings: Vec<TokenStream> = additional_resolvers
				.using
				.iter()
				.map(|e| e.parse::<TokenStream>().unwrap())
				.collect();

			methods.append(&mut bodies);
			dependencies.append(&mut usings);
		}

		if let Some(hidden_fields) = hidden_fields.get(&self.name) {
			let mut defs: Vec<TokenStream> = hidden_fields
				.field_defs
				.iter()
				.map(|e| e.parse::<TokenStream>().unwrap())
				.collect();

			let mut usings: Vec<TokenStream> =
				hidden_fields.using.iter().map(|e| e.parse::<TokenStream>().unwrap()).collect();

			members.append(&mut defs);
			dependencies.append(&mut usings);
		}

		let mut additional_attributes = match &render_config.additional_attributes {
			Some(attributes) => format!("{},", attributes).parse::<TokenStream>().unwrap(),
			None => TokenStream::new(),
		};

		let mut lifeline = quote! {};

		if let TypeDef::Object(object) = context.parent {
			//TODO(tacogips) more customize if needed
			if schema.is_query(&object.name) || schema.is_mutation(&object.name) {
				additional_attributes = quote! {};
				lifeline = quote! {<'a>};
				members.push(quote! {
					pub client: GraphQLClient<'a>
				});
			} else {
				methods.push(object.query_builder(schema)?);
			}
		}

		let members = separate_by_comma(members);
		let methods = separate_by_space(methods);

		let methods = match render_config.no_object_impl {
			true => quote! {},
			false => quote! {
				impl #lifeline #object_name #lifeline {
					#methods
				}
			},
		};

		let object_def = quote! {
			#comment

			#[derive(#additional_attributes Debug, Clone)]
			pub struct #object_name #lifeline{
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
		let mut objects: Vec<&Self> =
			structured_schema.definitions.objects.values().into_iter().collect();
		if objects.is_empty() {
			return Ok(false);
		}
		objects.sort_by(sort_by_line_pos_and_name);

		let mut all_dependencies = HashSet::<String>::new();
		let mut object_defs = Vec::<String>::new();

		let resolver_setting = render_config.resolver_setting();
		let additional_resolvers = render_config.additional_resolvers();
		let hidden_fields = render_config.hidden_fields();

		for each_obj in objects {
			let (object_token, dependencies) = each_obj.token(
				&structured_schema,
				render_config,
				&resolver_setting,
				&additional_resolvers,
				&hidden_fields,
			)?;

			object_defs.push(object_token.to_string());

			for each_dep in dependencies.into_iter() {
				all_dependencies.insert(each_dep.to_string());
			}
		}

		let mut dest_file =
			OutputFile::new("objects.rs", FILE_HEADER_COMMENT.to_string(), output_dir);

		dest_file.add_content(&render_config.header);

		if !render_config.no_dependency_imports {
			let dependencies_token = dependency_strs_to_token(all_dependencies);
			dest_file.add_content(&dependencies_token.to_string());
		}

		for each_obj_def in object_defs {
			dest_file.add_content(&each_obj_def);
		}

		dest_file.create()?;

		Ok(true)
	}
}
