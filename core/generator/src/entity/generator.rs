use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma};

use crate::{
	types::{DateTimeCrate, GeneratorType, WithSerde},
	util::escape_rust_keyword,
};

use super::Entity;

pub struct Generator {}

impl Generator {
	#[allow(clippy::too_many_arguments)]
	pub fn gen_expanded_code_blocks(
		entity: &Entity,
		with_serde: &WithSerde,
		date_time_crate: &DateTimeCrate,
		schema_name: &Option<String>,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: &TokenStream,
		model_extra_attributes: &TokenStream,
		seaography: bool,
	) -> Vec<TokenStream> {
		let mut imports = with_serde.gen_import(GeneratorType::Database);
		imports.extend(Self::gen_import_active_enum(entity));
		let mut code_blocks = vec![
			imports,
			Self::gen_entity_struct(),
			Self::gen_impl_entity_name(entity, schema_name),
			Self::gen_model_struct(
				entity,
				with_serde,
				date_time_crate,
				serde_skip_deserializing_primary_key,
				serde_skip_hidden_column,
				model_extra_derives,
				model_extra_attributes,
			),
			Self::gen_column_enum(entity),
			Self::gen_primary_key_enum(entity),
			Self::gen_impl_primary_key(entity, date_time_crate),
			Self::gen_relation_enum(entity),
			Self::gen_impl_column_trait(entity),
			Self::gen_impl_relation_trait(entity),
		];
		code_blocks.extend(Self::gen_impl_related(entity));
		code_blocks.extend(Self::gen_impl_conjunct_related(entity));
		code_blocks.extend([Self::gen_impl_active_model_behavior()]);
		if seaography {
			code_blocks.extend([Self::gen_related_entity(entity)]);
		}
		code_blocks
	}

	#[allow(clippy::too_many_arguments)]
	pub fn gen_compact_code_blocks(
		entity: &Entity,
		with_serde: &WithSerde,
		date_time_crate: &DateTimeCrate,
		schema_name: &Option<String>,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: &TokenStream,
		model_extra_attributes: &TokenStream,
		seaography: bool,
	) -> Vec<TokenStream> {
		let mut imports = with_serde.gen_import(GeneratorType::Database);
		imports.extend(Self::gen_import_active_enum(entity));
		let mut code_blocks = vec![
			imports,
			Self::gen_compact_model_struct(
				entity,
				with_serde,
				date_time_crate,
				schema_name,
				serde_skip_deserializing_primary_key,
				serde_skip_hidden_column,
				model_extra_derives,
				model_extra_attributes,
			),
			Self::gen_compact_relation_enum(entity),
		];
		code_blocks.extend(Self::gen_impl_related(entity));
		code_blocks.extend(Self::gen_impl_conjunct_related(entity));
		code_blocks.extend([Self::gen_impl_active_model_behavior()]);
		if seaography {
			code_blocks.extend([Self::gen_related_entity(entity)]);
		}
		code_blocks
	}

	pub fn gen_entity_struct() -> TokenStream {
		quote! {
			#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
			pub struct Entity;
		}
	}

	pub fn gen_impl_entity_name(entity: &Entity, schema_name: &Option<String>) -> TokenStream {
		let schema_name = match Self::gen_schema_name(schema_name) {
			Some(schema_name) => quote! {
				fn schema_name(&self) -> Option<&str> {
					Some(#schema_name)
				}
			},
			None => quote! {},
		};
		let table_name = entity.table_name.as_str();
		let table_name = quote! {
			fn table_name(&self) -> &str {
				#table_name
			}
		};
		quote! {
			impl EntityName for Entity {
				#schema_name
				#table_name
			}
		}
	}

	pub fn gen_import_active_enum(entity: &Entity) -> TokenStream {
		entity
			.columns
			.iter()
			.fold((TokenStream::new(), Vec::new()), |(mut ts, mut enums), col| {
				if let sea_query::ColumnType::Enum {
					name,
					..
				} = col.get_inner_col_type()
				{
					if !enums.contains(&name) {
						enums.push(name);
						let enum_name = format_ident!("{}", name.to_string().to_upper_camel_case());
						ts.extend([quote! {
							use super::sea_orm_active_enums::#enum_name;
						}]);
					}
				}
				(ts, enums)
			})
			.0
	}

	pub fn gen_model_struct(
		entity: &Entity,
		with_serde: &WithSerde,
		date_time_crate: &DateTimeCrate,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: &TokenStream,
		model_extra_attributes: &TokenStream,
	) -> TokenStream {
		let column_names_snake_case = entity.get_column_names_snake_case();
		let column_rs_types = entity.get_column_rs_types(date_time_crate, None);
		let if_eq_needed = entity.get_eq_needed();
		let serde_attributes = entity.get_column_serde_attributes(
			serde_skip_deserializing_primary_key,
			serde_skip_hidden_column,
		);
		let extra_derive = with_serde.extra_derive();

		quote! {
			#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel #if_eq_needed #extra_derive #model_extra_derives)]
			#model_extra_attributes
			pub struct Model {
				#(
					#serde_attributes
					pub #column_names_snake_case: #column_rs_types,
				)*
			}
		}
	}

