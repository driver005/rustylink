use crate::graphql::FieldsSetting;
use anyhow::{anyhow, Result};
use async_graphql_parser::types::Type;
use heck::SnakeCase;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use strum::{AsRefStr, EnumString};

//CONSTS AND STATICS
pub(crate) const SUPPRESS_LINT: &str = r#"#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms, clippy::clone_on_copy, clippy::too_many_arguments)]"#;
pub(crate) const FILE_HEADER_COMMENT: &str = r#"
// DO NOT EDIT THIS FILE
// This file was generated by https://github.com/tacogips/async-graphql-reverse
"#;
lazy_static! {
	// ref. https://doc.rust-lang.org/reference/keywords.html
	pub(crate) static ref RUST_KEYWORDS: HashSet<&'static str> = {
		let mut s = HashSet::new();
		s.insert("as");
		s.insert("break");
		s.insert("const");
		s.insert("continue");
		s.insert("crate");
		s.insert("else");
		s.insert("enum");
		s.insert("extern");
		s.insert("false");
		s.insert("fn");
		s.insert("for");
		s.insert("if");
		s.insert("impl");
		s.insert("in");
		s.insert("let");
		s.insert("loop");
		s.insert("match");
		s.insert("mod");
		s.insert("move");
		s.insert("mut");
		s.insert("pub");
		s.insert("ref");
		s.insert("return");
		s.insert("self");
		s.insert("Self");
		s.insert("static");
		s.insert("struct");
		s.insert("super");
		s.insert("trait");
		s.insert("true");
		s.insert("type");
		s.insert("unsafe");
		s.insert("use");
		s.insert("where");
		s.insert("while");
		s.insert("async");
		s.insert("await");
		s.insert("dyn");
		s.insert("abstract");
		s.insert("become");
		s.insert("box");
		s.insert("do");
		s.insert("final");
		s.insert("macro");
		s.insert("override");
		s.insert("priv");
		s.insert("typeof");
		s.insert("unsized");
		s.insert("virtual");
		s.insert("yield");
		s.insert("try");
		s.insert("union");
		s.insert("dyn");
		s
	};
}

pub trait NameString {
	fn name_string(&self) -> String;
}

pub trait LinePosition {
	fn line_position(&self) -> usize;
}

pub trait SnakeCaseWithUnderscores: ToOwned {
	/// Convert this type to snake case without trimming leading / trailing underscores
	/// that might already be present on the string.
	fn to_snake_case_with_underscores(&self) -> Self::Owned;
}

impl SnakeCaseWithUnderscores for str {
	fn to_snake_case_with_underscores(&self) -> String {
		let leading_underscores: String = self.chars().take_while(|&c| c == '_').collect();
		let trailing_underscores: String = self.chars().rev().take_while(|&c| c == '_').collect();

		let trimmed = &self[leading_underscores.len()..self.len() - trailing_underscores.len()];

		format!("{}{}{}", leading_underscores, trimmed.to_snake_case(), trailing_underscores)
	}
}

