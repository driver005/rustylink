use super::{
	is_preserverd_type, Definitions, Enum, InputObject, Interface, NameString, Object,
	PrimitiveKind, RenderContext, Scalar, StructuredSchema, Union, PRIMITIVE_KIND_MAP,
};
use anyhow::{anyhow, Result};
use async_graphql_parser::types::{BaseType, Type};
use paste::paste;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, PartialEq)]
pub struct ListValue {
	pub inner: Box<ValueTypeDef>,
	pub is_nullable: bool,
}

#[derive(Debug, PartialEq)]
pub struct NamedValue {
	pub value_type_name: String,
	pub is_nullable: bool,
}

impl NamedValue {
	pub(crate) fn as_type_def<'a>(&self, definitions: &'a Definitions) -> Result<TypeDef<'a>> {
		let type_name = &self.value_type_name;
		//TODO(tacogips) what about in the case that object and input_object has same name?
		let result = if let Some(primitive) = PRIMITIVE_KIND_MAP.get(type_name.as_str()) {
			TypeDef::Primitive(primitive)
		} else if let Some(input_object) = definitions.input_objects.get(type_name) {
			TypeDef::InputObject(input_object)
		} else if let Some(object) = definitions.objects.get(type_name) {
			TypeDef::Object(object)
		} else if let Some(scalar) = definitions.scalars.get(type_name) {
			TypeDef::Scalar(scalar)
		} else if let Some(union) = definitions.unions.get(type_name) {
			TypeDef::Union(union)
		} else if let Some(enm) = definitions.enums.get(type_name) {
			TypeDef::Enum(enm)
		} else if let Some(interface) = definitions.interfaces.get(type_name) {
			TypeDef::Interface(interface)
		} else {
			if is_preserverd_type(type_name) {
				TypeDef::AsyncGraphqlPreserved(type_name.clone())
			} else {
				return Err(anyhow!("type: {} not defined", type_name));
			}
		};

		Ok(result)
	}
}

#[derive(Debug, PartialEq)]
pub enum ValueTypeDef {
	Named(NamedValue),
	List(ListValue),
}

impl ValueTypeDef {
	pub(crate) fn new(type_def: Type) -> Self {
		match type_def.base {
			BaseType::Named(name) => Self::Named(NamedValue {
				value_type_name: name.as_str().to_string(),
				is_nullable: type_def.nullable,
			}),
			BaseType::List(inner_type) => {
				let inner = Self::new(*inner_type);

				Self::List(ListValue {
					inner: Box::new(inner),
					is_nullable: type_def.nullable,
				})
			}
		}
	}

	pub(crate) fn is_list(&self) -> bool {
		match self {
			Self::Named(_) => false,
			Self::List(_) => true,
		}
	}

	pub(crate) fn nullable(&self) -> bool {
		match self {
			Self::Named(v) => v.is_nullable,
			Self::List(v) => v.is_nullable,
		}
	}

