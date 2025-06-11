use super::{Error, Result, TYPE_REGISTRY, Type, TypeRef, from_bytes, to_bytes};
use crate::{
	BoxFieldFutureByte, BoxResolverFn, ContextBase, FieldFuture, FieldValue, FieldValueInner,
	ObjectAccessor, ResolverContext, SeaResult, SeaographyError, TypeRefTrait, Value,
	proto::Message,
};
use binary::proto::{Decoder, DecoderLit, Encoder, EncoderLit};
use bytes::{Buf, BufMut, Bytes, BytesMut, buf};
use futures::FutureExt;
use prost::{
	DecodeError,
	encoding::{DecodeContext, WireType, decode_key, merge_loop},
};
use prost_types::{FieldDescriptorProto, FieldOptions, MethodDescriptorProto, MethodOptions};
use std::{
	collections::BTreeMap,
	fmt::{self, Debug},
	sync::Arc,
};

/// A Protobuf field
pub struct Field {
	pub(crate) name: String,
	pub(crate) tag: u32,
	pub(crate) description: Option<String>,
	pub(crate) arguments: BTreeMap<String, Field>,
	pub(crate) ty: TypeRef,
	pub(crate) repeated: bool,
	pub(crate) optional: bool,
	pub(crate) deprecated: bool,
	pub(crate) packed: bool,
	pub(crate) resolver_fn: Option<BoxResolverFn>,
	pub(crate) default_value: Option<Value>,
}

impl Debug for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Field")
			.field("description", &self.description)
			.field("name", &self.name)
			.field("ty", &self.ty)
			.field("repeated", &self.repeated)
			.field("optional", &self.optional)
			.field("deprecated", &self.deprecated)
			.field("packed", &self.packed)
			.finish()
	}
}

impl Field {
	/// Create a new Protobuf field output
	pub fn output<N, T, F>(name: N, tag: u32, ty: T, resolver_fn: F) -> Self
	where
		N: Into<String>,
		T: Into<TypeRef>,
		F: for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync + 'static,
	{
		Self {
			name: name.into(),
			tag,
			description: None,
			arguments: Default::default(),
			ty: ty.into(),
			repeated: false,
			optional: false,
			deprecated: false,
			packed: false,
			resolver_fn: Some(Box::new(resolver_fn)),
			default_value: None,
		}
	}

	/// Create a new Protobuf input field
	pub fn input<N, T>(name: N, tag: u32, ty: T) -> Self
	where
		N: Into<String>,
		T: Into<TypeRef>,
	{
		Self {
			name: name.into(),
			tag,
			description: None,
			arguments: Default::default(),
			ty: ty.into(),
			repeated: false,
			optional: false,
			deprecated: false,
			packed: false,
			resolver_fn: None,
			default_value: None,
		}
	}

	/// Set the field as repeated
	#[inline]
	pub fn set_repeated(mut self) -> Self {
		self.repeated = true;
		self
	}

	/// Set the field as optional
	#[inline]
	pub fn set_optional(mut self) -> Self {
		self.optional = true;
		self
	}

	/// Mark the field as deprecated
	#[inline]
	pub fn set_deprecated(mut self) -> Self {
		self.deprecated = true;
		self
	}

	/// Set the field as packed (for repeated fields of scalar types)
	#[inline]
	pub fn set_packed(mut self) -> Self {
		self.packed = true;
		self
	}

	/// Set the default value
	#[inline]
	pub fn default_value(self, value: impl Into<Value>) -> Self {
		Self {
			default_value: Some(value.into()),
			..self
		}
	}

