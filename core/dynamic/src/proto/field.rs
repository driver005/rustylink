use super::{get_type, Error, ObjectAccessor, Result, Type, TypeRef, Value};
use crate::{prelude::Name, Context};
use binary::proto::{Decoder, DecoderLit, Encoder, EncoderLit};
use futures_util::{future::BoxFuture, Future, FutureExt};
use indexmap::IndexMap;
use std::{
	any::Any,
	borrow::Cow,
	fmt::{self, Debug},
	ops::Deref,
	pin::Pin,
};

/// A value returned from the resolver function
#[derive(Debug)]
pub struct FieldValue<'a>(pub(crate) FieldValueInner<'a>);

#[derive(Debug)]
pub(crate) enum FieldValueInner<'a> {
	/// Const value
	Value(Value),
	/// Borrowed any value
	BorrowedAny(&'a (dyn Any + Send + Sync)),
	/// Owned any value
	OwnedAny(Box<dyn Any + Send + Sync>),
	/// A list
	List(Vec<FieldValue<'a>>),
	/// A typed Field value
	WithType {
		/// Field value
		value: Box<FieldValue<'a>>,
		/// Object name
		ty: Cow<'static, str>,
	},
}

impl<'a> From<()> for FieldValue<'a> {
	#[inline]
	fn from(_: ()) -> Self {
		Self(FieldValueInner::Value(Value::Null))
	}
}

impl<'a> From<Value> for FieldValue<'a> {
	#[inline]
	fn from(value: Value) -> Self {
		Self(FieldValueInner::Value(value))
	}
}

impl<'a, T: Into<FieldValue<'a>>> From<Vec<T>> for FieldValue<'a> {
	fn from(values: Vec<T>) -> Self {
		Self(FieldValueInner::List(values.into_iter().map(Into::into).collect()))
	}
}

impl<'a> FieldValue<'a> {
	/// A null value equivalent to `FieldValue::Value(Value::Null)`
	pub const NULL: FieldValue<'a> = Self(FieldValueInner::Value(Value::Null));

	/// A none value equivalent to `None::<FieldValue>`
	///
	/// It is more convenient to use when your resolver needs to return `None`.
	///
	/// # Examples
	///
	/// ```
	/// use async_graphql::dynamic::*;
	///
	/// let query =
	///     Object::new("Query").field(Field::new("value", TypeRef::named(TypeRef::INT), |ctx| {
	///         FieldFuture::new(async move { Ok(FieldValue::NONE) })
	///     }));
	/// ```
	pub const NONE: Option<FieldValue<'a>> = None;