	pub fn gen_column_enum(entity: &Entity) -> TokenStream {
		let column_variants = entity.columns.iter().map(|col| {
			let variant = col.get_name_camel_case();
			let mut variant = quote! { #variant };
			if !col.is_snake_case_name() {
				let column_name = &col.name;
				variant = quote! {
					#[sea_orm(column_name = #column_name)]
					#variant
				};
			}
			variant
		});
		quote! {
			#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
			pub enum Column {
				#(#column_variants,)*
			}
		}
	}

	pub fn gen_primary_key_enum(entity: &Entity) -> TokenStream {
		let primary_key_names_camel_case = entity.get_primary_key_names_camel_case();
		quote! {
			#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
			pub enum PrimaryKey {
				#(#primary_key_names_camel_case,)*
			}
		}
	}

	pub fn gen_impl_primary_key(entity: &Entity, date_time_crate: &DateTimeCrate) -> TokenStream {
		let primary_key_auto_increment = entity.get_primary_key_auto_increment();
		let value_type = entity.get_primary_key_rs_type(date_time_crate, None);
		quote! {
			impl PrimaryKeyTrait for PrimaryKey {
				type ValueType = #value_type;

				fn auto_increment() -> bool {
					#primary_key_auto_increment
				}
			}
		}
	}

	pub fn gen_relation_enum(entity: &Entity) -> TokenStream {
		let relation_enum_name = entity.get_relation_enum_name();
		quote! {
			#[derive(Copy, Clone, Debug, EnumIter)]
			pub enum Relation {
				#(#relation_enum_name,)*
			}
		}
	}

	pub fn gen_impl_column_trait(entity: &Entity) -> TokenStream {
		let column_names_camel_case = entity.get_column_names_camel_case();
		let column_defs = entity.get_column_defs();
		quote! {
			impl ColumnTrait for Column {
				type EntityName = Entity;

				fn def(&self) -> ColumnDef {
					match self {
						#(Self::#column_names_camel_case => #column_defs,)*
					}
				}
			}
		}
	}

	pub fn gen_impl_relation_trait(entity: &Entity) -> TokenStream {
		let relation_enum_name = entity.get_relation_enum_name();
		let relation_defs = entity.get_relation_defs();
		let quoted = if relation_enum_name.is_empty() {
			quote! {
				panic!("No RelationDef")
			}
		} else {
			quote! {
				match self {
					#(Self::#relation_enum_name => #relation_defs,)*
				}
			}
		};
		quote! {
			impl RelationTrait for Relation {
				fn def(&self) -> RelationDef {
					#quoted
				}
			}
		}
	}

	pub fn gen_impl_related(entity: &Entity) -> Vec<TokenStream> {
		entity
			.relations
			.iter()
			.filter(|rel| !rel.self_referencing && rel.num_suffix == 0 && rel.impl_related)
			.map(|rel| {
				let enum_name = rel.get_enum_name();
				let module_name = rel.get_module_name();
				let inner = quote! {
					fn to() -> RelationDef {
						Relation::#enum_name.def()
					}
				};
				if module_name.is_some() {
					quote! {
						impl Related<super::#module_name::Entity> for Entity { #inner }
					}
				} else {
					quote! {
						impl Related<Entity> for Entity { #inner }
					}
				}
			})
			.collect()
	}

	/// Used to generate `enum RelatedEntity` that is useful to the Seaography project
	pub fn gen_related_entity(entity: &Entity) -> TokenStream {
		let related_enum_name = entity.get_related_entity_enum_name();
		let related_attrs = entity.get_related_entity_attrs();

		quote! {
			#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
			pub enum RelatedEntity {
				#(
					#related_attrs
					#related_enum_name
				),*
			}
		}
	}

	pub fn gen_impl_conjunct_related(entity: &Entity) -> Vec<TokenStream> {
		let table_name_camel_case = entity.get_table_name_camel_case_ident();
		let via_snake_case = entity.get_conjunct_relations_via_snake_case();
		let to_snake_case = entity.get_conjunct_relations_to_snake_case();
		let to_upper_camel_case = entity.get_conjunct_relations_to_upper_camel_case();
		via_snake_case
			.into_iter()
			.zip(to_snake_case)
			.zip(to_upper_camel_case)
			.map(|((via_snake_case, to_snake_case), to_upper_camel_case)| {
				quote! {
					impl Related<super::#to_snake_case::Entity> for Entity {
						fn to() -> RelationDef {
							super::#via_snake_case::Relation::#to_upper_camel_case.def()
						}

						fn via() -> Option<RelationDef> {
							Some(super::#via_snake_case::Relation::#table_name_camel_case.def().rev())
						}
					}
				}
			})
			.collect()
	}

	pub fn gen_impl_active_model_behavior() -> TokenStream {
		quote! {
			impl ActiveModelBehavior for ActiveModel {}
		}
	}

	#[allow(clippy::too_many_arguments)]
	pub fn gen_compact_model_struct(
		entity: &Entity,
		with_serde: &WithSerde,
		date_time_crate: &DateTimeCrate,
		schema_name: &Option<String>,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: &TokenStream,
		model_extra_attributes: &TokenStream,
	) -> TokenStream {
		let table_name = entity.table_name.as_str();
		let column_names_snake_case = entity.get_column_names_snake_case();
		let column_rs_types = entity.get_column_rs_types(date_time_crate, None);
		let if_eq_needed = entity.get_eq_needed();
		let primary_keys: Vec<String> =
			entity.primary_keys.iter().map(|pk| pk.name.clone()).collect();
		let attrs: Vec<TokenStream> = entity
			.columns
			.iter()
			.map(|col| {
				let mut attrs: Punctuated<_, Comma> = Punctuated::new();
				let is_primary_key = primary_keys.contains(&col.name);
				if !col.is_snake_case_name() {
					let column_name = &col.name;
					attrs.push(quote! { column_name = #column_name });
				}
				if is_primary_key {
					attrs.push(quote! { primary_key });
					if !col.auto_increment {
						attrs.push(quote! { auto_increment = false });
					}
				}
				if let Some(ts) = col.get_col_type_entity_attrs() {
					attrs.extend([ts]);
					if !col.not_null {
						attrs.push(quote! { nullable });
					}
				};
				if col.unique {
					attrs.push(quote! { unique });
				}
				let mut ts = quote! {};
				if !attrs.is_empty() {
					for (i, attr) in attrs.into_iter().enumerate() {
						if i > 0 {
							ts = quote! { #ts, };
						}
						ts = quote! { #ts #attr };
					}
					ts = quote! { #[sea_orm(#ts)] };
				}
				let serde_attribute = col.get_serde_attribute(
					is_primary_key,
					serde_skip_deserializing_primary_key,
					serde_skip_hidden_column,
				);
				ts = quote! {
					#ts
					#serde_attribute
				};
				ts
			})
			.collect();
		let schema_name = match Self::gen_schema_name(schema_name) {
			Some(schema_name) => quote! {
				schema_name = #schema_name,
			},
			None => quote! {},
		};
		let extra_derive = with_serde.extra_derive();

		quote! {
			#[derive(Clone, Debug, PartialEq, DeriveEntityModel #if_eq_needed #extra_derive #model_extra_derives)]
			#[sea_orm(
				#schema_name
				table_name = #table_name
			)]
			#model_extra_attributes
			pub struct Model {
				#(
					#attrs
					pub #column_names_snake_case: #column_rs_types,
				)*
			}
		}
	}

	pub fn gen_compact_relation_enum(entity: &Entity) -> TokenStream {
		let relation_enum_name = entity.get_relation_enum_name();
		let attrs = entity.get_relation_attrs();
		quote! {
			#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
			pub enum Relation {
				#(
					#attrs
					#relation_enum_name,
				)*
			}
		}
	}

	pub fn gen_schema_name(schema_name: &Option<String>) -> Option<TokenStream> {
		schema_name.as_ref().map(|schema_name| quote! { #schema_name })
	}

	pub fn gen_mod(entity: &Entity) -> TokenStream {
		let table_name_snake_case_ident =
			format_ident!("{}", escape_rust_keyword(entity.get_table_name_snake_case_ident()));
		quote! {
			pub mod #table_name_snake_case_ident;
		}
	}

	pub fn gen_prelude_use(entity: &Entity) -> TokenStream {
		let table_name_snake_case_ident = entity.get_table_name_snake_case_ident();
		let table_name_camel_case_ident = entity.get_table_name_camel_case_ident();

		let column_name = format_ident!("{}Column", table_name_camel_case_ident);
		let model_name = format_ident!("{}Model", table_name_camel_case_ident);
		let active_name = format_ident!("{}ActiveModel", table_name_camel_case_ident);

		quote! {
			pub use super::#table_name_snake_case_ident::Entity as #table_name_camel_case_ident;
			pub use super::#table_name_snake_case_ident::Model as #model_name;
			pub use super::#table_name_snake_case_ident::Column as #column_name;
			pub use super::#table_name_snake_case_ident::ActiveModel as #active_name;
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		entity::Column,
		entity::ConjunctRelation,
		entity::Entity,
		entity::Generator,
		entity::PrimaryKey,
		entity::Relation,
		entity::RelationType,
		types::WithSerde,
		types::{bonus_attributes, bonus_derive},
		DateTimeCrate,
	};
	use pretty_assertions::assert_eq;
	use proc_macro2::TokenStream;
	use quote::quote;
	use sea_query::{Alias, ColumnType, ForeignKeyAction, RcOrArc, SeaRc, StringLen};
	use std::io::{self, BufRead, BufReader, Read};

	fn setup() -> Vec<Entity> {
		vec![
			Entity {
				table_name: "cake".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "name".to_owned(),
						col_type: ColumnType::Text,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "fruit".to_owned(),
					columns: vec![],
					ref_columns: vec![],
					rel_type: RelationType::HasMany,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![ConjunctRelation {
					via: "cake_filling".to_owned(),
					to: "filling".to_owned(),
				}],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "_cake_filling_".to_owned(),
				columns: vec![
					Column {
						name: "cake_id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "filling_id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![
					Relation {
						ref_table: "cake".to_owned(),
						columns: vec!["cake_id".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: Some(ForeignKeyAction::Cascade),
						on_update: Some(ForeignKeyAction::Cascade),
						self_referencing: false,
						num_suffix: 0,
						impl_related: true,
					},
					Relation {
						ref_table: "filling".to_owned(),
						columns: vec!["filling_id".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: Some(ForeignKeyAction::Cascade),
						on_update: Some(ForeignKeyAction::Cascade),
						self_referencing: false,
						num_suffix: 0,
						impl_related: true,
					},
				],
				conjunct_relations: vec![],
				primary_keys: vec![
					PrimaryKey {
						name: "cake_id".to_owned(),
					},
					PrimaryKey {
						name: "filling_id".to_owned(),
					},
				],
			},
			Entity {
				table_name: "cake_filling_price".to_owned(),
				columns: vec![
					Column {
						name: "cake_id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "filling_id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "price".to_owned(),
						col_type: ColumnType::Decimal(None),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "cake_filling".to_owned(),
					columns: vec!["cake_id".to_owned(), "filling_id".to_owned()],
					ref_columns: vec!["cake_id".to_owned(), "filling_id".to_owned()],
					rel_type: RelationType::BelongsTo,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![],
				primary_keys: vec![
					PrimaryKey {
						name: "cake_id".to_owned(),
					},
					PrimaryKey {
						name: "filling_id".to_owned(),
					},
				],
			},
			Entity {
				table_name: "filling".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "name".to_owned(),
						col_type: ColumnType::String(StringLen::N(255)),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![],
				conjunct_relations: vec![ConjunctRelation {
					via: "cake_filling".to_owned(),
					to: "cake".to_owned(),
				}],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "fruit".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "name".to_owned(),
						col_type: ColumnType::String(StringLen::N(255)),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "cake_id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
				],
				relations: vec![
					Relation {
						ref_table: "cake".to_owned(),
						columns: vec!["cake_id".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: None,
						on_update: None,
						self_referencing: false,
						num_suffix: 0,
						impl_related: true,
					},
					Relation {
						ref_table: "vendor".to_owned(),
						columns: vec![],
						ref_columns: vec![],
						rel_type: RelationType::HasMany,
						on_delete: None,
						on_update: None,
						self_referencing: false,
						num_suffix: 0,
						impl_related: true,
					},
				],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "vendor".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "_name_".to_owned(),
						col_type: ColumnType::String(StringLen::N(255)),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "fruitId".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "fruit".to_owned(),
					columns: vec!["fruitId".to_owned()],
					ref_columns: vec!["id".to_owned()],
					rel_type: RelationType::BelongsTo,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "rust_keyword".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "testing".to_owned(),
						col_type: ColumnType::TinyInteger,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "rust".to_owned(),
						col_type: ColumnType::TinyUnsigned,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "keywords".to_owned(),
						col_type: ColumnType::SmallInteger,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "type".to_owned(),
						col_type: ColumnType::SmallUnsigned,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "typeof".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "crate".to_owned(),
						col_type: ColumnType::Unsigned,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "self".to_owned(),
						col_type: ColumnType::BigInteger,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "self_id1".to_owned(),
						col_type: ColumnType::BigUnsigned,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "self_id2".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "fruit_id1".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "fruit_id2".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "cake_id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![
					Relation {
						ref_table: "rust_keyword".to_owned(),
						columns: vec!["self_id1".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: None,
						on_update: None,
						self_referencing: true,
						num_suffix: 1,
						impl_related: true,
					},
					Relation {
						ref_table: "rust_keyword".to_owned(),
						columns: vec!["self_id2".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: None,
						on_update: None,
						self_referencing: true,
						num_suffix: 2,
						impl_related: true,
					},
					Relation {
						ref_table: "fruit".to_owned(),
						columns: vec!["fruit_id1".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: None,
						on_update: None,
						self_referencing: false,
						num_suffix: 1,
						impl_related: true,
					},
					Relation {
						ref_table: "fruit".to_owned(),
						columns: vec!["fruit_id2".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: None,
						on_update: None,
						self_referencing: false,
						num_suffix: 2,
						impl_related: true,
					},
					Relation {
						ref_table: "cake".to_owned(),
						columns: vec!["cake_id".to_owned()],
						ref_columns: vec!["id".to_owned()],
						rel_type: RelationType::BelongsTo,
						on_delete: None,
						on_update: None,
						self_referencing: false,
						num_suffix: 0,
						impl_related: true,
					},
				],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "cake_with_float".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "name".to_owned(),
						col_type: ColumnType::Text,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
					Column {
						name: "price".to_owned(),
						col_type: ColumnType::Float,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "fruit".to_owned(),
					columns: vec![],
					ref_columns: vec![],
					rel_type: RelationType::HasMany,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![ConjunctRelation {
					via: "cake_filling".to_owned(),
					to: "filling".to_owned(),
				}],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "cake_with_double".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "name".to_owned(),
						col_type: ColumnType::Text,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
					Column {
						name: "price".to_owned(),
						col_type: ColumnType::Double,
						auto_increment: false,
						not_null: false,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "fruit".to_owned(),
					columns: vec![],
					ref_columns: vec![],
					rel_type: RelationType::HasMany,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![ConjunctRelation {
					via: "cake_filling".to_owned(),
					to: "filling".to_owned(),
				}],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "collection".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "integers".to_owned(),
						col_type: ColumnType::Array(RcOrArc::new(ColumnType::Integer)),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "integers_opt".to_owned(),
						col_type: ColumnType::Array(RcOrArc::new(ColumnType::Integer)),
						auto_increment: false,
						not_null: false,
						unique: false,
					},
				],
				relations: vec![],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "collection_float".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "floats".to_owned(),
						col_type: ColumnType::Array(RcOrArc::new(ColumnType::Float)),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "doubles".to_owned(),
						col_type: ColumnType::Array(RcOrArc::new(ColumnType::Double)),
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "parent".to_owned(),
				columns: vec![
					Column {
						name: "id1".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "id2".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "child".to_owned(),
					columns: vec![],
					ref_columns: vec![],
					rel_type: RelationType::HasMany,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![],
				primary_keys: vec![
					PrimaryKey {
						name: "id1".to_owned(),
					},
					PrimaryKey {
						name: "id2".to_owned(),
					},
				],
			},
			Entity {
				table_name: "child".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "parent_id1".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "parent_id2".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![Relation {
					ref_table: "parent".to_owned(),
					columns: vec!["parent_id1".to_owned(), "parent_id2".to_owned()],
					ref_columns: vec!["id1".to_owned(), "id2".to_owned()],
					rel_type: RelationType::BelongsTo,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				}],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
		]
	}

	fn parse_from_file<R>(inner: R) -> io::Result<TokenStream>
	where
		R: Read,
	{
		let mut reader = BufReader::new(inner);
		let mut lines: Vec<String> = Vec::new();

		reader.read_until(b';', &mut Vec::new())?;

		let mut line = String::new();
		while reader.read_line(&mut line)? > 0 {
			lines.push(line.to_owned());
			line.clear();
		}
		let content = lines.join("");
		Ok(content.parse().unwrap())
	}

	#[test]
	fn test_gen_expanded_code_blocks() -> io::Result<()> {
		let entities = setup();
		const ENTITY_FILES: [&str; 13] = [
			include_str!("../../tests/expanded/cake.rs"),
			include_str!("../../tests/expanded/cake_filling.rs"),
			include_str!("../../tests/expanded/cake_filling_price.rs"),
			include_str!("../../tests/expanded/filling.rs"),
			include_str!("../../tests/expanded/fruit.rs"),
			include_str!("../../tests/expanded/vendor.rs"),
			include_str!("../../tests/expanded/rust_keyword.rs"),
			include_str!("../../tests/expanded/cake_with_float.rs"),
			include_str!("../../tests/expanded/cake_with_double.rs"),
			include_str!("../../tests/expanded/collection.rs"),
			include_str!("../../tests/expanded/collection_float.rs"),
			include_str!("../../tests/expanded/parent.rs"),
			include_str!("../../tests/expanded/child.rs"),
		];
		const ENTITY_FILES_WITH_SCHEMA_NAME: [&str; 13] = [
			include_str!("../../tests/expanded_with_schema_name/cake.rs"),
			include_str!("../../tests/expanded_with_schema_name/cake_filling.rs"),
			include_str!("../../tests/expanded_with_schema_name/cake_filling_price.rs"),
			include_str!("../../tests/expanded_with_schema_name/filling.rs"),
			include_str!("../../tests/expanded_with_schema_name/fruit.rs"),
			include_str!("../../tests/expanded_with_schema_name/vendor.rs"),
			include_str!("../../tests/expanded_with_schema_name/rust_keyword.rs"),
			include_str!("../../tests/expanded_with_schema_name/cake_with_float.rs"),
			include_str!("../../tests/expanded_with_schema_name/cake_with_double.rs"),
			include_str!("../../tests/expanded_with_schema_name/collection.rs"),
			include_str!("../../tests/expanded_with_schema_name/collection_float.rs"),
			include_str!("../../tests/expanded_with_schema_name/parent.rs"),
			include_str!("../../tests/expanded_with_schema_name/child.rs"),
		];

		assert_eq!(entities.len(), ENTITY_FILES.len());

		for (i, entity) in entities.iter().enumerate() {
			assert_eq!(
				parse_from_file(ENTITY_FILES[i].as_bytes())?.to_string(),
				Generator::gen_expanded_code_blocks(
					entity,
					&crate::types::WithSerde::None,
					&crate::DateTimeCrate::Chrono,
					&None,
					false,
					false,
					&TokenStream::new(),
					&TokenStream::new(),
					false
				)
				.into_iter()
				.skip(1)
				.fold(TokenStream::new(), |mut acc, tok| {
					acc.extend(tok);
					acc
				})
				.to_string()
			);
			assert_eq!(
				parse_from_file(ENTITY_FILES_WITH_SCHEMA_NAME[i].as_bytes())?.to_string(),
				Generator::gen_expanded_code_blocks(
					entity,
					&crate::types::WithSerde::None,
					&crate::DateTimeCrate::Chrono,
					&Some("schema_name".to_owned()),
					false,
					false,
					&TokenStream::new(),
					&TokenStream::new(),
					false,
				)
				.into_iter()
				.skip(1)
				.fold(TokenStream::new(), |mut acc, tok| {
					acc.extend(tok);
					acc
				})
				.to_string()
			);
		}

		Ok(())
	}

	#[test]
	fn test_gen_compact_code_blocks() -> io::Result<()> {
		let entities = setup();
		const ENTITY_FILES: [&str; 13] = [
			include_str!("../../tests/compact/cake.rs"),
			include_str!("../../tests/compact/cake_filling.rs"),
			include_str!("../../tests/compact/cake_filling_price.rs"),
			include_str!("../../tests/compact/filling.rs"),
			include_str!("../../tests/compact/fruit.rs"),
			include_str!("../../tests/compact/vendor.rs"),
			include_str!("../../tests/compact/rust_keyword.rs"),
			include_str!("../../tests/compact/cake_with_float.rs"),
			include_str!("../../tests/compact/cake_with_double.rs"),
			include_str!("../../tests/compact/collection.rs"),
			include_str!("../../tests/compact/collection_float.rs"),
			include_str!("../../tests/compact/parent.rs"),
			include_str!("../../tests/compact/child.rs"),
		];
		const ENTITY_FILES_WITH_SCHEMA_NAME: [&str; 13] = [
			include_str!("../../tests/compact_with_schema_name/cake.rs"),
			include_str!("../../tests/compact_with_schema_name/cake_filling.rs"),
			include_str!("../../tests/compact_with_schema_name/cake_filling_price.rs"),
			include_str!("../../tests/compact_with_schema_name/filling.rs"),
			include_str!("../../tests/compact_with_schema_name/fruit.rs"),
			include_str!("../../tests/compact_with_schema_name/vendor.rs"),
			include_str!("../../tests/compact_with_schema_name/rust_keyword.rs"),
			include_str!("../../tests/compact_with_schema_name/cake_with_float.rs"),
			include_str!("../../tests/compact_with_schema_name/cake_with_double.rs"),
			include_str!("../../tests/compact_with_schema_name/collection.rs"),
			include_str!("../../tests/compact_with_schema_name/collection_float.rs"),
			include_str!("../../tests/compact_with_schema_name/parent.rs"),
			include_str!("../../tests/compact_with_schema_name/child.rs"),
		];

		assert_eq!(entities.len(), ENTITY_FILES.len());

		for (i, entity) in entities.iter().enumerate() {
			assert_eq!(
				parse_from_file(ENTITY_FILES[i].as_bytes())?.to_string(),
				Generator::gen_compact_code_blocks(
					entity,
					&crate::types::WithSerde::None,
					&crate::DateTimeCrate::Chrono,
					&None,
					false,
					false,
					&TokenStream::new(),
					&TokenStream::new(),
					false,
				)
				.into_iter()
				.skip(1)
				.fold(TokenStream::new(), |mut acc, tok| {
					acc.extend(tok);
					acc
				})
				.to_string()
			);
			assert_eq!(
				parse_from_file(ENTITY_FILES_WITH_SCHEMA_NAME[i].as_bytes())?.to_string(),
				Generator::gen_compact_code_blocks(
					entity,
					&crate::types::WithSerde::None,
					&crate::DateTimeCrate::Chrono,
					&Some("schema_name".to_owned()),
					false,
					false,
					&TokenStream::new(),
					&TokenStream::new(),
					false,
				)
				.into_iter()
				.skip(1)
				.fold(TokenStream::new(), |mut acc, tok| {
					acc.extend(tok);
					acc
				})
				.to_string()
			);
		}

		Ok(())
	}

	#[test]
	fn test_gen_with_serde() -> io::Result<()> {
		let cake_entity = setup().get(0).unwrap().clone();

		assert_eq!(cake_entity.get_table_name_snake_case(), "cake");

		// Compact code blocks
		assert_eq!(
			comparable_file_string(include_str!("../../tests/compact_with_serde/cake_none.rs"))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/compact_with_serde/cake_serialize.rs"
			))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::Serialize,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/compact_with_serde/cake_deserialize.rs"
			))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::Deserialize,
				&DateTimeCrate::Chrono,
				&None,
				true,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!("../../tests/compact_with_serde/cake_both.rs"))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::Both,
				&DateTimeCrate::Chrono,
				&None,
				true,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);

		// Expanded code blocks
		assert_eq!(
			comparable_file_string(include_str!("../../tests/expanded_with_serde/cake_none.rs"))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/expanded_with_serde/cake_serialize.rs"
			))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::Serialize,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/expanded_with_serde/cake_deserialize.rs"
			))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::Deserialize,
				&DateTimeCrate::Chrono,
				&None,
				true,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!("../../tests/expanded_with_serde/cake_both.rs"))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::Both,
				&DateTimeCrate::Chrono,
				&None,
				true,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);

		Ok(())
	}

	#[test]
	fn test_gen_with_seaography() -> io::Result<()> {
		let cake_entity = Entity {
			table_name: "cake".to_owned(),
			columns: vec![
				Column {
					name: "id".to_owned(),
					col_type: ColumnType::Integer,
					auto_increment: true,
					not_null: true,
					unique: false,
				},
				Column {
					name: "name".to_owned(),
					col_type: ColumnType::Text,
					auto_increment: false,
					not_null: false,
					unique: false,
				},
				Column {
					name: "base_id".to_owned(),
					col_type: ColumnType::Integer,
					auto_increment: false,
					not_null: false,
					unique: false,
				},
			],
			relations: vec![
				Relation {
					ref_table: "fruit".to_owned(),
					columns: vec![],
					ref_columns: vec![],
					rel_type: RelationType::HasMany,
					on_delete: None,
					on_update: None,
					self_referencing: false,
					num_suffix: 0,
					impl_related: true,
				},
				Relation {
					ref_table: "cake".to_owned(),
					columns: vec![],
					ref_columns: vec![],
					rel_type: RelationType::HasOne,
					on_delete: None,
					on_update: None,
					self_referencing: true,
					num_suffix: 0,
					impl_related: true,
				},
			],
			conjunct_relations: vec![ConjunctRelation {
				via: "cake_filling".to_owned(),
				to: "filling".to_owned(),
			}],
			primary_keys: vec![PrimaryKey {
				name: "id".to_owned(),
			}],
		};

		assert_eq!(cake_entity.get_table_name_snake_case(), "cake");

		// Compact code blocks
		assert_eq!(
			comparable_file_string(include_str!("../../tests/with_seaography/cake.rs"))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				true,
			))
		);

		// Expanded code blocks
		assert_eq!(
			comparable_file_string(include_str!("../../tests/with_seaography/cake_expanded.rs"))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				true,
			))
		);

		Ok(())
	}

	#[test]
	fn test_gen_with_derives() -> io::Result<()> {
		let mut cake_entity = setup().get_mut(0).unwrap().clone();

		assert_eq!(cake_entity.get_table_name_snake_case(), "cake");

		// Compact code blocks
		assert_eq!(
			comparable_file_string(include_str!("../../tests/compact_with_derives/cake_none.rs"))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!("../../tests/compact_with_derives/cake_one.rs"))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&bonus_derive(["ts_rs::TS"]),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/compact_with_derives/cake_multiple.rs"
			))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&bonus_derive(["ts_rs::TS", "utoipa::ToSchema"]),
				&TokenStream::new(),
				false,
			))
		);

		// Expanded code blocks
		assert_eq!(
			comparable_file_string(include_str!("../../tests/expanded_with_derives/cake_none.rs"))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!("../../tests/expanded_with_derives/cake_one.rs"))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&bonus_derive(["ts_rs::TS"]),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/expanded_with_derives/cake_multiple.rs"
			))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&bonus_derive(["ts_rs::TS", "utoipa::ToSchema"]),
				&TokenStream::new(),
				false,
			))
		);

		// Make the `name` column of `cake` entity as hidden column
		cake_entity.columns[1].name = "_name".into();

		assert_serde_variant_results(
			&cake_entity,
			&(
				include_str!("../../tests/compact_with_serde/cake_serialize_with_hidden_column.rs"),
				WithSerde::Serialize,
				None,
			),
			Box::new(Generator::gen_compact_code_blocks),
		)?;
		assert_serde_variant_results(
			&cake_entity,
			&(
				include_str!(
					"../../tests/expanded_with_serde/cake_serialize_with_hidden_column.rs"
				),
				WithSerde::Serialize,
				None,
			),
			Box::new(Generator::gen_expanded_code_blocks),
		)?;

		Ok(())
	}

	#[allow(clippy::type_complexity)]
	fn assert_serde_variant_results(
		cake_entity: &Entity,
		entity_serde_variant: &(&str, WithSerde, Option<String>),
		generator: Box<
			dyn Fn(
				&Entity,
				&WithSerde,
				&DateTimeCrate,
				&Option<String>,
				bool,
				bool,
				&TokenStream,
				&TokenStream,
				bool,
			) -> Vec<TokenStream>,
		>,
	) -> io::Result<()> {
		let mut reader = BufReader::new(entity_serde_variant.0.as_bytes());
		let mut lines: Vec<String> = Vec::new();
		let serde_skip_deserializing_primary_key =
			matches!(entity_serde_variant.1, WithSerde::Both | WithSerde::Deserialize);
		let serde_skip_hidden_column = matches!(entity_serde_variant.1, WithSerde::Serialize);

		reader.read_until(b'\n', &mut Vec::new())?;

		let mut line = String::new();
		while reader.read_line(&mut line)? > 0 {
			lines.push(line.to_owned());
			line.clear();
		}
		let content = lines.join("");
		let expected: TokenStream = content.parse().unwrap();
		println!("{:?}", entity_serde_variant.1);
		let generated = generator(
			cake_entity,
			&entity_serde_variant.1,
			&DateTimeCrate::Chrono,
			&entity_serde_variant.2,
			serde_skip_deserializing_primary_key,
			serde_skip_hidden_column,
			&TokenStream::new(),
			&TokenStream::new(),
			false,
		)
		.into_iter()
		.fold(TokenStream::new(), |mut acc, tok| {
			acc.extend(tok);
			acc
		});

		assert_eq!(expected.to_string(), generated.to_string());
		Ok(())
	}

	#[test]
	fn test_gen_with_attributes() -> io::Result<()> {
		let cake_entity = setup().get(0).unwrap().clone();

		assert_eq!(cake_entity.get_table_name_snake_case(), "cake");

		// Compact code blocks
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/compact_with_attributes/cake_none.rs"
			))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/compact_with_attributes/cake_one.rs"
			))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&bonus_attributes([r#"serde(rename_all = "camelCase")"#]),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/compact_with_attributes/cake_multiple.rs"
			))?,
			generated_to_string(Generator::gen_compact_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&bonus_attributes([r#"serde(rename_all = "camelCase")"#, "ts(export)"]),
				false,
			))
		);

		// Expanded code blocks
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/expanded_with_attributes/cake_none.rs"
			))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&TokenStream::new(),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/expanded_with_attributes/cake_one.rs"
			))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&bonus_attributes([r#"serde(rename_all = "camelCase")"#]),
				false,
			))
		);
		assert_eq!(
			comparable_file_string(include_str!(
				"../../tests/expanded_with_attributes/cake_multiple.rs"
			))?,
			generated_to_string(Generator::gen_expanded_code_blocks(
				&cake_entity,
				&WithSerde::None,
				&DateTimeCrate::Chrono,
				&None,
				false,
				false,
				&TokenStream::new(),
				&bonus_attributes([r#"serde(rename_all = "camelCase")"#, "ts(export)"]),
				false,
			))
		);

		Ok(())
	}

	fn generated_to_string(generated: Vec<TokenStream>) -> String {
		generated
			.into_iter()
			.fold(TokenStream::new(), |mut acc, tok| {
				acc.extend(tok);
				acc
			})
			.to_string()
	}

	fn comparable_file_string(file: &str) -> io::Result<String> {
		let mut reader = BufReader::new(file.as_bytes());
		let mut lines: Vec<String> = Vec::new();

		reader.read_until(b'\n', &mut Vec::new())?;

		let mut line = String::new();
		while reader.read_line(&mut line)? > 0 {
			lines.push(line.to_owned());
			line.clear();
		}
		let content = lines.join("");
		let expected: TokenStream = content.parse().unwrap();

		Ok(expected.to_string())
	}

	#[test]
	fn test_gen_postgres() -> io::Result<()> {
		let entities = vec![
			// This tests that the JsonBinary column type is annotated
			// correctly in compact entity form. More information can be found
			// in this issue:
			//
			// https://github.com/SeaQL/sea-orm/issues/1344
			Entity {
				table_name: "task".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "payload".to_owned(),
						col_type: ColumnType::Json,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "payload_binary".to_owned(),
						col_type: ColumnType::JsonBinary,
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
		];
		const ENTITY_FILES: [&str; 1] = [include_str!("../../tests/postgres/binary_json.rs")];

		const ENTITY_FILES_EXPANDED: [&str; 1] =
			[include_str!("../../tests/postgres/binary_json_expanded.rs")];

		assert_eq!(entities.len(), ENTITY_FILES.len());

		for (i, entity) in entities.iter().enumerate() {
			assert_eq!(
				parse_from_file(ENTITY_FILES[i].as_bytes())?.to_string(),
				Generator::gen_compact_code_blocks(
					entity,
					&crate::types::WithSerde::None,
					&crate::DateTimeCrate::Chrono,
					&None,
					false,
					false,
					&TokenStream::new(),
					&TokenStream::new(),
					false,
				)
				.into_iter()
				.skip(1)
				.fold(TokenStream::new(), |mut acc, tok| {
					acc.extend(tok);
					acc
				})
				.to_string()
			);
			assert_eq!(
				parse_from_file(ENTITY_FILES_EXPANDED[i].as_bytes())?.to_string(),
				Generator::gen_expanded_code_blocks(
					entity,
					&crate::types::WithSerde::None,
					&crate::DateTimeCrate::Chrono,
					&Some("schema_name".to_owned()),
					false,
					false,
					&TokenStream::new(),
					&TokenStream::new(),
					false,
				)
				.into_iter()
				.skip(1)
				.fold(TokenStream::new(), |mut acc, tok| {
					acc.extend(tok);
					acc
				})
				.to_string()
			);
		}

		Ok(())
	}

	#[test]
	fn test_gen_import_active_enum() -> io::Result<()> {
		let entities = vec![
			Entity {
				table_name: "tea_pairing".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "first_tea".to_owned(),
						col_type: ColumnType::Enum {
							name: SeaRc::new(Alias::new("tea_enum")),
							variants: vec![
								SeaRc::new(Alias::new("everyday_tea")),
								SeaRc::new(Alias::new("breakfast_tea")),
							],
						},
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "second_tea".to_owned(),
						col_type: ColumnType::Enum {
							name: SeaRc::new(Alias::new("tea_enum")),
							variants: vec![
								SeaRc::new(Alias::new("everyday_tea")),
								SeaRc::new(Alias::new("breakfast_tea")),
							],
						},
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
			Entity {
				table_name: "tea_pairing_with_size".to_owned(),
				columns: vec![
					Column {
						name: "id".to_owned(),
						col_type: ColumnType::Integer,
						auto_increment: true,
						not_null: true,
						unique: false,
					},
					Column {
						name: "first_tea".to_owned(),
						col_type: ColumnType::Enum {
							name: SeaRc::new(Alias::new("tea_enum")),
							variants: vec![
								SeaRc::new(Alias::new("everyday_tea")),
								SeaRc::new(Alias::new("breakfast_tea")),
							],
						},
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "second_tea".to_owned(),
						col_type: ColumnType::Enum {
							name: SeaRc::new(Alias::new("tea_enum")),
							variants: vec![
								SeaRc::new(Alias::new("everyday_tea")),
								SeaRc::new(Alias::new("breakfast_tea")),
							],
						},
						auto_increment: false,
						not_null: true,
						unique: false,
					},
					Column {
						name: "size".to_owned(),
						col_type: ColumnType::Enum {
							name: SeaRc::new(Alias::new("tea_size")),
							variants: vec![
								SeaRc::new(Alias::new("small")),
								SeaRc::new(Alias::new("medium")),
								SeaRc::new(Alias::new("huge")),
							],
						},
						auto_increment: false,
						not_null: true,
						unique: false,
					},
				],
				relations: vec![],
				conjunct_relations: vec![],
				primary_keys: vec![PrimaryKey {
					name: "id".to_owned(),
				}],
			},
		];

		assert_eq!(
			quote!(
				use super::sea_orm_active_enums::TeaEnum;
			)
			.to_string(),
			Generator::gen_import_active_enum(&entities[0]).to_string()
		);

		assert_eq!(
			quote!(
				use super::sea_orm_active_enums::TeaEnum;
				use super::sea_orm_active_enums::TeaSize;
			)
			.to_string(),
			Generator::gen_import_active_enum(&entities[1]).to_string()
		);

		Ok(())
	}
}
