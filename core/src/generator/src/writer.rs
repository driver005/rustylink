use crate::{
	api::{Graphql, Proto},
	entity::{ActiveEnum, Entity, Generator, PrimaryKey},
	types::{
		GeneratorType, Operation, OutputFile, WebFrameworkEnum, WithSerde, WriterContext,
		WriterOutput,
	},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Writer {
	pub(crate) entities: Vec<Entity>,
	pub(crate) enums: BTreeMap<String, ActiveEnum>,
}

impl Writer {
	pub fn generate(&mut self, context: &WriterContext) -> WriterOutput {
		let mut files = Vec::new();

		self.entities.iter_mut().for_each(|entity| {
			if entity.primary_keys.len() == 0 {
				entity.primary_keys = entity
					.columns
					.iter()
					.map(|column| {
						let name = column.name.clone();

						PrimaryKey {
							name,
						}
					})
					.collect();
			}
		});

		if let Some(exclude_table) = &context.exclude_tables {
			self.entities
				.retain(|entity| !exclude_table.contains(&entity.get_table_name_snake_case()));
		}

		if let Some(name) = &context.crate_name {
			files.push(Self::write_cargo_toml(name, context.framework));
		}

		files.extend(self.write_entities(context));
		files.push(self.write_index_file(&GeneratorType::Database, context, true));
		files.push(self.write_prelude(context));
		if !self.enums.is_empty() {
			files.push(self.write_sea_orm_active_enums(
				&context.with_serde,
				context.with_copy_enums,
				&context.enum_extra_derives,
				&context.enum_extra_attributes,
				context,
			));
		}

		if context.seaography {
			files.push(self.write_index_file(&GeneratorType::Api, context, false));
			// files.push(self.write_graphql_main(context));
			files.push(self.write_graphql_query_root(context));
		}

		if context.proto {
			files.extend(self.write_proto(context));
		}

		WriterOutput {
			files,
		}
	}

	pub fn write_entities(&self, context: &WriterContext) -> Vec<OutputFile> {
		self.entities
			.iter()
			.map(|entity| {
				let entity_file = format!("{}.rs", entity.get_table_name_snake_case());
				let column_info = entity
					.columns
					.iter()
					.map(|column| column.get_info(&context.date_time_crate))
					.collect::<Vec<String>>();
				// Serde must be enabled to use this
				let serde_skip_deserializing_primary_key = context
					.serde_skip_deserializing_primary_key
					&& matches!(context.with_serde, WithSerde::Both | WithSerde::Deserialize);
				let serde_skip_hidden_column = context.serde_skip_hidden_column
					&& matches!(
						context.with_serde,
						WithSerde::Both | WithSerde::Serialize | WithSerde::Deserialize
					);

				info!("Generating Entity {}", entity_file);
				for info in column_info.iter() {
					info!("    > {}", info);
				}

				let mut lines = Vec::new();
				Self::write_doc_comment(&mut lines);
				let code_blocks = if context.expanded_format {
					Generator::gen_expanded_code_blocks(
						entity,
						&context.with_serde,
						&context.date_time_crate,
						&context.schema_name,
						serde_skip_deserializing_primary_key,
						serde_skip_hidden_column,
						&context.model_extra_derives,
						&context.model_extra_attributes,
						context.seaography,
					)
				} else {
					Generator::gen_compact_code_blocks(
						entity,
						&context.with_serde,
						&context.date_time_crate,
						&context.schema_name,
						serde_skip_deserializing_primary_key,
						serde_skip_hidden_column,
						&context.model_extra_derives,
						&context.model_extra_attributes,
						context.seaography,
					)
				};
				Self::write(&mut lines, code_blocks);
				OutputFile {
					name: entity_file,
					content: lines.join("\n\n"),
					dir: Self::generate_dir_name(context, Some("entities".to_string())),
				}
			})
			.collect()
	}

	pub fn write_index_file(
		&self,
		generator_type: &GeneratorType,
		context: &WriterContext,
		module: bool,
	) -> OutputFile {
		let mut lines = Vec::new();
		Self::write_doc_comment(&mut lines);
		let code_blocks: Vec<TokenStream> = self.entities.iter().map(Generator::gen_mod).collect();
		match generator_type {
			GeneratorType::Database => {
				Self::write(
					&mut lines,
					vec![quote! {
						pub mod prelude;
					}],
				);
				lines.push("".to_owned());
				Self::write(&mut lines, code_blocks);
				if !self.enums.is_empty() {
					Self::write(
						&mut lines,
						vec![quote! {
							pub mod sea_orm_active_enums;
						}],
					);
				}
			}
			GeneratorType::Api => {
				let mut modules = Vec::new();

				modules.push(quote! {
					#![allow(unused)]

					pub mod entities;
					pub mod query_root;
				});

				Self::write(&mut lines, modules);
			}
			GeneratorType::Proto => {}
		}

		let file_name = match context.crate_name.is_some() && !module {
			true => "lib.rs".to_owned(),
			false => "mod.rs".to_owned(),
		};

		let dir = match generator_type {
			GeneratorType::Database => {
				Self::generate_dir_name(context, Some("entities".to_string()))
			}
			GeneratorType::Proto => Self::generate_dir_name(context, Some("proto".to_string())),
			GeneratorType::Api => Self::generate_dir_name(context, None),
		};

		OutputFile {
			name: file_name,
			content: lines.join("\n"),
			dir,
		}
	}

	pub fn write_prelude(&self, context: &WriterContext) -> OutputFile {
		let mut lines = Vec::new();
		Self::write_doc_comment(&mut lines);
		let code_blocks = self.entities.iter().map(Generator::gen_prelude_use).collect();
		Self::write(&mut lines, code_blocks);
		OutputFile {
			name: "prelude.rs".to_owned(),
			content: lines.join("\n"),
			dir: Self::generate_dir_name(context, Some("entities".to_string())),
		}
	}

	pub fn write_sea_orm_active_enums(
		&self,
		with_serde: &WithSerde,
		with_copy_enums: bool,
		extra_derives: &TokenStream,
		extra_attributes: &TokenStream,
		context: &WriterContext,
	) -> OutputFile {
		let mut lines = Vec::new();
		Self::write_doc_comment(&mut lines);
		Self::write(&mut lines, vec![with_serde.gen_import(GeneratorType::Database)]);
		lines.push("".to_owned());

		self.enums.values().for_each(|active_enum| {
			Self::write(
				&mut lines,
				vec![
					active_enum.gen_active_enum(
						with_serde,
						with_copy_enums,
						extra_derives,
						extra_attributes,
					),
					active_enum.gen_enum_into(),
				],
			);
		});

		OutputFile {
			name: "sea_orm_active_enums.rs".to_owned(),
			content: lines.join("\n"),
			dir: Self::generate_dir_name(context, Some("entities".to_string())),
		}
	}

	pub fn write(lines: &mut Vec<String>, code_blocks: Vec<TokenStream>) {
		lines.extend(
			code_blocks.into_iter().map(|code_block| code_block.to_string()).collect::<Vec<_>>(),
		);
	}

	pub fn write_doc_comment(lines: &mut Vec<String>) {
		let ver = env!("CARGO_PKG_VERSION");
		let comments = vec![format!("//! `SeaORM` Entity, @generated by sea-orm-codegen {ver}")];
		lines.extend(comments);
		lines.push("".to_owned());
	}

	pub fn write_proto(&self, context: &WriterContext) -> Vec<OutputFile> {
		self.entities
			.iter()
			.map(|entity| {
				let entity_file = format!("{}.proto", entity.get_table_name_snake_case());
				let column_info = entity
					.columns
					.iter()
					.map(|column| column.get_info(&context.date_time_crate))
					.collect::<Vec<String>>();

				let primary_keys_name =
					entity.primary_keys.iter().map(|pk| pk.name.clone()).collect::<Vec<String>>();

				info!("Generating Proto {}", entity_file);
				for info in column_info.iter() {
					info!("    > {}", info);
				}

				let mut lines = Vec::new();
				Self::write_doc_comment(&mut lines);

				Self::write(&mut lines, vec![WithSerde::None.gen_import(GeneratorType::Proto)]);

				Self::write(&mut lines, vec![Proto::gen_service(entity, Operation::create())]);

				Operation::create().iter().for_each(|op| {
					Self::write(
						&mut lines,
						Proto::gen_code_blocks(entity, op, primary_keys_name.clone()),
					);
				});

				self.enums.values().for_each(|active_enum| {
					Self::write(&mut lines, vec![Proto::gen_active_enum(active_enum)]);
				});

				OutputFile {
					name: entity_file,
					content: lines.join("\n\n"),
					dir: Self::generate_dir_name(context, Some("proto".to_string())),
				}
			})
			.collect()
	}

	pub fn write_graphql_main(&self, context: &WriterContext) -> OutputFile {
		let mut lines = Vec::new();
		Self::write_doc_comment(&mut lines);

		Self::write(&mut lines, vec![Graphql::main(&context.crate_name, context.framework)]);

		OutputFile {
			name: "main.rs".to_owned(),
			content: lines.join("\n"),
			dir: Self::generate_dir_name(context, None),
		}
	}

	pub fn write_graphql_query_root(&self, context: &WriterContext) -> OutputFile {
		let mut lines = Vec::new();
		Self::write_doc_comment(&mut lines);

		Self::write(
			&mut lines,
			vec![Graphql::query_root(&self.entities, &self.enums, Self::gen_name(context))],
		);

		OutputFile {
			name: "query_root.rs".to_owned(),
			content: lines.join("\n"),
			dir: Self::generate_dir_name(context, None),
		}
	}

	pub fn write_cargo_toml(crate_name: &str, framework: WebFrameworkEnum) -> OutputFile {
		OutputFile {
			name: "Cargo.toml".to_owned(),
			content: Graphql::cargo_toml(crate_name, "sqlx-all", "1.0.0", framework),
			dir: None,
		}
	}

	fn generate_dir_name(context: &WriterContext, dir: Option<String>) -> Option<String> {
		match &context.crate_name {
			Some(_) => match dir {
				Some(val) => Some(format!("src/{}", val)),
				None => Some("src".to_string()),
			},
			None => match dir {
				Some(val) => Some(val),
				None => None,
			},
		}
	}

	fn gen_name(context: &WriterContext) -> &Option<String> {
		if context.crate_name.is_some() {
			&context.crate_name
		} else if context.module_name.is_some() {
			&context.module_name
		} else {
			&None
		}
	}
}