	/// Returns a `None::<FieldValue>` meaning the resolver no results.
	pub const fn none() -> Option<FieldValue<'a>> {
		None
	}

	/// Create a FieldValue from [`Value`]
	#[inline]
	pub fn value(value: impl Into<Value>) -> Self {
		Self(FieldValueInner::Value(value.into()))
	}

	/// Create a FieldValue from owned any value
	#[inline]
	pub fn owned_any(obj: impl Any + Send + Sync) -> Self {
		Self(FieldValueInner::OwnedAny(Box::new(obj)))
	}

	/// Create a FieldValue from unsized any value
	#[inline]
	pub fn boxed_any(obj: Box<dyn Any + Send + Sync>) -> Self {
		Self(FieldValueInner::OwnedAny(obj))
	}

	/// Create a FieldValue from owned any value
	#[inline]
	pub fn borrowed_any(obj: &'a (dyn Any + Send + Sync)) -> Self {
		Self(FieldValueInner::BorrowedAny(obj))
	}

	/// Create a FieldValue from list
	#[inline]
	pub fn list<I, T>(values: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<FieldValue<'a>>,
	{
		Self(FieldValueInner::List(values.into_iter().map(Into::into).collect()))
	}

	/// Create a FieldValue and specify its type, which must be an object
	///
	/// NOTE: Fields of type `Interface` or `Union` must return
	/// `FieldValue::WithType`.
	///
	/// # Examples
	///
	/// ```
	/// use async_graphql::{dynamic::*, value, Value};
	///
	/// struct MyObjData {
	///     a: i32,
	/// }
	///
	/// let my_obj = Object::new("MyObj").field(Field::new(
	///     "a",
	///     TypeRef::named_nn(TypeRef::INT),
	///     |ctx| FieldFuture::new(async move {
	///         let data = ctx.parent_value.try_downcast_ref::<MyObjData>()?;
	///         Ok(Some(Value::from(data.a)))
	///     }),
	/// ));
	///
	/// let my_union = Union::new("MyUnion").possible_type(my_obj.type_name());
	///
	/// let query = Object::new("Query").field(Field::new(
	///     "obj",
	///     TypeRef::named_nn(my_union.type_name()),
	///     |_| FieldFuture::new(async move {
	///         Ok(Some(FieldValue::owned_any(MyObjData { a: 10 }).with_type("MyObj")))
	///     }),
	/// ));
	///
	/// let schema = Schema::build("Query", None, None)
	///     .register(my_obj)
	///     .register(my_union)
	///     .register(query)
	///     .finish()
	///     .unwrap();
	///
	/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
	/// assert_eq!(
	///    schema
	///        .execute("{ obj { ... on MyObj { a } } }")
	///        .await
	///        .into_result()
	///        .unwrap()
	///        .data,
	///    value!({ "obj": { "a": 10 } })
	/// );
	/// # });
	/// ```
	pub fn with_type(self, ty: impl Into<Cow<'static, str>>) -> Self {
		Self(FieldValueInner::WithType {
			value: Box::new(self),
			ty: ty.into().clone(),
		})
	}

	/// If the FieldValue is a value, returns the associated
	/// Value. Returns `None` otherwise.
	#[inline]
	pub fn as_value(&self) -> Option<&Value> {
		match &self.0 {
			FieldValueInner::Value(value) => Some(value),
			_ => None,
		}
	}

	/// Like `as_value`, but returns `Result`.
	#[inline]
	pub fn try_to_value(&self) -> Result<&Value> {
		self.as_value().ok_or_else(|| Error::new("internal: not a Value"))
	}

	/// If the FieldValue is a list, returns the associated
	/// vector. Returns `None` otherwise.
	#[inline]
	pub fn as_list(&self) -> Option<&[FieldValue]> {
		match &self.0 {
			FieldValueInner::List(values) => Some(values),
			_ => None,
		}
	}

	/// Like `as_list`, but returns `Result`.
	#[inline]
	pub fn try_to_list(&self) -> Result<&[FieldValue]> {
		self.as_list().ok_or_else(|| Error::new("internal: not a list"))
	}

	/// If the FieldValue is a any, returns the associated
	/// vector. Returns `None` otherwise.
	#[inline]
	pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
		match &self.0 {
			FieldValueInner::BorrowedAny(value) => value.downcast_ref::<T>(),
			FieldValueInner::OwnedAny(value) => value.downcast_ref::<T>(),
			_ => None,
		}
	}

	/// Like `downcast_ref`, but returns `Result`.
	#[inline]
	pub fn try_downcast_ref<T: Any>(&self) -> Result<&T> {
		self.downcast_ref().ok_or_else(|| {
			Error::new(format!("internal: not type \"{}\"", std::any::type_name::<T>()))
		})
	}

	/// Convert FieldValue to Value, returns the associated
	/// value. Returns `None` otherwise.
	#[inline]
	pub fn to_val(&self) -> Option<Value> {
		match &self.0 {
			FieldValueInner::Value(value) => Some(value.to_owned()),
			_ => None,
		}
	}
}

type BoxResolveFut<'a> = BoxFuture<'a, Result<Option<FieldValue<'a>>>>;

/// A context for resolver function
pub struct ResolverContext<'a> {
	pub type_name: &'a str,
	/// GraphQL context
	pub ctx: &'a Context<'a>,
	/// Field arguments
	pub args: ObjectAccessor<'a>,
	// /// Parent value
	pub parent_value: &'a FieldValue<'a>,
}

