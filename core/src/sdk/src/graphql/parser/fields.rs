use super::{
	maybe_replace_field, node_as_string, separate_by_comma, sort_by_line_pos_and_name,
	to_rust_docs_token, Argument, Definitions, LinePosition, NameString, PrimitiveKind,
	RenderContext, SnakeCaseWithUnderscores, StructuredSchema, TypeDef, ValueTypeDef,
	RUST_KEYWORDS,
};
use crate::graphql::{FieldsResolverSetting, FieldsSetting, RendererConfig, ResolverSetting};
use anyhow::Result;
use async_graphql_parser::{types::FieldDefinition, Positioned};
use macros::{LinePosition, NameString};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashMap, str::FromStr};
use strum::EnumString;

pub struct FieldsInfo {
	pub members: Vec<TokenStream>,
	pub methods: Vec<TokenStream>,
	pub dependencies: Vec<TokenStream>,
}

impl FieldsInfo {
	pub(crate) fn new(
		mut fields: Vec<&Field>,
		schema: &StructuredSchema,
		config: &RendererConfig,
		context: &RenderContext,
		resolver_settings: Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<Self> {
		fields.sort_by(sort_by_line_pos_and_name);
		let mut result = Self {
			members: vec![],
			methods: vec![],
			dependencies: vec![],
		};
		for field in fields.iter() {
			let MemberAndMethod {
				member,
				method,
				mut dependencies,
			} = field.convert(schema, context, config, &resolver_settings)?;

			if let Some(member) = member {
				result.members.push(member)
			}

			if let Some(method) = method {
				result.methods.push(method);
			}

			result.dependencies.append(&mut dependencies);
		}

		Ok(result)
	}
}

struct MemberAndMethod {
	pub member: Option<TokenStream>,
	pub method: Option<TokenStream>,
	pub dependencies: Vec<TokenStream>,
}

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct Field {
	pub name: String,
	pub description: Option<String>,
	pub typ: ValueTypeDef,
	pub arguments: Vec<Argument>,
	pub line_pos: usize,
}

impl Field {
	pub(crate) fn new(
		field_def: &Positioned<FieldDefinition>,
		fields_setting: Option<&FieldsSetting>,
		fields_resolver_setting: Option<&FieldsResolverSetting>,
	) -> Self {
		let line_pos = field_def.pos.line;
		let field_def = field_def.node.clone();

		if !field_def.directives.is_empty() {
			log::warn!("directive is not supported yet, {}", node_as_string!(field_def.name));
		}

		let mut arguments: Vec<Argument> =
			field_def.arguments.iter().map(|arg| Argument::new(arg)).collect();
		let field_name = &node_as_string!(field_def.name);
		if let Some(fields_resolver_setting) = fields_resolver_setting {
			if let Some(resolver_setting) = fields_resolver_setting.get(field_name) {
				if let Some(args) = &resolver_setting.argument {
					let mut additional_args: Vec<Argument> =
						args.iter().map(|arg| Argument::new_from_config_arg(arg)).collect();
					arguments.append(&mut additional_args);
				}
			}
		}

		let field_type = match maybe_replace_field(field_name, fields_setting).unwrap() {
			Some(replaced_field) => replaced_field,
			None => field_def.ty.node,
		};

		Self {
			name: node_as_string!(field_def.name),
			description: field_def.description.map(|desc| node_as_string!(desc)),
			typ: ValueTypeDef::new(field_type),
			arguments,
			line_pos,
		}
	}

	pub(crate) fn new_from_list(
		fields: &Vec<Positioned<FieldDefinition>>,
		fields_setting: Option<&FieldsSetting>,
		fields_resolver_setting: Option<&FieldsResolverSetting>,
	) -> Vec<Self> {
		fields
			.iter()
			.map(|field| Self::new(field, fields_setting, fields_resolver_setting))
			.collect()
	}

	/// Returns Some for the second element if the field was renamed. Otherwise, returns None.
	fn name(&self) -> (Ident, Option<String>) {
		let field_name: String = self.name_string().to_snake_case_with_underscores().into();
		if field_name.to_lowercase() == "self" {
			(format_ident!("{}_", field_name), Some(field_name))
		} else if RUST_KEYWORDS.contains(&field_name.as_ref()) {
			(format_ident!("r#{}", field_name), None)
		} else {
			(format_ident!("{}", field_name), None)
		}
	}

