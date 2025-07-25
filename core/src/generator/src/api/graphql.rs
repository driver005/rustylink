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
			use ::apy::{Builder, BuilderContext, QueryRoot, FilterTypeTrait, GraphQlFilterType, ProtoFilterType};
			use ::dynamic::{
				prelude::{Proto as DynamicProto, Schema as DynamicSchema, SchemaError, DynamicBuilder, GraphQLTypeRef, ProtoTypeRef, GraphQLEnum, ProtoEnum, TypeRefTrait, EnumTrait},
			};
			use sea_orm::DatabaseConnection;

			lazy_static::lazy_static! {
				static ref CONTEXT: BuilderContext = BuilderContext::default();
			}

			fn builder<T, E, F>(database: &DatabaseConnection) -> DynamicBuilder<T, E>
			where
				T: TypeRefTrait,
				E: EnumTrait,
				F: FilterTypeTrait,
			{
				let mut builder = Builder::<T, E, F>::new(&CONTEXT, database.clone());

				apy::register_entities!(
					builder,
					[
						#(#entities,)*
					],
					T,
					F
				);

				#(#enumerations)*

				builder.builder()
			}

			pub struct Schema {
				depth: Option<u16>,
				complexity: Option<u16>,
			}

			impl Schema {
				pub fn new() -> Self {
					Self {
						depth: None,
						complexity: None,
					}
				}
			}

			impl QueryRoot<DynamicSchema> for Schema {
				fn config_schema(&mut self, depth: u16, complexity: u16) {
					self.depth = Some(depth);
					self.complexity = Some(complexity);
				}

				fn root(&self, database: &DatabaseConnection)-> Result<DynamicSchema, SchemaError> {
					let builder = builder::<GraphQLTypeRef, GraphQLEnum, GraphQlFilterType>(database);

					let schema = builder.builder();

					let schema = if let Some(depth) = self.depth {
						schema.limit_depth(depth)
					} else {
						schema
					};

					let schema = if let Some(complexity) = self.complexity {
						schema.limit_complexity(complexity)
					} else {
						schema
					};

					schema.data(database.clone()).finish()
				}
			}

			pub struct Proto {}

			impl Proto {
				pub fn new() -> Self {
					Self {}
				}
			}

			impl QueryRoot<DynamicProto> for Proto {
				fn root(&self, database: &DatabaseConnection)-> Result<DynamicProto, SchemaError> {
					let builder = builder::<ProtoTypeRef, ProtoEnum, ProtoFilterType>(database);

					let proto = builder.builder();

					proto.data(database.clone()).finish()
				}
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