	/// Add an argument to the field
	#[inline]
	pub fn argument(mut self, input_value: Field) -> Self {
		self.arguments.insert(input_value.name.clone(), input_value);
		self
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	#[inline]
	pub fn argument_by_name(&self, name: String) -> Option<&Self> {
		match self.arguments.iter().find(|(_, field)| field.name == name) {
			Some((_, field)) => Some(field),
			None => None,
		}
	}

	#[inline]
	pub fn argument_by_tag(&self, tag: u32) -> Option<&Self> {
		match self.arguments.iter().find(|(_, field)| field.tag == tag) {
			Some((_, field)) => Some(field),
			None => None,
		}
	}

	pub(crate) async fn encode<'a, B>(
		&self,
		buf: &mut B,
		ctx: &'a ContextBase,
		val: &'a FieldValue<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
		recursion: u32,
		is_response: bool,
	) -> SeaResult<usize>
	where
		B: BufMut,
	{
		match &val.0 {
			FieldValueInner::Value(value) => match self.ty.type_name() {
				TypeRef::DOUBLE
				| TypeRef::FLOAT
				| TypeRef::INT32
				| TypeRef::INT64
				| TypeRef::UINT32
				| TypeRef::UINT64
				| TypeRef::SINT32
				| TypeRef::SINT64
				| TypeRef::FIXED32
				| TypeRef::FIXED64
				| TypeRef::SFIXED32
				| TypeRef::SFIXED64
				| TypeRef::BOOL
				| TypeRef::STRING
				| TypeRef::BYTES => to_bytes(buf, value, self.tag, self.ty.type_name()),
				name => match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.to_value(buf, value, self.tag),
					None => {
						Err(SeaographyError::new(format!("Unsupported type for field `{}`", name)))
					}
				},
			},
			FieldValueInner::List(values) => {
				let mut size = 0;

				for value in values.iter() {
					size += Box::pin(self.encode(
						buf,
						ctx,
						value,
						arguments,
						parent_value,
						recursion,
						is_response,
					))
					.await?;
				}

				Ok(size)
			}
			FieldValueInner::OwnedAny(..) => match TYPE_REGISTRY.get(self.ty.type_name()) {
				Some(inner) => {
					let mut size = 0;
					let mut buffer = BytesMut::new();

					for field in inner.collect(ctx, arguments, Some(val), recursion) {
						let (field_size, fielf_buffer) = field.await?;
						if field_size > 0 {
							buffer.put_slice(&fielf_buffer[..]);
							size += field_size;
						}
					}

					if size > 0 {
						if inner.as_message().is_some() {
							if !is_response {
								prost::encoding::encode_key(
									self.tag,
									WireType::LengthDelimited,
									buf,
								);
								prost::encoding::encode_varint(size as u64, buf);
							}
						}
						buf.put_slice(&buffer[..]);
					}

					Ok(size)
				}
				None => Ok(0),
			},
			FieldValueInner::BorrowedAny(..) => match TYPE_REGISTRY.get(self.ty.type_name()) {
				Some(inner) => {
					let mut size = 0;
					let mut buffer = BytesMut::new();

					for field in inner.collect(ctx, arguments, Some(val), recursion) {
						let (field_size, fielf_buffer) = field.await?;
						if field_size > 0 {
							buffer.put_slice(&fielf_buffer[..]);
							size += field_size;
						}
					}

					if size > 0 {
						if inner.as_message().is_some() {
							if !is_response {
								prost::encoding::encode_key(
									self.tag,
									WireType::LengthDelimited,
									buf,
								);
								prost::encoding::encode_varint(size as u64, buf);
							}
						}
						buf.put_slice(&buffer[..]);
					}
					Ok(size)
				}
				None => Ok(0),
			},
			FieldValueInner::WithType {
				value,
				..
			} => {
				Box::pin(self.encode(
					buf,
					ctx,
					value,
					arguments,
					parent_value,
					recursion,
					is_response,
				))
				.await
			}
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
		mut recursion: u32,
		is_response: bool,
	) -> BoxFieldFutureByte<'a, BytesMut> {
		recursion += 1;
		async move {
			let resolve_fut = async {
				let parent_val = match parent_value {
					Some(val) => val,
					None => &FieldValue::NULL,
				};
				let field_future = match &self.resolver_fn {
					Some(resolver_fn) => (resolver_fn)(ResolverContext {
						ctx,
						args: arguments.clone(),
						parent_value: parent_val,
					}),
					None => FieldFuture::Value(FieldValue::NONE),
				};

				let field_value = match field_future {
					FieldFuture::Value(field_value) => field_value,
					FieldFuture::Future(future) => future.await?,
				};

				let mut buf = BytesMut::new();
				let size = match field_value {
					Some(val) => {
						self.encode(
							&mut buf,
							ctx,
							&val,
							arguments,
							parent_value,
							recursion,
							is_response,
						)
						.await?
					}
					None => match &self.default_value {
						Some(value) => {
							self.encode(
								&mut buf,
								ctx,
								&FieldValue::value(value.to_owned()),
								arguments,
								parent_value,
								recursion,
								is_response,
							)
							.await?
						}
						None => 0usize,
					},
				};
				Ok::<(usize, BytesMut), Error>((size, buf))
			};
			futures::pin_mut!(resolve_fut);

			Ok(resolve_fut.await?)
		}
		.boxed()
	}

	pub(crate) fn decode<B>(
		&self,
		buf: &mut B,
		ctx: DecodeContext,
		wire_type: WireType,
		arguments: &mut BTreeMap<Value, Value>,
	) -> Result<(), DecodeError>
	where
		B: Buf,
	{
		match self.ty.type_name() {
			TypeRef::DOUBLE
			| TypeRef::FLOAT
			| TypeRef::INT32
			| TypeRef::INT64
			| TypeRef::UINT32
			| TypeRef::UINT64
			| TypeRef::SINT32
			| TypeRef::SINT64
			| TypeRef::FIXED32
			| TypeRef::FIXED64
			| TypeRef::SFIXED32
			| TypeRef::SFIXED64
			| TypeRef::BOOL
			| TypeRef::STRING
			| TypeRef::BYTES => from_bytes(
				self.type_name(),
				self.ty.type_name(),
				self.ty.is_repeated(),
				ctx,
				buf,
				wire_type,
				arguments,
			),
			_ => {
				let mut argument = BTreeMap::new();

				match TYPE_REGISTRY.get(self.ty.type_name()) {
					Some(inner) => match &*inner {
						// Type::Scalar(scalar) => {
						// 	let mut value = self.bytes(buf, tag)?;
						// 	arguments.insert(Value::from(scalar.type_name()), value);
						// 	println!("name: {:?} scalar: {:?}", inner.type_name(), scalar);
						// }
						Type::Message(message) => {
							message.decode(buf, ctx, &mut argument)?;
						}
						Type::Enum(e) => {
							e.bytes(buf, ctx, wire_type, self.ty.is_repeated(), &mut argument)?;
						} // Type::Service(service) => {
						  // 	let mut value = service.decode(decoder, buf)?;
						  // 	arguments.insert(Value::from(service.type_name()), value.into());
						  // }
					},
					None => {
						return Err(DecodeError::new(format!(
							"Unknown type {}",
							self.ty.type_name()
						)));
					}
				};

				arguments.insert(Value::from(self.type_name()), Value::Map(argument));

				Ok(())
			}
		}
	}

	pub(crate) fn field_descriptor(&self, oneof: bool) -> FieldDescriptorProto {
		let mut field = FieldDescriptorProto::default();
		field.name = Some(self.name.clone());
		field.number = Some(self.tag as i32);
		field.label = Some(self.ty.field_label().into());
		field.r#type = Some(self.ty.field_type().into());
		field.type_name = Some(self.ty.type_name().to_string());
		if oneof {
			field.oneof_index = Some(self.tag as i32);
		}
		field.proto3_optional = Some(self.optional);
		//TODO: add default value
		// field.default_value = self.default_value.clone().map(|value| value.to_string());
		field.options = Some(FieldOptions {
			packed: Some(self.packed),
			deprecated: Some(self.deprecated),
			..Default::default()
		});

		field
	}

	pub(super) fn method_descriptor(&self, input_type: Option<String>) -> MethodDescriptorProto {
		let mut method = MethodDescriptorProto::default();
		method.name = Some(self.name.clone());
		method.input_type = input_type;
		method.output_type = Some(match self.ty.type_name() {
			TypeRef::DOUBLE => "google.protobuf.DoubleValue".to_string(),
			TypeRef::FLOAT => "google.protobuf.FloatValue".to_string(),
			TypeRef::INT32 => "google.protobuf.Int32Value".to_string(),
			TypeRef::INT64 => "google.protobuf.Int64Value".to_string(),
			TypeRef::UINT32 => "google.protobuf.UInt32Value".to_string(),
			TypeRef::UINT64 => "google.protobuf.UInt64Value".to_string(),
			TypeRef::SINT32 => "google.protobuf.Int32Value".to_string(),
			TypeRef::SINT64 => "google.protobuf.Int64Value".to_string(),
			TypeRef::FIXED32 => "google.protobuf.UInt32Value".to_string(),
			TypeRef::FIXED64 => "google.protobuf.UInt64Value".to_string(),
			TypeRef::SFIXED32 => "google.protobuf.Int32Value".to_string(),
			TypeRef::SFIXED64 => "google.protobuf.Int64Value".to_string(),
			TypeRef::BOOL => "google.protobuf.BoolValue".to_string(),
			TypeRef::STRING => "google.protobuf.StringValue".to_string(),
			TypeRef::BYTES => "google.protobuf.BytesValue".to_string(),
			name => name.to_string(),
		});
		method.options = Some(MethodOptions {
			deprecated: Some(self.deprecated),
			..Default::default()
		});

		method
	}
}
