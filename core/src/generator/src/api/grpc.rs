use crate::{
	entity::{ActiveEnum, Column, Entity},
	types::{DateTimeCrate, Operation, WithSerde},
	util::escape_rust_keyword,
};
use heck::ToUpperCamelCase;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use sea_orm::ColumnType;
use std::fmt::Write;
use syn::{punctuated::Punctuated, token::Comma};

pub struct Grpc {}

impl Grpc {
	#[allow(clippy::too_many_arguments)]
	pub fn gen_code_blocks(
		entity: &Entity,
		with_serde: &WithSerde,
		date_time_crate: &DateTimeCrate,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: &TokenStream,
		model_extra_attributes: &TokenStream,
		operation: &Operation,
		primary_keys: Vec<Column>,
		primary_keys_name: Vec<String>,
	) -> Vec<TokenStream> {
		let column: Vec<Column> = entity
			.columns
			.clone()
			.into_iter()
			.filter(|col| {
				if *operation == Operation::Create || *operation == Operation::Update {
					!primary_keys_name.contains(&col.name)
				} else if *operation == Operation::Delete {
					primary_keys_name.contains(&col.name)
				} else {
					true
				}
			})
			.collect();

		let data = &Entity {
			columns: column,
			..entity.clone()
		};

		let mut res = vec![Self::gen_model_struct(
			data,
			with_serde,
			date_time_crate,
			serde_skip_deserializing_primary_key,
			serde_skip_hidden_column,
			model_extra_derives,
			model_extra_attributes,
			operation,
			primary_keys_name,
		)];

		res.push(Self::gen_impl_route(data, primary_keys, operation, date_time_crate));

		res
	}

