mod argument;
// mod comment;
// mod dependencies;
mod enums;
mod fields;
mod input_fields;
mod input_objects;
mod interfaces;
// mod keywords;
// mod linter;
mod objects;
mod scalars;
// mod sorter;
// mod tokens;
mod schema;
mod typ;
mod unions;
mod utils;

macro_rules! node_as_string {
	($variable:expr) => {
		$variable.node.as_str().to_string()
	};
}

pub use argument::*;
pub use enums::*;
pub use fields::*;
pub use input_fields::*;
pub use input_objects::*;
pub use interfaces::*;
use node_as_string;
pub use objects::*;
pub use scalars::*;
pub use schema::*;
pub use typ::*;
pub use unions::*;
pub use utils::*;

use super::RendererConfig;
use anyhow::{anyhow, Result};
use std::fs;

pub fn parse_schema(schema_body: &str, config: &RendererConfig) -> Result<StructuredSchema> {
	match async_graphql_parser::parse_schema(schema_body) {
		Ok(schema) => StructuredSchema::new(schema, config),
		Err(e) => Err(anyhow!("{}", e)),
	}
}

pub fn parse_schema_file(path: &str, config: &RendererConfig) -> Result<StructuredSchema> {
	match fs::read_to_string(path) {
		Ok(mut schema_body) => {
			if let Some(additionals) = &config.additional {
				let merged_additional = additionals
					.iter()
					.map(|each| each.body.to_string())
					.collect::<Vec<String>>()
					.join(" ");

				schema_body = format!("{} {}", schema_body, merged_additional);
			}

			let mut schema = parse_schema(&schema_body, config)?;

			schema.remove_ignored(&config)?;
			Ok(schema)
		}
		Err(e) => Err(anyhow!("{}", e)),
	}
}
