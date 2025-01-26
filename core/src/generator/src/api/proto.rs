use heck::ToUpperCamelCase;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use sea_query::Write;
use std::ops::Add;
use syn::{punctuated::Punctuated, token::Comma};

use crate::entity::ActiveEnum;
use crate::entity::Column;
use crate::entity::Entity;
use crate::types::Operation;

pub struct Proto {}

impl Proto {
	pub fn gen_code_blocks(
		entity: &Entity,
		operation: &Operation,
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

		vec![Self::gen_message(data, operation)]
	}

	pub fn gen_message(entity: &Entity, operation: &Operation) -> TokenStream {
		let table_name =
			format_ident!("{}{}", operation.to_string(), entity.get_table_name_camel_case_ident());

		let attrs: Vec<TokenStream> = entity
			.columns
			.iter()
			.enumerate()
			.map(|(index, col)| {
				let mut attrs: Punctuated<_, Comma> = Punctuated::new();
				let name_snake_case = col.get_name_snake_case();

				if let Some(ts) = col.get_col_type_proto_attrs() {
					if !col.not_null {
						attrs.push(quote! { optional });
					}
					attrs.extend([ts]);
				};

				let tag = Literal::usize_unsuffixed(index.add(1)).to_token_stream();

				let mut ts = quote! {};
				if !attrs.is_empty() {
					for (i, attr) in attrs.into_iter().enumerate() {
						if i > 0 {
							ts = quote! { #ts };
						}
						ts = quote! { #ts #attr };
					}
					ts = quote! { #ts #name_snake_case = #tag; };
				}

				ts
			})
			.collect();

		quote! {
			message #table_name {
				#(
					#attrs
				)*
			}
		}
	}

	pub fn gen_active_enum(active_enum: &ActiveEnum) -> TokenStream {
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
                    #name = #tag;
                }
            });

		quote! {
			enum #enum_iden {
				#(
					#variants
				)*
			}
		}
	}

	pub fn gen_service(entity: &Entity, operations: Vec<Operation>) -> TokenStream {
		let table_name = entity.get_table_name_camel_case_ident();
		let rpcs = operations.iter().map(|operation| {
			let operation_name = format_ident!("{}", operation.to_string());
			let operation_type = format_ident!("{}{}", operation_name, table_name);
			let operation_return_type = format_ident!("Response{}", table_name);
			quote! {
				rpc #operation_name(#operation_type) returns (#operation_return_type);
			}
		});

		quote! {
			service #table_name {
				#(#rpcs)*
			}
		}
	}

	pub fn gen_proto_file(entity: &Vec<Entity>) -> TokenStream {
		let path: Vec<String> = entity
			.iter()
			.map(|entity| {
				format!(
					"./link/common/src/plugin/proto/{}.proto",
					entity.get_table_name_snake_case()
				)
			})
			.collect();

		let input = if path.len() == 1 {
			quote! {
				parser.input(#(#path)*)
			}
		} else {
			quote! {
				parser.inputs(
					#(
						#path,
					)*
				)
			}
		};

		quote! {
			use actix_web::Result;
			use prost::encoding::{
				decode_key, encode_varint, encoded_len_varint, message, DecodeContext, WireType,
			};
			use prost::Message;
			use prost_build::Config;
			use prost_types::{DescriptorProto, FileDescriptorProto};
			use std::collections::BTreeMap;

			#[derive(Clone, Debug)]
			pub struct DynamicMessage {
				pub name: String,
				pub descriptor: DescriptorProto,
				pub fields: BTreeMap<u32, FieldValue>,
			}

			#[derive(Clone, Debug)]
			pub enum FieldValue {
				Bool(bool),
				Bytes(Vec<u8>),
				Double(f64),
				Fixed32(u32),
				Fixed64(u64),
				Float(f32),
				Int32(i32),
				Int64(i64),
				Sfixed32(i32),
				Sfixed64(i64),
				Sint32(i32),
				Sint64(i64),
				String(String),
				Uint32(u32),
				Uint64(u64),
				Vec(Box<FieldValue>),
			}

			pub enum ComplexType {
				// BTreeMap(BTreeMap<u32, FieldValue<M>>),
				// Group(String),
				// Message(M),
				// HashMap(HashMap<FieldValue, FieldValue>),
			}

			impl FieldValue {
				pub fn encode(&self, tag: u32, buf: &mut impl prost::bytes::BufMut) {
					match self {
						FieldValue::Bool(val) => prost::encoding::bool::encode(tag, &val, buf),
						FieldValue::Bytes(val) => prost::encoding::bytes::encode(tag, val, buf),
						FieldValue::Double(val) => prost::encoding::double::encode(tag, &val, buf),
						FieldValue::Fixed32(val) => prost::encoding::fixed32::encode(tag, &val, buf),
						FieldValue::Fixed64(val) => prost::encoding::fixed64::encode(tag, &val, buf),
						FieldValue::Float(val) => prost::encoding::float::encode(tag, &val, buf),
						FieldValue::Int32(val) => prost::encoding::int32::encode(tag, &val, buf),
						FieldValue::Int64(val) => prost::encoding::int64::encode(tag, &val, buf),
						FieldValue::Sfixed32(val) => prost::encoding::sfixed32::encode(tag, &val, buf),
						FieldValue::Sfixed64(val) => prost::encoding::sfixed64::encode(tag, &val, buf),
						FieldValue::Sint32(val) => prost::encoding::sint32::encode(tag, &val, buf),
						FieldValue::Sint64(val) => prost::encoding::sint64::encode(tag, &val, buf),
						FieldValue::String(val) => prost::encoding::string::encode(tag, &val, buf),
						FieldValue::Uint32(val) => prost::encoding::uint32::encode(tag, &val, buf),
						FieldValue::Uint64(val) => prost::encoding::uint64::encode(tag, &val, buf),
						FieldValue::Vec(val) => val.encode(tag, buf),
						// FieldValue::HashMap(map) => {
						// 	for (key, value) in map.iter() {
						// 		let len = key.encoded_len(1) + value.encoded_len(1);

						// 		encode_key(tag, WireType::LengthDelimited, buf);
						// 		encode_varint(len as u64, buf);

						// 		key.encode(tag, buf);
						// 		value.encode(tag, buf);
						// 	}
						// }
					};
				}

				pub fn merge_field(
					&mut self,
					tag: u32,
					wire_type: WireType,
					buf: &mut impl prost::bytes::Buf,
					ctx: DecodeContext,
				) -> Result<(), prost::DecodeError> {
					match self {
						FieldValue::Bool(val) => prost::encoding::bool::merge(wire_type, val, buf, ctx),
						FieldValue::Bytes(val) => prost::encoding::bytes::merge(wire_type, val, buf, ctx),
						FieldValue::Double(val) => prost::encoding::double::merge(wire_type, val, buf, ctx),
						FieldValue::Fixed32(val) => prost::encoding::fixed32::merge(wire_type, val, buf, ctx),
						FieldValue::Fixed64(val) => prost::encoding::fixed64::merge(wire_type, val, buf, ctx),
						FieldValue::Float(val) => prost::encoding::float::merge(wire_type, val, buf, ctx),
						FieldValue::Int32(val) => prost::encoding::int32::merge(wire_type, val, buf, ctx),
						FieldValue::Int64(val) => prost::encoding::int64::merge(wire_type, val, buf, ctx),
						FieldValue::Sfixed32(val) => prost::encoding::sfixed32::merge(wire_type, val, buf, ctx),
						FieldValue::Sfixed64(val) => prost::encoding::sfixed64::merge(wire_type, val, buf, ctx),
						FieldValue::Sint32(val) => prost::encoding::sint32::merge(wire_type, val, buf, ctx),
						FieldValue::Sint64(val) => prost::encoding::sint64::merge(wire_type, val, buf, ctx),
						FieldValue::String(val) => prost::encoding::string::merge(wire_type, val, buf, ctx),
						FieldValue::Uint32(val) => prost::encoding::uint32::merge(wire_type, val, buf, ctx),
						FieldValue::Uint64(val) => prost::encoding::uint64::merge(wire_type, val, buf, ctx),
						FieldValue::Vec(val) => val.merge_field(tag, wire_type, buf, ctx),
					}
				}

				pub fn encoded_len(&self, tag: u32) -> usize {
					match self {
						FieldValue::Bool(val) => prost::encoding::bool::encoded_len(tag, val),
						FieldValue::Bytes(val) => prost::encoding::bytes::encoded_len(tag, val),
						FieldValue::Double(val) => prost::encoding::double::encoded_len(tag, val),
						FieldValue::Fixed32(val) => prost::encoding::fixed32::encoded_len(tag, val),
						FieldValue::Fixed64(val) => prost::encoding::fixed64::encoded_len(tag, val),
						FieldValue::Float(val) => prost::encoding::float::encoded_len(tag, val),
						FieldValue::Int32(val) => prost::encoding::int32::encoded_len(tag, val),
						FieldValue::Int64(val) => prost::encoding::int64::encoded_len(tag, val),
						FieldValue::Sfixed32(val) => prost::encoding::sfixed32::encoded_len(tag, val),
						FieldValue::Sfixed64(val) => prost::encoding::sfixed64::encoded_len(tag, val),
						FieldValue::Sint32(val) => prost::encoding::sint32::encoded_len(tag, val),
						FieldValue::Sint64(val) => prost::encoding::sint64::encoded_len(tag, val),
						FieldValue::String(val) => prost::encoding::string::encoded_len(tag, val),
						FieldValue::Uint32(val) => prost::encoding::uint32::encoded_len(tag, val),
						FieldValue::Uint64(val) => prost::encoding::uint64::encoded_len(tag, val),
						FieldValue::Vec(val) => val.encoded_len(tag),
					}
				}
			}

			impl Default for DynamicMessage {
				fn default() -> Self {
					DynamicMessage {
						descriptor: DescriptorProto::default(),
						name: String::new(),
						fields: BTreeMap::new(),
					}
				}
			}
			impl DynamicMessage {
				pub fn new() -> Self {
					Default::default()
				}
			}
			impl Message for DynamicMessage {
				fn encode_raw(&self, buf: &mut impl prost::bytes::BufMut)
				where
					Self: Sized,
				{
					if !self.fields.is_empty() {
						self.fields.clone().into_iter().for_each(|(tag, field)| field.encode(tag, buf));
					}
				}

				fn merge_field(
					&mut self,
					tag: u32,
					wire_type: WireType,
					buf: &mut impl prost::bytes::Buf,
					ctx: DecodeContext,
				) -> std::result::Result<(), prost::DecodeError>
				where
					Self: Sized,
				{
					match self.fields.get(&tag) {
						Some(field) => field.clone().merge_field(tag, wire_type, buf, ctx),
						None => prost::encoding::skip_field(wire_type, tag, buf, ctx),
					}?;

					Ok(())
				}

				fn encoded_len(&self) -> usize {
					self.fields.clone().into_iter().map(|(tag, field)| field.encoded_len(tag)).sum()
				}

				fn clear(&mut self) {
					self.fields.clear();
				}
			}

			pub struct Parser {
				file_descriptors: BTreeMap<String, FileDescriptorProto>,
				messages: BTreeMap<String, FileDescriptorProto>,
			}
			impl Parser {
				pub fn new() -> Self {
					Self {
						messages: BTreeMap::new(),
						file_descriptors: BTreeMap::new(),
					}
				}
				pub fn parse_proto(&mut self) -> Result<(), Box<dyn std::error::Error>> {
					let mut parser = protobuf_parse::Parser::new();

					parser.protoc();

					#input.include("./link/common/src/plugin/proto/");

					// let file_descriptor_set =
					// 	config.load_fds(&["src/frontend.proto", "src/backend.proto"], &["src"])?;

					Ok(())
				}
				// pub fn get_message(&self, name: &String) -> Result<&MessageDescriptor> {
				// 	match self.messages.get(name) {
				// 		Some(message) => Ok(message),
				// 		None => Err(actix_web::error::ErrorNotFound("Message not found")),
				// 	}
				// }
			}


		}
	}
}