	pub(crate) fn element_value_type_def<'a>(
		&self,
		definitions: &'a Definitions,
	) -> Result<TypeDef<'a>> {
		match self {
			Self::Named(v) => v.as_type_def(&definitions),
			Self::List(v) => (*v.inner).element_value_type_def(&definitions),
		}
	}

	pub(crate) fn type_name(&self) -> String {
		match self {
			Self::Named(named_value) => named_value.value_type_name.clone(),
			Self::List(list_value) => list_value.inner.type_name(),
		}
	}

	pub(crate) fn source<'a>(&self, schema: &'a StructuredSchema) -> Result<TypeDef<'a>> {
		match self {
			Self::Named(named_value) => named_value.as_type_def(&schema.definitions),
			Self::List(list_value) => list_value.inner.source(&schema),
		}
	}

	pub(crate) fn dependency(
		&self,
		schema: &StructuredSchema,
		context: &RenderContext,
	) -> Result<Vec<TokenStream>> {
		let result = match self.source(schema)? {
			TypeDef::Primitive(_) => {
				return Ok(vec![]);
			}
			TypeDef::Object(object) => {
				if context.parent.is_object() {
					return Ok(vec![]);
				}
				let name = format_ident!("{}", object.name_string());
				quote! { use super::objects::#name }
			}
			TypeDef::Enum(enum_kind) => {
				if context.parent.is_enum() {
					return Ok(vec![]);
				}
				let name = format_ident!("{}", enum_kind.name_string());
				quote! { use super::enums::#name }
			}
			TypeDef::InputObject(input_object) => {
				if context.parent.is_input_object() {
					return Ok(vec![]);
				}
				let name = format_ident!("{}", input_object.name_string());
				quote! { use super::input_objects::#name }
			}
			TypeDef::Scalar(scalar) => {
				if context.parent.is_scalar() {
					return Ok(vec![]);
				}
				let name = format_ident!("{}", scalar.name_string());
				quote! { use super::scalars::#name }
			}
			TypeDef::Union(union) => {
				if context.parent.is_union() {
					return Ok(vec![]);
				}
				let name = format_ident!("{}", union.name_string());
				quote! { use super::unions::#name }
			}
			TypeDef::Interface(interface) => {
				if context.parent.is_interface() {
					return Ok(vec![]);
				}
				let name = format_ident!("{}", interface.name_string());
				quote! { use super::interfaces::#name }
			}

			TypeDef::AsyncGraphqlPreserved(_) => {
				// imported by use ayncgraphql::*;
				return Ok(vec![]);
			}
		};
		Ok(vec![result])
	}

	pub(crate) fn token(
		&self,
		schema: &StructuredSchema,
		render_context: &RenderContext,
	) -> Result<TokenStream> {
		let result = match self {
			Self::Named(named_value) => {
				let nullable = named_value.is_nullable;
				let type_def = named_value.as_type_def(&schema.definitions)?;
				let type_def = type_def.token(&render_context)?;
				if nullable {
					quote! { Option<#type_def > }
				} else {
					quote! { #type_def  }
				}
			}
			Self::List(list_value) => {
				let nullable = list_value.is_nullable;
				let inner_token = list_value.inner.token(schema, render_context)?;

				if nullable {
					quote! { Option<Vec<#inner_token>>}
				} else {
					quote! { Vec<#inner_token> }
				}
			}
		};
		Ok(result)
	}

	pub(crate) fn token_nullable(
		&self,
		schema: &StructuredSchema,
		render_context: &RenderContext,
	) -> Result<(TokenStream, bool)> {
		let result = match self {
			Self::Named(named_value) => {
				let nullable = named_value.is_nullable;
				let type_def = named_value.as_type_def(&schema.definitions)?;
				(type_def.token(&render_context)?, nullable)
			}
			Self::List(list_value) => {
				let nullable = list_value.is_nullable;
				let inner_token = list_value.inner.token(schema, render_context)?;

				(quote! { Vec<#inner_token> }, nullable)
			}
		};
		Ok(result)
	}
}

macro_rules! is {
	($v:ident) => {
		paste! {
			pub(crate) fn [<is_ $v:snake>] (&self) -> bool {
				if let TypeDef::$v(_) = self {
					true
				} else {
					false
				}
			}
		}
	};
}

#[derive(Debug)]
pub enum TypeDef<'a> {
	Primitive(&'a PrimitiveKind),
	Object(&'a Object),
	Enum(&'a Enum),
	InputObject(&'a InputObject),
	Scalar(&'a Scalar),
	Union(&'a Union),
	Interface(&'a Interface),
	AsyncGraphqlPreserved(String),
}
impl<'a> TypeDef<'a> {
	is! {Primitive}
	is! {Object}
	is! {Enum}
	is! {InputObject}
	is! {Scalar}
	is! {Union}
	is! {Interface}

	pub(crate) fn name(&self) -> String {
		match self {
			Self::Primitive(v) => v.rust_type(),
			Self::Object(v) => v.name.to_string(),
			Self::Enum(v) => v.name.to_string(),
			Self::InputObject(v) => v.name.to_string(),
			Self::Scalar(v) => v.name.to_string(),
			Self::Union(v) => v.name.to_string(),
			Self::Interface(v) => v.name.to_string(),
			Self::AsyncGraphqlPreserved(name) => name.clone(),
		}
	}

	fn token(&self, render_context: &'a RenderContext) -> Result<TokenStream> {
		let result = match self {
			Self::Primitive(primitive) => {
				let name = format_ident!("{}", primitive.rust_type());
				quote! { #name }
			}
			Self::Object(object) => {
				let recursive = if let Self::InputObject(parent) = render_context.parent {
					parent.name == object.name
				} else {
					false
				};

				let name: TokenStream = if recursive {
					format!("Box<{}>", object.name_string()).parse().unwrap()
				} else {
					format!("{}", object.name_string()).parse().unwrap()
				};

				quote! { #name }
			}
			Self::Enum(enum_kind) => {
				let name = format_ident!("{}", enum_kind.name_string());
				quote! { #name }
			}
			Self::InputObject(input_object) => {
				let recursive = if let Self::InputObject(parent) = render_context.parent {
					parent.name == input_object.name
				} else {
					false
				};

				let name: TokenStream = if recursive {
					format!("Box<{}>", input_object.name_string()).parse().unwrap()
				} else {
					format!("{}", input_object.name_string()).parse().unwrap()
				};
				quote! { #name }
			}
			Self::Scalar(scalar) => {
				let name = format_ident!("{}", scalar.name_string());
				quote! { #name }
			}
			Self::Union(union) => {
				let name = format_ident!("{}", union.name_string());
				quote! { #name }
			}
			Self::Interface(interface) => {
				let name = format_ident!("{}", interface.name_string());
				quote! { #name }
			}

			Self::AsyncGraphqlPreserved(type_name) => {
				let name = format_ident!("{}", type_name);
				quote! { #name }
			}
		};
		Ok(result)
	}
}
