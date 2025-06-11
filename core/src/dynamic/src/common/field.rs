use crate::{ContextBase, ObjectAccessor, SeaResult, SeaographyError, Value};
use bytes::BufMut;
use futures::{FutureExt, future::BoxFuture};
use std::{any::Any, borrow::Cow, ops::Deref, pin::Pin};

//// A value returned from the resolver function
pub struct FieldValue<'a>(pub(crate) FieldValueInner<'a>);

pub(crate) enum FieldValueInner<'a> {
	/// Const value
	Value(Value),
	/// Borrowed any value
	/// The first item is the [`std::any::type_name`] of the value used for
	/// debugging.
	BorrowedAny(Cow<'static, str>, &'a (dyn Any + Send + Sync)),
	/// Owned any value
	/// The first item is the [`std::any::type_name`] of the value used for
	/// debugging.
	OwnedAny(Cow<'static, str>, Box<dyn Any + Send + Sync>),
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

impl std::fmt::Debug for FieldValue<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			FieldValueInner::Value(v) => write!(f, "{}", v),
			FieldValueInner::BorrowedAny(ty, _)
			| FieldValueInner::OwnedAny(ty, _)
			| FieldValueInner::WithType {
				ty,
				..
			} => write!(f, "{}", ty),
			FieldValueInner::List(list) => match list.first() {
				Some(v) => {
					write!(f, "[{:?}, ...]", v)
				}
				None => {
					write!(f, "[()]")
				}
			},
		}
	}
}

impl From<()> for FieldValue<'_> {
	#[inline]
	fn from(_: ()) -> Self {
		Self(FieldValueInner::Value(Value::Null))
	}
}

impl From<Value> for FieldValue<'_> {
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
	pub fn owned_any<T: Any + Send + Sync>(obj: T) -> Self {
		Self(FieldValueInner::OwnedAny(std::any::type_name::<T>().into(), Box::new(obj)))
	}

	/// Create a FieldValue from unsized any value
	#[inline]
	pub fn boxed_any(obj: Box<dyn Any + Send + Sync>) -> Self {
		Self(FieldValueInner::OwnedAny("Any".into(), obj))
	}

	/// Create a FieldValue from owned any value
	#[inline]
	pub fn borrowed_any(obj: &'a (dyn Any + Send + Sync)) -> Self {
		Self(FieldValueInner::BorrowedAny("Any".into(), obj))
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
			ty: ty.into(),
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

	/// Like `as_value`, but returns `SeaResult`.
	#[inline]
	pub fn try_to_value(&self) -> SeaResult<&Value> {
		self.as_value()
			.ok_or_else(|| SeaographyError::new(format!("internal: \"{:?}\" not a Value", self)))
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

	/// Like `as_list`, but returns `SeaResult`.
	#[inline]
	pub fn try_to_list(&self) -> SeaResult<&[FieldValue]> {
		self.as_list()
			.ok_or_else(|| SeaographyError::new(format!("internal: \"{:?}\" not a List", self)))
	}

	/// If the FieldValue is a any, returns the associated
	/// vector. Returns `None` otherwise.
	#[inline]
	pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
		match &self.0 {
			FieldValueInner::BorrowedAny(_, value) => value.downcast_ref::<T>(),
			FieldValueInner::OwnedAny(_, value) => value.downcast_ref::<T>(),
			_ => None,
		}
	}

	/// Like `downcast_ref`, but returns `SeaResult`.
	#[inline]
	pub fn try_downcast_ref<T: Any>(&self) -> SeaResult<&T> {
		self.downcast_ref().ok_or_else(|| {
			SeaographyError::new(format!(
				"internal: \"{:?}\" is not of the expected type \"{}\"",
				self,
				std::any::type_name::<T>()
			))
		})
	}
}

type BoxResolveFut<'a> = BoxFuture<'a, SeaResult<Option<FieldValue<'a>>>>;

/// A context for resolver function
pub struct ResolverContext<'a> {
	// Proto context
	pub ctx: &'a ContextBase,
	/// Field arguments
	pub args: ObjectAccessor<'a>,
	/// Parent value
	pub parent_value: &'a FieldValue<'a>,
}

impl<'a> Deref for ResolverContext<'a> {
	type Target = ContextBase;

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
		Fut: Future<Output = SeaResult<Option<R>>> + Send + 'a,
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

pub(crate) type BoxFieldFutureJson<'a> =
	Pin<Box<dyn Future<Output = SeaResult<(Value, Value)>> + 'a + Send>>;

pub(crate) type BoxFieldFutureByte<'a, B: BufMut> =
	Pin<Box<dyn Future<Output = SeaResult<(usize, B)>> + 'a + Send>>;