pub(crate) fn dependency_strs_to_token(dependencies: HashSet<String>) -> TokenStream {
	merge_with_trailing_semicomman(
		dependencies
			.into_iter()
			.map(|dep| {
				let dep: TokenStream = dep.parse().unwrap();
				quote! {#dep}
			})
			.collect(),
	)
}

//TODO(tacogips) rename to  join_with_space
pub(crate) fn separate_by_space(tokens: Vec<TokenStream>) -> TokenStream {
	separate_tokens_by(tokens, " ")
}

//TODO(tacogips) rename to join_with_comma
pub(crate) fn separate_by_comma(tokens: Vec<TokenStream>) -> TokenStream {
	separate_tokens_by(tokens, ",")
}

//TODO(tacogips) rename to join_tokens_with
fn separate_tokens_by(mut tokens: Vec<TokenStream>, by: &'static str) -> TokenStream {
	if tokens.is_empty() {
		quote! {}
	} else {
		let by: TokenStream = by.parse().unwrap();
		let first = tokens.remove(0);
		let result = tokens.iter().fold(quote! { #first }, |acc, each| quote! {#acc #by #each});
		result
	}
}

pub(crate) fn merge_with_trailing_semicomman(mut tokens: Vec<TokenStream>) -> TokenStream {
	if tokens.is_empty() {
		quote! {}
	} else {
		let by: TokenStream = ";".parse().unwrap();
		let first = tokens.remove(0);
		let result = tokens.iter().fold(quote! {#first #by}, |acc, each| quote! {#acc #each #by });
		result
	}
}

pub(crate) fn sort_by_line_pos_and_name<T>(l: &T, r: &T) -> Ordering
where
	T: LinePosition + NameString,
{
	if l.line_position() == r.line_position() {
		l.name_string().cmp(&r.name_string())
	} else if l.line_position() < r.line_position() {
		Ordering::Less
	} else {
		Ordering::Greater
	}
}

pub(crate) fn to_rust_docs_token(comment: &str) -> TokenStream {
	let comments: Vec<TokenStream> = comment
		.split("\n")
		.into_iter()
		.map(|each_comment_line| format!("///{}", each_comment_line).parse().unwrap())
		.collect();

	let result = comments.into_iter().reduce(|acc, each| quote! {#acc #each });
	result.unwrap_or_else(|| quote! {})
}

#[derive(AsRefStr, EnumString, Debug)]
pub enum PrimitiveKind {
	#[strum(serialize = "Int")]
	Int,
	#[strum(serialize = "Float")]
	Float,
	#[strum(serialize = "String")]
	Str,
	#[strum(serialize = "Boolean")]
	Boolean,
	#[strum(serialize = "ID")]
	ID,
}

impl PrimitiveKind {
	pub(crate) fn rust_type(&self) -> String {
		match self {
			PrimitiveKind::Int => "i64".to_string(),
			PrimitiveKind::Float => "f64".to_string(),
			PrimitiveKind::Str => "String".to_string(),
			PrimitiveKind::Boolean => "bool".to_string(),
			PrimitiveKind::ID => "ID".to_string(),
		}
	}
}

lazy_static! {
	pub(crate) static ref PRIMITIVE_KIND_MAP: HashMap<&'static str, PrimitiveKind> = {
		let mut m = HashMap::new();
		m.insert(PrimitiveKind::Int.as_ref(), PrimitiveKind::Int);
		m.insert(PrimitiveKind::Float.as_ref(), PrimitiveKind::Float);
		m.insert(PrimitiveKind::Str.as_ref(), PrimitiveKind::Str);
		m.insert(PrimitiveKind::Boolean.as_ref(), PrimitiveKind::Boolean);
		m.insert(PrimitiveKind::ID.as_ref(), PrimitiveKind::ID);
		m
	};
	pub(crate) static ref PRESERVED_SCALARS: HashSet<&'static str> = {
		let mut s = HashSet::new();
		s.insert("Upload");
		s
	};
}

pub(crate) fn is_preserverd_type(type_name: &str) -> bool {
	PRESERVED_SCALARS.contains(&type_name)
}

pub(crate) fn setup_output_dir(output_dir: &str) -> Result<()> {
	let output_path = Path::new(output_dir);
	if output_path.exists() {
		let output_metadata = fs::metadata(output_dir)?;
		if !output_metadata.is_dir() {
			return Err(anyhow!("output path {} is not dir.", output_dir));
		}
	} else {
		fs::create_dir_all(output_dir)?;
	}
	Ok(())
}

pub(crate) fn maybe_replace_field(
	field_name: &str,
	fields_settings: Option<&FieldsSetting>,
) -> Result<Option<Type>> {
	if let Some(fields_settings) = fields_settings {
		if let Some(fields_setting) = fields_settings.get(field_name) {
			if let Some(replace_field_type) = &fields_setting.replace_field_type {
				let repalced_type = Type::new(&replace_field_type)
					.ok_or(anyhow!("invalid replace field type: {}", replace_field_type))?;
				return Ok(Some(repalced_type));
			}
		}
	}
	Ok(None)
}
