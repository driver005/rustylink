use serde::Deserialize;
use std::collections::HashMap;

// https://spec.graphql.org/June2018/#sec-Errors
#[derive(Deserialize, Debug)]
pub struct GraphQLErrorMessage {
	pub message: String,
	pub locations: Option<Vec<GraphQLErrorLocation>>,
	pub extensions: Option<HashMap<String, String>>,
	pub path: Option<Vec<GraphQLErrorPathParam>>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLErrorLocation {
	pub line: u32,
	pub column: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GraphQLErrorPathParam {
	String(String),
	Number(u32),
}
