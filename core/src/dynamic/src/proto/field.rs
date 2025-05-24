use super::{Error, Result, Type, TypeRef, get_type};
use crate::{
	BoxFieldFuture, BoxResolverFn, ContextBase, EnumTrait, FieldFuture, FieldValue,
	FieldValueInner, FieldValueTrait, ObjectAccessor, ResolverContext, TypeRefTrait, Value,
};
use binary::proto::{Decoder, DecoderLit, Encoder, EncoderLit};
use futures_util::FutureExt;
use std::{
	collections::BTreeMap,
	fmt::{self, Debug},
};

/// A Protobuf field
pub struct Field {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) arguments: BTreeMap<String, Field>,
	pub(crate) tag: u32,
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
			.field("tag", &self.tag)
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
			description: None,
			tag,
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
			description: None,
			tag,
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
	pub fn argument_by_tag(&self, tag: u32) -> Option<&Self> {
		match self.arguments.iter().find(|(_, field)| field.tag == tag) {
			Some((_, field)) => Some(field),
			None => None,
		}
	}

	pub(crate) async fn to_value<'a>(
		&self,
		ctx: &'a ContextBase,
		val: &'a FieldValue<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Result<Value> {
		match &val.0 {
			FieldValueInner::Value(val) => Ok(val.clone()),
			FieldValueInner::List(values) => {
				let mut list = Vec::new();
				for value in values.iter() {
					list.push(Box::pin(self.to_value(ctx, value, arguments, parent_value)).await?);
				}

				Ok(Value::List(list))
			}
			FieldValueInner::OwnedAny(_) => match get_type(self.ty.type_name()) {
				Some(inner) => {
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, arguments, Some(val)) {
						let (name, value) = field.await?;

						data.insert(name, value);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::BorrowedAny(_) => match get_type(self.ty.type_name()) {
				Some(inner) => {
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, arguments, Some(val)) {
						let (name, value) = field.await?;

						data.insert(name, value);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::WithType {
				value,
				..
			} => Box::pin(self.to_value(ctx, value, arguments, parent_value)).await,
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> BoxFieldFuture<'a> {
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

				let value = match field_value {
					Some(val) => self.to_value(ctx, &val, arguments, parent_value).await?,
					None => match &self.default_value {
						Some(value) => value.clone(),
						None => Value::Null,
					},
				};
				Ok::<Value, Error>(value)
			};
			futures_util::pin_mut!(resolve_fut);

			Ok((Value::from(self.name.clone()), resolve_fut.await?))
		}
		.boxed()
	}

	pub(crate) fn encode(
		&self,
		encoder: &Encoder,
		buf: &mut Vec<u8>,
		value: Value,
	) -> Result<usize> {
		match value {
			Value::Int8(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Int32(&i32::from(data))), buf)?)
			}
			Value::Int16(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Int32(&i32::from(data))), buf)?)
			}
			Value::Int32(data) => Ok(encoder.encode((&self.tag, EncoderLit::Int32(&data)), buf)?),
			Value::Int64(data) => Ok(encoder.encode((&self.tag, EncoderLit::Int64(&data)), buf)?),
			Value::UInt8(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::UInt32(&u32::from(data))), buf)?)
			}
			Value::UInt16(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::UInt32(&u32::from(data))), buf)?)
			}
			Value::UInt32(data) => Ok(encoder.encode((&self.tag, EncoderLit::UInt32(&data)), buf)?),
			Value::UInt64(data) => Ok(encoder.encode((&self.tag, EncoderLit::UInt64(&data)), buf)?),
			Value::Float32(data) => Ok(encoder.encode((&self.tag, EncoderLit::Float(&data)), buf)?),
			Value::Float64(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Double(&data)), buf)?)
			}
			Value::Bool(data) => Ok(encoder.encode((&self.tag, EncoderLit::Bool(&data)), buf)?),
			Value::Char(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Bytes(&vec![data as u8])), buf)?)
			}
			Value::String(data) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Bytes(&data.into_bytes())), buf)?)
			}
			Value::List(values) => {
				values.iter().map(|value| self.encode(encoder, buf, value.to_owned())).sum()
			}
			Value::Map(btree_map) => {
				btree_map.iter().map(|(_, value)| self.encode(encoder, buf, value.to_owned())).sum()
			}
			Value::Option(value) => match *value {
				Some(data) => self.encode(encoder, buf, data),
				None => Ok(0),
			},
			Value::Var(data) => {
				let val = *data;
				Ok(self.encode(encoder, buf, val.0)?)
			}
			Value::Null => Ok(0),
			name => {
				Err(Error::new(format!("Unsupported type or wire type combination: {:?}", name)))
			}
		}
	}

	pub(crate) fn decode(
		&self,
		decoder: &mut Decoder,
		buf: Vec<u8>,
		tag: u32,
		arguments: &mut BTreeMap<Value, Value>,
	) -> Result<()> {
		match get_type(self.ty.type_name()) {
			Some(inner) => match &*inner {
				Type::Scalar(scalar) => {
					let value = self.bytes(buf, tag)?;
					arguments.insert(Value::from(scalar.type_name()), value);
					println!("name: {:?} scalar: {:?}", inner.name(), scalar);
				}
				Type::Message(data) => {
					let value = data.decode(decoder, buf)?;
					arguments.insert(Value::from(data.type_name()), value.into());
				}
				Type::Enum(e) => {
					let value = e.bytes(buf)?;
					arguments.insert(Value::from(e.type_name()), value);
				}
				Type::Service(service) => {
					let value = service.decode(decoder, buf)?;
					arguments.insert(Value::from(service.type_name()), value.into());
				}
			},
			None => return Err(Error::new(format!("Unknown type {}", self.ty.type_name()))),
		}

		Ok(())
	}

	pub(crate) fn bytes(&self, buf: Vec<u8>, tag: u32) -> Result<Value> {
		if tag != self.tag {
			return Err(Error::new(format!("invalid tag: expected {}, got {}", self.tag, tag)));
		}
		match self.ty.type_name() {
			TypeRef::INT32 => match self.ty.is_repeated() {
				false => Ok(i32::from(DecoderLit::Int32(buf)).into()),
				true => Ok(Vec::<i32>::from(DecoderLit::Int32Vec(buf)).into()),
			},
			TypeRef::INT64 => match self.ty.is_repeated() {
				false => Ok(i64::from(DecoderLit::Int64(buf)).into()),
				true => Ok(Vec::<i64>::from(DecoderLit::Int64Vec(buf)).into()),
			},
			TypeRef::UINT32 => match self.ty.is_repeated() {
				false => Ok(u32::from(DecoderLit::UInt32(buf)).into()),
				true => Ok(Vec::<u32>::from(DecoderLit::UInt32Vec(buf)).into()),
			},
			TypeRef::UINT64 => match self.ty.is_repeated() {
				false => Ok(u64::from(DecoderLit::UInt64(buf)).into()),
				true => Ok(Vec::<u64>::from(DecoderLit::UInt64Vec(buf)).into()),
			},
			TypeRef::SINT32 => match self.ty.is_repeated() {
				false => Ok(i32::from(DecoderLit::SInt32(buf)).into()),
				true => Ok(Vec::<i32>::from(DecoderLit::SInt32Vec(buf)).into()),
			},
			TypeRef::SINT64 => match self.ty.is_repeated() {
				false => Ok(i64::from(DecoderLit::SInt64(buf)).into()),
				true => Ok(Vec::<i64>::from(DecoderLit::SInt64Vec(buf)).into()),
			},
			TypeRef::SFIXED32 => match self.ty.is_repeated() {
				false => Ok(i32::from(DecoderLit::SFixed32(buf)).into()),
				true => Ok(Vec::<i32>::from(DecoderLit::SFixed32Vec(buf)).into()),
			},
			TypeRef::SFIXED64 => match self.ty.is_repeated() {
				false => Ok(i64::from(DecoderLit::SFixed64(buf)).into()),
				true => Ok(Vec::<i64>::from(DecoderLit::SFixed64Vec(buf)).into()),
			},
			TypeRef::BOOL => match self.ty.is_repeated() {
				false => Ok(bool::from(DecoderLit::Bool(buf)).into()),
				true => Ok(Vec::<bool>::from(DecoderLit::BoolVec(buf)).into()),
			},
			TypeRef::FLOAT => match self.ty.is_repeated() {
				false => Ok(f32::from(DecoderLit::Float(buf)).into()),
				true => Ok(Vec::<f32>::from(DecoderLit::FloatVec(buf)).into()),
			},
			TypeRef::DOUBLE => match self.ty.is_repeated() {
				false => Ok(f64::from(DecoderLit::Double(buf)).into()),
				true => Ok(Vec::<f64>::from(DecoderLit::DoubleVec(buf)).into()),
			},
			TypeRef::STRING => match self.ty.is_repeated() {
				false => Ok(String::from(DecoderLit::Bytes(buf)).into()),
				true => Ok(String::from(DecoderLit::Bytes(buf)).into()),
			},
			TypeRef::BYTES => match self.ty.is_repeated() {
				false => Ok(Vec::<u8>::from(DecoderLit::Bytes(buf)).into()),
				true => Ok(Vec::<u8>::from(DecoderLit::Bytes(buf)).into()),
			},
			name => Err(Error::new(format!("Unsupported type or wire type combination: {}", name))),
		}
	}
}