impl<'a> Deref for ResolverContext<'a> {
	type Target = Context<'a>;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

/// A future that returned from field resolver
pub enum FieldFuture<'a> {
	/// A pure value without any async operation
	Value(Option<FieldValue<'a>>),

	/// A future that returned from field resolver
	Future(BoxResolveFut<'a>),
}

impl<'a> FieldFuture<'a> {
	/// Create a `FieldFuture` from a `Future`
	pub fn new<Fut, R>(future: Fut) -> Self
	where
		Fut: Future<Output = Result<Option<R>>> + Send + 'a,
		R: Into<FieldValue<'a>> + Send,
	{
		FieldFuture::Future(
			async move {
				let res = future.await?;
				Ok(res.map(Into::into))
			}
			.boxed(),
		)
	}

	/// Create a `FieldFuture` from a `Value`
	pub fn from_value(value: Option<Value>) -> Self {
		FieldFuture::Value(value.map(FieldValue::from))
	}
}

pub(crate) type BoxResolverFn =
	Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)>;

pub(crate) type BoxFieldFuture<'a> =
	Pin<Box<dyn Future<Output = Result<(Name, Value)>> + 'a + Send>>;

/// A Protobuf field
pub struct Field {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) arguments: IndexMap<String, Field>,
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
		ctx: &'a Context<'a>,
		val: &'a FieldValue<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Result<Value> {
		match &val.0 {
			FieldValueInner::Value(val) => Ok(val.clone()),
			FieldValueInner::List(values) => {
				let mut list = Vec::new();
				for value in values.iter() {
					list.push(
						Box::pin(self.to_value(ctx, &*value, arguments, parent_value)).await?,
					);
				}

				Ok(Value::List(list))
			}
			FieldValueInner::OwnedAny(_) => match get_type(self.ty.type_name()) {
				Some(inner) => {
					let mut message = IndexMap::new();
					for field in inner.collect(ctx, arguments, Some(val)) {
						let (name, value) = field.await?;

						message.insert(name, value);
					}

					Ok(Value::Message(message))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::BorrowedAny(_) => match get_type(self.ty.type_name()) {
				Some(inner) => {
					let mut message = IndexMap::new();
					for field in inner.collect(ctx, arguments, Some(val)) {
						let (name, value) = field.await?;

						message.insert(name, value);
					}

					Ok(Value::Message(message))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::WithType {
				value,
				..
			} => Box::pin(self.to_value(ctx, &*value, arguments, parent_value)).await,
		}
	}

	pub(crate) fn decode(
		&self,
		decoder: &mut Decoder,
		buf: Vec<u8>,
		tag: u32,
		arguments: &mut IndexMap<Name, Value>,
	) -> Result<()> {
		match get_type(self.ty.type_name()) {
			Some(inner) => match &*inner {
				Type::Scalar(scalar) => {
					let value = self.bytes(buf, tag)?;
					arguments.insert(Name::new(scalar.type_name()), value);
					println!("name: {:?} scalar: {:?}", inner.name(), scalar);
				}
				Type::Message(message) => {
					let value = message.decode(decoder, buf)?;
					arguments.insert(Name::new(message.type_name()), value.into());
				}
				Type::Enum(e) => {
					let value = e.bytes(buf)?;
					arguments.insert(Name::new(e.type_name()), value);
				}
				Type::Service(service) => {
					let value = service.decode(decoder, buf)?;
					arguments.insert(Name::new(service.type_name()), value.into());
				}
			},
			None => return Err(Error::new(format!("Unknown type {}", self.ty.type_name()))),
		}

		Ok(())
	}

	pub(crate) async fn execute<'a>(
		&self,
		ctx: &'a Context<'a>,
		arguments: &'a ObjectAccessor<'a>,
	) -> Result<Vec<u8>> {
		let field = self.collect(ctx, arguments, None).await?;

		println!("res: {:#?}", field.0);
		println!("val: {:#?}", field.1);

		let encoder = Encoder::default();
		let mut output = Vec::new();
		self.encode(&encoder, &mut output, &field.1)?;
		Ok(output)
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a Context<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> BoxFieldFuture<'a> {
		async move {
			let resolve_fut = async {
				let field_future = match &self.resolver_fn {
					Some(resolver_fn) => (resolver_fn)(ResolverContext {
						ctx,
						type_name: &self.ty.type_name(),
						args: arguments.clone(),
						parent_value: parent_value.unwrap_or(&FieldValue::NULL),
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

			Ok((Name::new(self.name.clone()), resolve_fut.await?))
		}
		.boxed()
	}

	pub(crate) fn encode(
		&self,
		encoder: &Encoder,
		buf: &mut Vec<u8>,
		value: &Value,
	) -> Result<usize> {
		match value {
			Value::Int32(value) => Ok(encoder.encode((&self.tag, EncoderLit::Int32(value)), buf)?),
			Value::Int64(value) => Ok(encoder.encode((&self.tag, EncoderLit::Int64(value)), buf)?),
			Value::UInt32(value) => {
				Ok(encoder.encode((&self.tag, EncoderLit::UInt32(value)), buf)?)
			}
			Value::UInt64(value) => {
				Ok(encoder.encode((&self.tag, EncoderLit::UInt64(value)), buf)?)
			}
			Value::SInt32(value) => {
				Ok(encoder.encode((&self.tag, EncoderLit::SInt32(value)), buf)?)
			}
			Value::SInt64(value) => {
				Ok(encoder.encode((&self.tag, EncoderLit::SInt64(value)), buf)?)
			}
			Value::Float(value) => Ok(encoder.encode((&self.tag, EncoderLit::Float(value)), buf)?),
			Value::Double(value) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Double(value)), buf)?)
			}
			Value::Boolean(value) => Ok(encoder.encode((&self.tag, EncoderLit::Bool(value)), buf)?),
			Value::String(value) => {
				Ok(encoder
					.encode((&self.tag, EncoderLit::Bytes(&value.clone().into_bytes())), buf)?)
			}
			Value::Binary(value) => Ok(encoder.encode((&self.tag, EncoderLit::Bytes(value)), buf)?),
			Value::Enum((_, value)) => {
				Ok(encoder.encode((&self.tag, EncoderLit::Int32(value)), buf)?)
			}
			Value::List(values) => {
				values.iter().map(|value| self.encode(encoder, buf, value)).sum()
			}
			Value::Message(values) => {
				values.iter().map(|(_, value)| self.encode(encoder, buf, value)).sum()
			}
			Value::Null => Ok(0),
		}
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