	#[allow(clippy::too_many_arguments)]
	pub fn gen_model_struct(
		entity: &Entity,
		with_serde: &WithSerde,
		date_time_crate: &DateTimeCrate,
		serde_skip_deserializing_primary_key: bool,
		serde_skip_hidden_column: bool,
		model_extra_derives: &TokenStream,
		model_extra_attributes: &TokenStream,
		operation: &Operation,
		primary_keys_name: Vec<String>,
	) -> TokenStream {
		let table_name =
			format_ident!("{}{}", operation.to_string(), entity.get_table_name_camel_case_ident());
		let column_names_snake_case = entity.get_column_names_snake_case();
		let column_rs_types = entity.get_column_std_rs_types(date_time_crate, Some(operation));
		let if_eq_needed = entity.get_eq_needed();

		let attrs: Vec<TokenStream> = entity
			.columns
			.iter()
			.enumerate()
			.map(|(index, col)| {
				let tag = index.to_string();
				let mut attrs: Punctuated<_, Comma> = Punctuated::new();
				let is_primary_key = primary_keys_name.contains(&col.name);
				if let Some(ts) = col.get_col_type_grpc_attrs() {
					attrs.extend([ts]);
					if !col.not_null
						|| *operation == Operation::Update
						|| *operation == Operation::List
						|| *operation == Operation::Get
					{
						attrs.push(quote! { optional });
					}
				};
				let mut ts = quote! {};
				if !attrs.is_empty() {
					for (i, attr) in attrs.into_iter().enumerate() {
						if i > 0 {
							ts = quote! { #ts, };
						}
						ts = quote! { #ts #attr };
					}
					ts = quote! { #[prost(#ts, tag=#tag)] };
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

		let extra_derive = with_serde.extra_derive();

		quote! {
			#[derive(Clone, PartialEq, Message #if_eq_needed #extra_derive #model_extra_derives)]
			#model_extra_attributes
			pub struct #table_name {
				#(
					#attrs
					pub #column_names_snake_case: #column_rs_types,
				)*
			}
		}
	}

	pub fn gen_active_enum(
		active_enum: &ActiveEnum,
		with_serde: &WithSerde,
		with_copy_enums: bool,
		extra_derives: &TokenStream,
		extra_attributes: &TokenStream,
	) -> TokenStream {
		let enum_name = active_enum.get_active_enum_name_snake_case();
		let enum_iden = active_enum.get_active_enum_name_camel_case_ident();

		let values: Vec<String> = active_enum.values.iter().map(|v| v.to_string()).collect();
		let variants = values.iter().map(|v| v.trim()).enumerate().map(|(index, v)| {
                let name = if v.chars().next().map(char::is_numeric).unwrap_or(false) {
                    format_ident!("_{}", v)
                } else {
                    let variant_name = v.to_upper_camel_case();
                    if variant_name.is_empty() {
                        println!("Warning: item '{}' in the enumeration '{}' cannot be converted into a valid Rust enum member name. It will be converted to its corresponding UTF-8 encoding. You can modify it later as needed.", v, enum_name);
                        let mut ss = String::new();
                        for c in v.chars() {
                            if c.len_utf8() > 1 {
                                write!(&mut ss, "{c}").unwrap();
                            } else {
                                write!(&mut ss, "U{:04X}", c as u32).unwrap();
                            }
                        }
                        format_ident!("{}", ss)
                    } else {
                        format_ident!("{}", variant_name)
                    }
                };

                let tag = Literal::usize_unsuffixed(index).to_token_stream();

                quote! {
                    #name = #tag,
                }
            });

		let serde_derive = with_serde.extra_derive();
		let copy_derive = if with_copy_enums {
			quote! { , Copy }
		} else {
			quote! {}
		};

		quote! {
			#[derive(Debug, Clone, PartialEq, Eq, Enumeration #copy_derive #serde_derive #extra_derives)]
			#[repr(i32)]
			#extra_attributes
			pub enum #enum_iden {
				#(
					#variants
				)*
			}
		}
	}

	pub fn gen_into_enum(active_enum: &ActiveEnum) -> TokenStream {
		let enum_iden = active_enum.get_active_enum_name_camel_case_ident();

		let columns: Vec<TokenStream> = active_enum
			.values
			.iter()
			.map(|val| {
				let name = format_ident!("{}", val.to_string().to_upper_camel_case());
				quote! {
					#enum_iden::#name => sea_orm_active_enums::#enum_iden::#name,
				}
			})
			.collect();

		quote! {
			impl From<#enum_iden> for sea_orm_active_enums::#enum_iden {
				fn from(model: #enum_iden) -> Self {

					match model {
						#(#columns)*
					}
				}
			}
		}
	}

	pub fn gen_into_enum_string(active_enum: &ActiveEnum) -> TokenStream {
		let enum_iden = active_enum.get_active_enum_name_camel_case_ident();

		let columns: Vec<TokenStream> = active_enum
			.values
			.iter()
			.map(|val| {
				let val_type = val.to_string().to_upper_camel_case();
				let name = format_ident!("{}", val_type);
				quote! {
					#enum_iden::#name => #val_type.to_string(),
				}
			})
			.collect();

		quote! {
			impl #enum_iden {
				fn to_string(&self) -> String {
					match self {
						#(#columns)*
					}
				}
			}
		}
	}

	pub fn gen_impl_route(
		entity: &Entity,
		primary_keys: Vec<Column>,
		operation: &Operation,
		date_time_crate: &DateTimeCrate,
	) -> TokenStream {
		match *operation {
			Operation::Get => Self::gen_impl_route_get(entity),
			Operation::List => Self::gen_impl_route_list(entity),
			Operation::Create => Self::gen_impl_route_create(entity, primary_keys, date_time_crate),
			Operation::Update => Self::gen_impl_route_update(entity, primary_keys, date_time_crate),
			Operation::Delete => Self::gen_impl_route_delete(entity, primary_keys),
			Operation::Response => Self::gen_into_responce(entity),
			Operation::Custom(_) => TokenStream::new(),
		}
	}

	pub fn gen_impl_route_get(entity: &Entity) -> TokenStream {
		let name = format_ident!("{}Column", entity.get_table_name_camel_case_ident());
		let table = entity.get_table_name_camel_case_ident();
		let table_name = format_ident!("Get{}", entity.get_table_name_camel_case_ident());
		let table_response = format_ident!("Response{}", entity.get_table_name_camel_case_ident());

		let filter: Vec<TokenStream> = entity
			.columns
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();
				let name_camel_case = col.get_name_camel_case();

				quote! {
					if let Some(#name_snake_case) = &model.#name_snake_case {
						query = query.filter(#name::#name_camel_case.eq(#name_snake_case.clone()));
					}
				}
			})
			.collect();

