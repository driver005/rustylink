use crate::OutputFile;

use super::{
	sort_by_line_pos_and_name, LinePosition, NameString, StructuredSchema, FILE_HEADER_COMMENT,
};
use anyhow::Result;
use macros::{LinePosition, NameString};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, NameString, LinePosition, PartialEq)]
pub struct Scalar {
	pub name: String,
	pub line_pos: usize,
}

impl Scalar {
	fn token(&self) -> Result<TokenStream> {
		let scalar_name = format_ident!("{}", self.name);
		let scalar_def = quote! {

		#[derive(Debug, Clone)]
		pub struct #scalar_name(pub String);
		#[Scalar]
		impl ScalarType for #scalar_name {
			fn parse(value: Value) -> InputValueResult<Self> {
				match value {
					Value::String(s) => Ok( #scalar_name(s)),
					_ => Err(InputValueError::expected_type(value)),

				}
			}
			fn to_value(&self) -> Value {
				Value::String(self.0.to_string())
			}
		}


		};
		Ok(scalar_def)
	}
	pub(crate) fn write(structured_schema: &StructuredSchema, output_dir: &str) -> Result<bool> {
		let mut scalars: Vec<&Self> =
			structured_schema.definitions.scalars.values().into_iter().collect();
		if scalars.is_empty() {
			return Ok(false);
		}
		scalars.sort_by(sort_by_line_pos_and_name);

		let mut scalar_defs = Vec::<String>::new();

		for each_scalar in scalars {
			let scalar_token = each_scalar.token()?;
			scalar_defs.push(scalar_token.to_string());
		}

		let mut dest_file =
			OutputFile::new("scalars.rs", FILE_HEADER_COMMENT.to_string(), output_dir);

		dest_file.add_content("use sdk::prelude::*;");

		for each_obj_def in scalar_defs {
			dest_file.add_content(&each_obj_def);
		}

		dest_file.create()?;

		Ok(true)
	}
}
