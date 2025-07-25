use core::fmt;
use proc_macro2::TokenStream;
use quote::quote;
use std::{error::Error, fs, io::Write, path::Path, process::Command, str::FromStr};

#[derive(Debug)]
pub enum DateTimeCrate {
	Chrono,
	Time,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WebFrameworkEnum {
	Actix,
	Poem,
	Axum,
}

#[derive(Debug, Clone)]
pub enum GeneratorType {
	Database,
	Api,
	Proto,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
	Get,
	List,
	Create,
	Update,
	Delete,
	Response,
	Custom(String),
}

impl Operation {
	pub fn create() -> Vec<Operation> {
		vec![
			Operation::Get,
			Operation::List,
			Operation::Create,
			Operation::Update,
			Operation::Delete,
			Operation::Response,
		]
	}
}

impl fmt::Display for Operation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub struct WriterOutput {
	pub files: Vec<OutputFile>,
}

pub struct OutputFile {
	pub name: String,
	pub content: String,
	pub dir: Option<String>,
}

impl WriterOutput {
	pub fn create(&self, output_dir: &str) -> Result<(), Box<dyn Error>> {
		let crate_dir = Path::new(output_dir);
		fs::create_dir_all(crate_dir)?;

		for OutputFile {
			name,
			content,
			dir,
		} in self.files.iter()
		{
			let dir_path = crate_dir.join(dir.as_ref().unwrap_or(&"".to_string()));
			if !dir_path.exists() {
				fs::create_dir_all(&dir_path)?;
			}
			let file_path = dir_path.join(name);
			println!("Writing {}", file_path.display());
			let mut file = fs::File::create(file_path)?;
			file.write_all(content.as_bytes())?;
		}

		// Format each of the files
		for OutputFile {
			name,
			dir,
			..
		} in self.files.iter()
		{
			let file_path = crate_dir.join(dir.as_ref().unwrap_or(&"".to_string())).join(name);
			if file_path.extension().unwrap() == "toml" || file_path.extension().unwrap() == "proto"
			{
				continue;
			}
			let exit_status = Command::new("rustfmt").arg(&file_path).status()?; // Get the status code
			if !exit_status.success() {
				// Propagate the error if any
				return Err(format!("Fail to format file `{name}`").into());
			}

			println!("Formating {}", file_path.display());
		}

		println!("... Done.");

		Ok(())
	}
}

#[derive(PartialEq, Eq, Debug)]
pub enum WithSerde {
	None,
	Serialize,
	Deserialize,
	Both,
}

impl WithSerde {
	pub fn extra_derive(&self) -> TokenStream {
		let mut extra_derive = match self {
			Self::None => {
				quote! {}
			}
			Self::Serialize => {
				quote! {
					Serialize
				}
			}
			Self::Deserialize => {
				quote! {
					Deserialize
				}
			}
			Self::Both => {
				quote! {
					Serialize, Deserialize
				}
			}
		};
		if !extra_derive.is_empty() {
			extra_derive = quote! { , #extra_derive }
		}
		extra_derive
	}

	pub fn gen_import(&self, db_type: GeneratorType) -> TokenStream {
		let prelude_import = match db_type {
			GeneratorType::Database => {
				quote!(
					use sea_orm::entity::prelude::*;
				)
			}

			GeneratorType::Proto => {
				quote! {
					syntax = "proto3";
				}
			}
			GeneratorType::Api => TokenStream::new(),
		};

		match self {
			WithSerde::None => prelude_import,
			WithSerde::Serialize => {
				quote! {
					#prelude_import
					use serde::Serialize;
				}
			}
			WithSerde::Deserialize => {
				quote! {
					#prelude_import
					use serde::Deserialize;
				}
			}
			WithSerde::Both => {
				quote! {
					#prelude_import
					use serde::{Deserialize,Serialize};
				}
			}
		}
	}
}

impl FromStr for WithSerde {
	type Err = crate::error::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"none" => Self::None,
			"serialize" => Self::Serialize,
			"deserialize" => Self::Deserialize,
			"both" => Self::Both,
			v => {
				return Err(crate::error::Error::TransformError(format!(
					"Unsupported enum variant '{v}'"
				)));
			}
		})
	}
}

/// Converts *_extra_derives argument to token stream
pub(crate) fn bonus_derive<T, I>(extra_derives: I) -> TokenStream
where
	T: Into<String>,
	I: IntoIterator<Item = T>,
{
	extra_derives.into_iter().map(Into::<String>::into).fold(
		TokenStream::default(),
		|acc, derive| {
			let tokens: TokenStream = derive.parse().unwrap();
			quote! { #acc, #tokens }
		},
	)
}

/// convert *_extra_attributes argument to token stream
pub(crate) fn bonus_attributes<T, I>(attributes: I) -> TokenStream
where
	T: Into<String>,
	I: IntoIterator<Item = T>,
{
	attributes.into_iter().map(Into::<String>::into).fold(
		TokenStream::default(),
		|acc, attribute| {
			let tokens: TokenStream = attribute.parse().unwrap();
			quote! {
				#acc
				#[#tokens]
			}
		},
	)
}

#[derive(Debug)]
pub struct WriterContext {
	pub expanded_format: bool,
	pub with_serde: WithSerde,
	pub with_copy_enums: bool,
	pub date_time_crate: DateTimeCrate,
	pub schema_name: Option<String>,
	pub serde_skip_hidden_column: bool,
	pub serde_skip_deserializing_primary_key: bool,
	pub model_extra_derives: TokenStream,
	pub model_extra_attributes: TokenStream,
	pub enum_extra_derives: TokenStream,
	pub enum_extra_attributes: TokenStream,
	pub seaography: bool,
	pub crate_name: Option<String>,
	pub module_name: Option<String>,
	pub framework: WebFrameworkEnum,
	pub grpc: bool,
	pub rest: bool,
	pub proto: bool,
	pub exclude_tables: Option<Vec<String>>,
	pub exclude_enums: Option<Vec<String>>,
}

impl WriterContext {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		expanded_format: bool,
		with_serde: WithSerde,
		with_copy_enums: bool,
		date_time_crate: DateTimeCrate,
		schema_name: Option<String>,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: Vec<String>,
		model_extra_attributes: Vec<String>,
		enum_extra_derives: Vec<String>,
		enum_extra_attributes: Vec<String>,
		seaography: bool,
		crate_name: Option<String>,
		module_name: Option<String>,
		framework: WebFrameworkEnum,
		grpc: bool,
		rest: bool,
		proto: bool,
		exclude_tables: Option<Vec<String>>,
		exclude_enums: Option<Vec<String>>,
	) -> Self {
		Self {
			expanded_format,
			with_serde,
			with_copy_enums,
			date_time_crate,
			schema_name,
			serde_skip_deserializing_primary_key,
			serde_skip_hidden_column,
			model_extra_derives: bonus_derive(model_extra_derives),
			model_extra_attributes: bonus_attributes(model_extra_attributes),
			enum_extra_derives: bonus_derive(enum_extra_derives),
			enum_extra_attributes: bonus_attributes(enum_extra_attributes),
			seaography,
			crate_name,
			module_name,
			framework,
			grpc,
			rest,
			proto,
			exclude_tables,
			exclude_enums,
		}
	}
}