	fn convert(
		&self,
		schema: &StructuredSchema,
		render_context: &RenderContext,
		renderer_config: &RendererConfig,
		resolver_settings: &Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<MemberAndMethod> {
		match self.get_resolver_type(
			&schema,
			&render_context,
			&renderer_config,
			&resolver_settings,
		)? {
			ResolverType::Field => self.field(schema, render_context, &resolver_settings),
			resolver_type => self.method(schema, render_context, resolver_type, &resolver_settings),
		}
	}

	fn get_resolver_type(
		&self,
		schema: &StructuredSchema,
		render_context: &RenderContext,
		renderer_config: &RendererConfig,
		resolver_settings: &Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<ResolverType> {
		// First check for specific overrides.
		if let Some(field_resolver) = resolver_settings {
			if let Some(resolver_type) =
				resolver_type_in_resolver_setting(&self.name, &field_resolver)
			{
				return Ok(resolver_type);
			}
		}

		// Now check if there is a default setting.
		if let Some(resolver_type) = get_default_resolver_type(&renderer_config) {
			return Ok(resolver_type);
		}

		if let TypeDef::Object(object) = render_context.parent {
			//TODO(tacogips) more customize if needed
			if schema.is_query(&object.name) {
				return Ok(ResolverType::Query);
			} else if schema.is_mutation(&object.name) {
				return Ok(ResolverType::Mutation);
			}
		}

		Ok(ResolverType::Field)
	}

	fn field(
		&self,
		schema: &StructuredSchema,
		context: &RenderContext,
		resolver_settings: &Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<MemberAndMethod> {
		let (name, old_name) = self.name();
		let typ = self.typ.token(&schema, &context)?;

		// Handle field names that cannot use `r#`, such as `self`.
		let field_attribute = match old_name {
			Some(old_name) => {
				quote! {
					#[serde(rename = #old_name)]
				}
			}
			None => {
				let filed_name = &self.name;
				quote! {
					#[serde(rename = #filed_name)]
				}
			}
		};

		let member = Some(quote! { #field_attribute pub #name :#typ });

		let attribute = match get_attribute_from_resolver_settings(&self.name, resolver_settings) {
			Some(attribute) => attribute.to_token_stream(),
			None => quote! {},
		};
		let field_rustdoc = match &self.description {
			Some(desc_token) => to_rust_docs_token(desc_token),
			None => quote! {},
		};

		let member_need_clone = if let ValueTypeDef::Named(typ) = &self.typ {
			let type_def = typ.as_type_def(&schema.definitions).unwrap();
			match type_def {
				TypeDef::Primitive(PrimitiveKind::Int)
				| TypeDef::Primitive(PrimitiveKind::Float)
				| TypeDef::Primitive(PrimitiveKind::Boolean) => false,
				_ => true,
			}
		} else {
			true
		};

		let resolver_body = if member_need_clone {
			quote! { self.#name.clone() }
		} else {
			quote! { self.#name }
		};

		let method = Some(quote! {
			#field_rustdoc
			#attribute
			pub async fn #name(&self) -> #typ  {
				#resolver_body
			}
		});

		let dependencies = self.typ.dependency(schema, context)?;

		Ok(MemberAndMethod {
			member,
			method,
			dependencies,
		})
	}

	fn args_defs(
		&self,
		schema: &StructuredSchema,
		context: &RenderContext,
		name_prefix: &str,
	) -> Result<TokenStream> {
		if self.arguments.is_empty() {
			Ok(quote! {})
		} else {
			let arg_defs = self
				.arguments
				.iter()
				.map(|argument| argument.token(&schema, &context, &name_prefix))
				.collect::<Result<Vec<TokenStream>>>()?;
			let arg_defs = separate_by_comma(arg_defs);

			Ok(quote! {,#arg_defs})
		}
	}

	fn query(&self, definition: &Definitions) -> Result<TokenStream> {
		let mut arguments = Vec::new();
		for argument in self.arguments.iter() {
			let name = argument.name_string();
			let field_name =
				format_ident!("{}", argument.name_string().to_snake_case_with_underscores());

			let selection = quote! {
				fileds.insert(#name.to_owned(), serde_json::to_value(&#field_name)?);
			};

			if argument.typ.nullable() {
				arguments.push(quote! {
					if let Some(#field_name) = #field_name {
						#selection
					}
				});
			} else {
				arguments.push(selection);
			}
		}

		let name = self.name_string();

		let argument = if !arguments.is_empty() {
			quote! {
				Some(serde_json::Value::Object(fileds))
			}
		} else {
			quote! {None}
		};

		let query = if !arguments.is_empty() {
			quote! {
				let mut fileds = serde_json::Map::new();
			}
		} else {
			quote! {}
		};

		let fields = match get_field_return_type(&self.typ, definition)? {
			Some(val) => quote! {Some(#val::to_query())},
			None => quote! {None},
		};

		Ok(quote! {
			#query

			#(#arguments)*

			let data = SelectionSet {
				operation: #name,
				alias: None,
				fields: #fields,
				arguments: #argument,
				is_union: false,
			};
		})
	}

	fn method(
		&self,
		schema: &StructuredSchema,
		context: &RenderContext,
		resolver_type: ResolverType,
		resolver_settings: &Option<&HashMap<String, &ResolverSetting>>,
	) -> Result<MemberAndMethod> {
		let arg_defs = self.args_defs(&schema, &context, "")?;

		let (field_name, _old_name) = self.name();

		let attribute = match get_attribute_from_resolver_settings(&self.name, resolver_settings) {
			Some(attribute) => attribute.to_token_stream(),
			None => quote! {},
		};

		let field_rustdoc = match &self.description {
			Some(desc_token) => to_rust_docs_token(desc_token),
			None => quote! {},
		};

		let typ = self.typ.token(&schema, &context)?;
		let result_typ: TokenStream = quote! {Result<#typ>};
		let query = self.query(&schema.definitions)?;

		let operation_type = match resolver_type {
			ResolverType::Mutation => quote! {OperationType::Mutation},
			ResolverType::Query => quote! {OperationType::Query},
			ResolverType::Field => unreachable!(),
		};

		let function_name = self.name_string();

		let method = quote! {
			#field_rustdoc
			#attribute
			pub async fn #field_name(&self #arg_defs ) -> #result_typ { // -> #typ, ctx: &Context<'_>
				#query
				let query = QueryBuilder::new(#operation_type, &data);
				self.client.request::<#typ>(&query, #function_name).await
			}
		};

		let mut dependencies = self.typ.dependency(schema, context)?;

		for argument in self.arguments.iter() {
			let mut each_deps = argument.typ.dependency(schema, context)?;
			dependencies.append(&mut each_deps);
		}

		Ok(MemberAndMethod {
			member: None,
			method: Some(method),
			dependencies,
		})
	}
}

#[derive(Eq, PartialEq, Debug, EnumString)]
pub enum ResolverType {
	#[strum(serialize = "mutation")]
	Mutation,
	#[strum(serialize = "query")]
	Query,
	#[strum(serialize = "field")]
	Field,
}

fn resolver_type_in_resolver_setting(
	field_name: &str,
	resolver_field_setting: &HashMap<String, &ResolverSetting>,
) -> Option<ResolverType> {
	match resolver_field_setting.get(field_name) {
		Some(ResolverSetting {
			resolver_type,
			..
		}) => resolver_type.as_ref().map(|typ| ResolverType::from_str(typ).unwrap()),
		None => None,
	}
}

fn get_default_resolver_type(renderer_config: &RendererConfig) -> Option<ResolverType> {
	match &renderer_config.resolver_type {
		Some(resolver_type) => Some(ResolverType::from_str(resolver_type).unwrap()),
		None => None,
	}
}

pub(crate) fn get_attribute_from_resolver_settings(
	field_name: &str,
	resolver_settings: &Option<&HashMap<String, &ResolverSetting>>,
) -> Option<String> {
	if let Some(field_resolver) = resolver_settings {
		if let Some(resolver_sertting) = field_resolver.get(field_name) {
			return resolver_sertting.attribute.clone();
		}
	}
	return None;
}

fn get_field_return_type(
	typ: &ValueTypeDef,
	definition: &Definitions,
) -> Result<Option<TokenStream>> {
	match typ {
		ValueTypeDef::Named(named_value) => match named_value.as_type_def(&definition)? {
			//TODO: check other types
			TypeDef::Object(_) => {
				let name = format_ident!("{}", named_value.value_type_name);
				Ok(Some(quote! {#name}))
			}
			_ => Ok(None),
		},
		ValueTypeDef::List(list_value) => get_field_return_type(&list_value.inner, definition),
	}
}
