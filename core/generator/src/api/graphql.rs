use crate::{entity, types::WebFrameworkEnum, util::escape_rust_keyword};
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

pub struct Graphql {}

impl Graphql {
	pub fn query_root(
		entities: &Vec<entity::Entity>,
		enumerations: &BTreeMap<String, entity::ActiveEnum>,
		crate_name: &Option<String>,
	) -> TokenStream {
		let name = match crate_name {
			Some(val) => {
				let name = format_ident!("{}", val);
				quote! {
					crate::#name
				}
			}
			None => quote! {
				crate
			},
		};
		let entities: Vec<TokenStream> = entities
			.iter()
			.map(|entity| {
				let entity_name = &entity.get_table_name_snake_case();

				let entity_path = format_ident!("{}", escape_rust_keyword(entity_name));

				quote! {
					#entity_path
				}
			})
			.collect();

		let enumerations = enumerations.iter().map(|definition| {
			let enum_name = &definition.0; //.get_active_enum_name_snake_case();

			let enum_path =
				format_ident!("{}", escape_rust_keyword(enum_name.to_upper_camel_case()));

			quote! {
				builder.register_enumeration::<#name::entities::sea_orm_active_enums::#enum_path>();
			}
		});

		quote! {
			use #name::entities::*;
			use async_graphql::dynamic::*;
			use sea_orm::DatabaseConnection;
			use seaography::{Builder, BuilderContext};

			lazy_static::lazy_static! {
				static ref CONTEXT: BuilderContext = BuilderContext::default();
			}

			pub fn schema(
				database: DatabaseConnection,
				depth: Option<usize>,
				complexity: Option<usize>,
			) -> Result<Schema, SchemaError> {
				let mut builder = Builder::new(&CONTEXT, database.clone());

				seaography::register_entities!(
					builder,
					[
						#(#entities,)*
					]
				);

				#(#enumerations)*

				let schema = builder.schema_builder();

				let schema = if let Some(depth) = depth {
					schema.limit_depth(depth)
				} else {
					schema
				};

				let schema = if let Some(complexity) = complexity {
					schema.limit_complexity(complexity)
				} else {
					schema
				};

				schema.data(database).finish()
			}
		}
	}

	pub fn cargo_toml(
		crate_name: &str,
		sql_library: &str,
		version: &str,
		framework: WebFrameworkEnum,
	) -> String {
		let content = match framework {
			WebFrameworkEnum::Actix => include_str!("../templates/graphql/actix_cargo.toml"),
			WebFrameworkEnum::Poem => include_str!("../templates/graphql/poem_cargo.toml"),
			WebFrameworkEnum::Axum => include_str!("../templates/graphql/axum_cargo.toml"),
		};

		content
			.replace("<seaography-package-name>", crate_name)
			.replace("<seaography-sql-library>", sql_library)
			.replace("<seaography-version>", version)
	}

	pub fn main(crate_name: &Option<String>, framework: WebFrameworkEnum) -> TokenStream {
		let content = match framework {
			WebFrameworkEnum::Actix => crate::templates::actix::generate_main(crate_name),
			WebFrameworkEnum::Poem => crate::templates::poem::generate_main(crate_name),
			WebFrameworkEnum::Axum => crate::templates::axum::generate_main(crate_name),
		};

		quote! {
			#content
		}
	}
}