		quote! {
			impl #table_name {
				async fn route(
					db: web::Data<DatabaseConnection>,
					proto_data: ProtoBuf<#table_name>
				) -> Result<HttpResponse, actix_web::Error> {
					let model = proto_data.0;

					let mut query = #table::find();

					#(#filter)*

					let result = query.one(db.get_ref()).await;

					match result {
						Ok(response) => match response {
							Some(res) => HttpResponse::Ok().protobuf::<#table_response>(res.into()),
							None => Ok(HttpResponse::NotFound().finish()),
						},
						Err(err) => Ok(HttpResponse::InternalServerError().body(format!("Database query error: {}", err))),
					}
				}
			}
		}
	}

	pub fn gen_impl_route_list(entity: &Entity) -> TokenStream {
		let name = format_ident!("{}Column", entity.get_table_name_camel_case_ident());
		let table = entity.get_table_name_camel_case_ident();
		let table_name = format_ident!("List{}", entity.get_table_name_camel_case_ident());
		let table_response = format_ident!("Response{}", entity.get_table_name_camel_case_ident());

		let filter: Vec<TokenStream> = entity
			.columns
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();
				let name_camel_case = col.get_name_camel_case();

				quote! {
					if let Some(#name_snake_case) = &model.#name_snake_case {
						query = query.filter(#name::#name_camel_case.eq(#name_snake_case.clone()));
					}
				}
			})
			.collect();

		quote! {
			impl #table_name {
				async fn route(
					db: web::Data<DatabaseConnection>,
					proto_data: ProtoBuf<#table_name>
				) -> Result<HttpResponse, actix_web::Error> {
					let model = proto_data.0;

					let mut query = #table::find();

					#(#filter)*

					let result = query.all(db.get_ref()).await;

					match result {
						Ok(responses) => {
							let mut buf = Vec::new();
							for response in responses {
								let model: #table_response = response.into();
								model.encode(&mut buf).map_err(ProtoBufPayloadError::Serialize)?;
							}
							Ok(HttpResponse::Ok().content_type("application/protobuf").body(buf))
						},
						Err(err) => Ok(HttpResponse::InternalServerError().body(format!("Database query error: {}", err))),
					}
				}
			}
		}
	}

	pub fn gen_impl_route_create(
		entity: &Entity,
		primary_keys: Vec<Column>,
		date_time_crate: &DateTimeCrate,
	) -> TokenStream {
		let name = format_ident!("{}ActiveModel", entity.get_table_name_camel_case_ident());
		let table_name = format_ident!("Create{}", entity.get_table_name_camel_case_ident());
		let table_response = format_ident!("Response{}", entity.get_table_name_camel_case_ident());

		let mut convertions: Vec<TokenStream> = vec![];

		let columns: Vec<TokenStream> = entity
			.columns
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();
				let column_type = col.get_inner_col_type();

				let column_name = match column_type {
					ColumnType::Decimal(_) | ColumnType::Money(_) => {
						if !col.not_null {
							quote! {
								*value
							}
						} else {
							quote! {
								model.#name_snake_case
							}
						}
					}
					_ => {
						if !col.not_null {
							quote! {
								value
							}
						} else {
							quote! {
								&model.#name_snake_case
							}
						}
					}
				};

				let convertion = Self::convertions(
					column_type,
					column_name,
					date_time_crate,
					!col.not_null,
					true,
				);

				match convertion {
					Some(convertion) => {
						if !col.not_null {
							convertions.push(quote! {
								let #name_snake_case = if let Some(value) = &model.#name_snake_case {
									#convertion
								} else {
									None
								};
							});
						} else {
							convertions.push(quote! {
								let #name_snake_case = #convertion;
							});
						}

						quote! {
							#name_snake_case: Set(#name_snake_case),
						}
					}
					None => quote! {
						#name_snake_case: Set(model.#name_snake_case.clone()),
					},
				}
			})
			.collect();

		let primary_columns = Self::generate_primary_keys(primary_keys);

		quote! {
			impl #table_name {
				async fn route(
					db: web::Data<DatabaseConnection>,
					proto_data: ProtoBuf<#table_name>
				) -> Result<HttpResponse, actix_web::Error> {
					let model = proto_data.0;

					#(#convertions)*

					let new_table = #name {
						#(#primary_columns)*
						#(#columns)*
					};

					let insert_result = new_table
						.insert(db.get_ref())
						.await;

					match insert_result {
						Ok(res) => HttpResponse::Created().protobuf::<#table_response>(res.into()),
						Err(err) => Ok(HttpResponse::InternalServerError().body(format!("Failed to insert: {}", err)))
					}
				}
			}
		}
	}

	pub fn gen_impl_route_update(
		entity: &Entity,
		primary_keys: Vec<Column>,
		date_time_crate: &DateTimeCrate,
	) -> TokenStream {
		let name = format_ident!("{}ActiveModel", entity.get_table_name_camel_case_ident());
		let table = entity.get_table_name_camel_case_ident();
		let table_name = format_ident!("Update{}", entity.get_table_name_camel_case_ident());
		let table_response = format_ident!("Response{}", entity.get_table_name_camel_case_ident());

		let columns: Vec<TokenStream> = entity
			.columns
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();
				let column_type = col.get_inner_col_type();

				let column_name = match column_type {
					ColumnType::Decimal(_) | ColumnType::Money(_) => {
						quote! {
							*value
						}
					}
					_ => {
						quote! {
							value
						}
					}
				};

				let convertion = Self::convertions(
					column_type,
					column_name,
					date_time_crate,
					!col.not_null,
					true,
				);

				match convertion {
					Some(convertion) => {
						quote! {
							if let Some(value) = &model.#name_snake_case {
								let #name_snake_case = #convertion;
								model_active.#name_snake_case = Set(#name_snake_case);
							}
						}
					}
					None => {
						if !col.not_null {
							quote! {
								model_active.#name_snake_case = Set(model.#name_snake_case);
							}
						} else {
							quote! {
								if let Some(#name_snake_case) = &model.#name_snake_case {
									model_active.#name_snake_case = Set(#name_snake_case.clone());
								}
							}
						}
					}
				}
			})
			.collect();

		let path_name = format_ident!("{}Parmns", entity.get_table_name_camel_case_ident());
		let primary_path = Self::build_primary_keys(primary_keys);

		quote! {
			impl #table_name {
				async fn route(
					db: web::Data<DatabaseConnection>,
					path: web::Path<#path_name>,
					proto_data: ProtoBuf<#table_name>
				) -> Result<HttpResponse, actix_web::Error> {
					let model = proto_data.0;

					let old_table = #table::find_by_id(#primary_path)
						.one(db.get_ref())
						.await;

					match old_table {
						Ok(old) => {
							match old {
								Some(table) => {
									let mut model_active: #name = table.into();

									#(#columns)*

									let update_result = model_active.update(db.get_ref()).await;

									match update_result {
										Ok(res) => HttpResponse::Ok().protobuf::<#table_response>(res.into()),
										Err(err) => Ok(HttpResponse::InternalServerError().body(format!("Failed to update: {}", err)))
									}
								},
								None => Ok(HttpResponse::NotFound().finish()),
							}

						}
						Err(err) => Ok(HttpResponse::InternalServerError().body(format!("Failed to find table to update: {}", err)))
					}
				}
			}
		}
	}

	pub fn gen_impl_route_delete(entity: &Entity, primary_keys: Vec<Column>) -> TokenStream {
		let table = entity.get_table_name_camel_case_ident();
		let table_name = format_ident!("Delete{}", entity.get_table_name_camel_case_ident());

		let path_name = format_ident!("{}Parmns", entity.get_table_name_camel_case_ident());
		let primary_path = Self::build_primary_keys(primary_keys);

		quote! {
			impl #table_name {
				async fn route(
					db: web::Data<DatabaseConnection>,
					path: web::Path<#path_name>,
				) -> impl Responder {
					let res = #table::delete_by_id(#primary_path).exec(db.get_ref()).await;

					match res {
						Ok(_) => HttpResponse::Ok().finish(),
						Err(err) => HttpResponse::InternalServerError().body(format!("Failed to delete: {}", err))
					}
				}
			}
		}
	}

	pub fn gen_into_responce(entity: &Entity) -> TokenStream {
		let table_name = format_ident!("Response{}", entity.get_table_name_camel_case_ident());
		let table = format_ident!("{}Model", entity.get_table_name_camel_case_ident());

		let columns: Vec<TokenStream> = entity
			.columns
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();
				let column_type = col.get_inner_col_type();

				let convertions = match column_type {
					ColumnType::Json | ColumnType::JsonBinary => Some(if !col.not_null {
						quote! {
							Some(serde_json::to_string(&model.#name_snake_case).unwrap_or_default())
						}
					} else {
						quote! {
							serde_json::to_string(&model.#name_snake_case).unwrap_or_default()
						}
					}),
					ColumnType::Date
					| ColumnType::Time
					| ColumnType::DateTime
					| ColumnType::Timestamp
					| ColumnType::TimestampWithTimeZone => Some(if !col.not_null {
						quote! {
							model.#name_snake_case.map(|v| v.to_rfc3339())
						}
					} else {
						quote! {
							model.#name_snake_case.to_string()
						}
					}),
					ColumnType::Decimal(_) => Some(if !col.not_null {
						quote! {
							model.#name_snake_case.to_f32()
						}
					} else {
						quote! {
							model.#name_snake_case.to_f32().unwrap_or(0.0)
						}
					}),
					ColumnType::Enum {
						..
					} => Some(if !col.not_null {
						quote! {
							model.#name_snake_case.map(|val| val.to_i32())
						}
					} else {
						quote! {
							model.#name_snake_case.to_i32()
						}
					}),
					_ => None,
				};

				match convertions {
					Some(convertion) => {
						quote! {
							#name_snake_case: #convertion,
						}
					}
					None => quote! {
						#name_snake_case: model.#name_snake_case.clone(),
					},
				}
			})
			.collect();

		quote! {
			impl From<#table> for #table_name {
				fn from(model: #table) -> Self {

					#table_name {
						#(#columns)*
					}
				}
			}
		}
	}

	pub fn gen_route(entity: &Entity) -> TokenStream {
		let name = format!("/{}", entity.get_table_name_snake_case());
		let name_get = format_ident!("Get{}", entity.get_table_name_camel_case());
		let name_list = format_ident!("List{}", entity.get_table_name_camel_case());
		let name_create = format_ident!("Create{}", entity.get_table_name_camel_case());
		let name_update = format_ident!("Update{}", entity.get_table_name_camel_case());
		let name_delete = format_ident!("Delete{}", entity.get_table_name_camel_case());
		quote! {
			pub fn configure(cfg: &mut web::ServiceConfig) {
				cfg.service(
					web::scope(#name)
						.route("/", web::get().to(#name_get::route))
						.route("/", web::post().to(#name_create::route))
						.route("/list", web::get().to(#name_list::route))
						.route("/{id}", web::put().to(#name_update::route))
						.route("/{id}", web::get().to(#name_delete::route))
				);
			}
		}
	}

	pub fn gen_router(entitys: &Vec<Entity>) -> TokenStream {
		let configures: Vec<TokenStream> = entitys
			.iter()
			.map(|entity| {
				let table_name_snake_case_ident = format_ident!(
					"{}",
					escape_rust_keyword(entity.get_table_name_snake_case_ident())
				);

				quote! {
					.configure(#table_name_snake_case_ident::configure)
				}
			})
			.collect();

		quote! {
			pub fn router(cfg: &mut web::ServiceConfig) {
				cfg.service(
					web::scope("/grpc")
						#(#configures)*
				);
			}
		}
	}

	fn convertions(
		column_type: &ColumnType,
		column_name: TokenStream,
		date_time_crate: &DateTimeCrate,
		with_output_option: bool,
		error_as_responce: bool,
	) -> Option<TokenStream> {
		let output = if with_output_option {
			quote! {
				Ok(output) => Some(output)
			}
		} else {
			quote! {
				Ok(output) => output
			}
		};

		let error_type = if error_as_responce {
			quote! {
				Ok
			}
		} else {
			quote! {
				Err
			}
		};

		match column_type {
			ColumnType::Json | ColumnType::JsonBinary => Some(quote! {
				match serde_json::from_str(#column_name) {
					Ok(output) => output,
					Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse JSON: {}", err))),
				}
			}),
			ColumnType::Date => Some(match date_time_crate {
				DateTimeCrate::Chrono => quote! {
					match Date::parse_from_str(#column_name, "%Y-%m-%d") {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
				DateTimeCrate::Time => quote! {
					match TimeDate::parse(#column_name, "%Y-%m-%d") {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
			}),
			ColumnType::Time => Some(match date_time_crate {
				DateTimeCrate::Chrono => quote! {
					match Time::parse_from_str(#column_name, "%H:%M:%S") {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
				DateTimeCrate::Time => quote! {
					match TimeTime::parse(#column_name, "%H:%M:%S") {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
			}),
			ColumnType::DateTime => Some(match date_time_crate {
				DateTimeCrate::Chrono => quote! {
					match DateTime::parse_from_str(#column_name, "%Y-%m-%d %H:%M:%S") {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
				DateTimeCrate::Time => quote! {
					match TimeDateTime::parse(#column_name, "%Y-%m-%d %H:%M:%S") {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
			}),
			ColumnType::Timestamp => Some(match date_time_crate {
				DateTimeCrate::Chrono => quote! {
					match DateTimeUtc::parse_from_rfc3339(#column_name) {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
				DateTimeCrate::Time => quote! {
					match TimeDateTime::parse(#column_name, &time::format_description::well_known::Rfc3339) {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
			}),
			ColumnType::TimestampWithTimeZone => Some(match date_time_crate {
				DateTimeCrate::Chrono => quote! {
					match DateTimeWithTimeZone::parse_from_rfc3339(#column_name) {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
				DateTimeCrate::Time => quote! {
					match TimeDateTimeWithTimeZone::parse(#column_name, &time::format_description::well_known::Rfc3339) {
						#output,
						Err(err) => return #error_type(HttpResponse::InternalServerError().body(format!("Failed to parse date: {}", err))),
					}
				},
			}),
			ColumnType::Decimal(_) => Some(if with_output_option {
				quote! {
					Decimal::from_f32_retain(#column_name)
				}
			} else {
				quote! {
					match Decimal::from_f32_retain(#column_name) {
						Some(output) => output,
						None => return #error_type(HttpResponse::InternalServerError().body("Failed to parse decimal")),
					}
				}
			}),
			ColumnType::Enum {
				name,
				..
			} => {
				let enum_iden = format_ident!("{}", name.to_string().to_upper_camel_case());

				Some(quote! {
					match sea_orm_active_enums::#enum_iden::from_i32(*#column_name) {
						#output,
						Err(err) => return #error_type(err),
					}
				})
			}
			_ => None,
		}
	}

	fn build_primary_keys(primary_keys: Vec<Column>) -> TokenStream {
		let primary_names: Vec<TokenStream> = primary_keys
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();

				quote! {
					path.#name_snake_case.clone(),
				}
			})
			.collect();

		if primary_names.len() == 1 {
			quote! {
				#(#primary_names)*
			}
		} else {
			quote! {
				(#(#primary_names)*)
			}
		}
	}

	fn generate_primary_keys(primary_keys: Vec<Column>) -> Vec<TokenStream> {
		primary_keys
			.iter()
			.map(|col| {
				let name_snake_case = col.get_name_snake_case();

				let column_type = match col.get_inner_col_type() {
					ColumnType::Char(_)
					| ColumnType::String(_)
					| ColumnType::Text
					| ColumnType::Custom(_) => quote! { "id".to_string() },
					ColumnType::TinyInteger
					| ColumnType::SmallInteger
					| ColumnType::Integer
					| ColumnType::BigInteger => quote! { 0 },
					ColumnType::TinyUnsigned
					| ColumnType::SmallUnsigned
					| ColumnType::Unsigned
					| ColumnType::BigUnsigned => quote! { 0 },
					ColumnType::Float => quote! { 0.0 },
					ColumnType::Double => quote! { 0.0 },
					ColumnType::Json | ColumnType::JsonBinary => quote! { "id".to_string() },
					ColumnType::Date => quote! { "id".to_string() },
					ColumnType::Time => quote! { "id".to_string() },
					ColumnType::DateTime => quote! { "id".to_string() },
					ColumnType::Timestamp => quote! { "id".to_string() },
					ColumnType::TimestampWithTimeZone => quote! { "id".to_string() },
					ColumnType::Decimal(_) | ColumnType::Money(_) => quote! { "id".to_string() },
					ColumnType::Uuid => quote! { "id".to_string() },
					_ => quote! { "id".to_string() },
				};

				quote! {
					#name_snake_case: Set(#column_type),
				}
			})
			.collect()
	}
}
