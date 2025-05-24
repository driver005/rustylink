use crate::{
	ContextBase, FieldFutureTrait, FieldValueTrait, ObjectAccessor, ResolverContextDyn, SeaResult,
	SeaographyError, Value,
};
use futures::{FutureExt, future::BoxFuture};
use std::{any::Any, borrow::Cow, ops::Deref, pin::Pin};

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

impl<'a> FieldValueTrait<'a> for FieldValue<'a> {
	type Value = Value;
	type Error = SeaographyError;

	/// A null value equivalent to `FieldValue::Value(Value::Null)`
	const NULL: Self = Self(FieldValueInner::Value(Value::Null));

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
	const NONE: Option<Self> = None;

	/// Returns a `Null::<FieldValue>` meaning the resolver no results.
	fn null() -> Self {
		Self::NULL
	}

	/// Returns a `None::<FieldValue>` meaning the resolver no results.
	fn none() -> Option<Self> {
		None
	}

	/// Create a FieldValue from [`Value`]
	#[inline]
	fn value(value: impl Into<Self::Value>) -> Self {
		Self(FieldValueInner::Value(value.into()))
	}

	/// Create a FieldValue from owned any value
	#[inline]
	fn owned_any<T: Any + Send + Sync>(obj: T) -> Self {
		Self(FieldValueInner::OwnedAny(Box::new(obj)))
	}

	/// Create a FieldValue from unsized any value
	#[inline]
	fn boxed_any(obj: Box<dyn Any + Send + Sync>) -> Self {
		Self(FieldValueInner::OwnedAny(obj))
	}

	/// Create a FieldValue from owned any value
	#[inline]
	fn borrowed_any(obj: &'a (dyn Any + Send + Sync)) -> Self {
		Self(FieldValueInner::BorrowedAny(obj))
	}

	/// Create a FieldValue from list
	#[inline]
	fn list<I, T>(values: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<Self>,
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
	fn with_type(self, ty: impl Into<Cow<'static, str>>) -> Self {
		Self(FieldValueInner::WithType {
			value: Box::new(self),
			ty: ty.into().clone(),
		})
	}

	/// If the FieldValue is a value, returns the associated
	/// Value. Returns `None` otherwise.
	#[inline]
	fn as_value(&self) -> Option<&Self::Value> {
		match &self.0 {
			FieldValueInner::Value(value) => Some(value),
			_ => None,
		}
	}

	/// Like `as_value`, but returns `Result`.
	#[inline]
	fn try_to_value(&self) -> std::result::Result<&Self::Value, Self::Error> {
		self.as_value().ok_or_else(|| SeaographyError::new("internal: not a Value"))
	}

	/// If the FieldValue is a list, returns the associated
	/// vector. Returns `None` otherwise.
	#[inline]
	fn as_list(&'a self) -> Option<&'a [Self]> {
		match &self.0 {
			FieldValueInner::List(values) => Some(values),
			_ => None,
		}
	}

	/// Like `as_list`, but returns `Result`.
	#[inline]
	fn try_to_list(&'a self) -> std::result::Result<&'a [Self], Self::Error> {
		self.as_list().ok_or_else(|| SeaographyError::new("internal: not a list"))
	}

	/// If the FieldValue is a any, returns the associated
	/// vector. Returns `None` otherwise.
	#[inline]
	fn downcast_ref<T: Any>(&self) -> Option<&T> {
		match &self.0 {
			FieldValueInner::BorrowedAny(value) => value.downcast_ref::<T>(),
			FieldValueInner::OwnedAny(value) => value.downcast_ref::<T>(),
			_ => None,
		}
	}

	/// Like `downcast_ref`, but returns `Result`.
	#[inline]
	fn try_downcast_ref<T: Any>(&self) -> std::result::Result<&T, Self::Error> {
		self.downcast_ref().ok_or_else(|| {
			SeaographyError::new(format!("internal: not type \"{}\"", std::any::type_name::<T>()))
		})
	}

	/// Convert FieldValue to Value, returns the associated
	/// value. Returns `None` otherwise.
	#[inline]
	fn to_val(&self) -> Option<Self::Value> {
		match &self.0 {
			FieldValueInner::Value(value) => Some(value.to_owned()),
			_ => None,
		}
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

impl<'a> ResolverContextDyn<'a> for ResolverContext<'a> {
	type Context = ContextBase;
	type ObjectAccessor = ObjectAccessor<'a>;
	type FieldValue = FieldValue<'a>;

	fn ctx(&'a self) -> &'a Self::Context {
		self.ctx
	}

	fn args(self) -> Self::ObjectAccessor {
		self.args
	}

	fn parent_value(&'a self) -> &'a Self::FieldValue {
		self.parent_value
	}
}

/// A future that returned from field resolver
pub enum FieldFuture<'a> {
	/// A pure value without any async operation
	Value(Option<FieldValue<'a>>),

	/// A future that returned from field resolver
	Future(BoxResolveFut<'a>),
}

impl<'a> FieldFutureTrait<'a> for FieldFuture<'a> {
	type Error = SeaographyError;
	type ValueType = Value;
	type FieldValue = FieldValue<'a>;

	/// Create a `FieldFuture` from a `Future`
	fn new<Fut, R>(future: Fut) -> Self
	where
		Fut: Future<Output = Result<Option<R>, Self::Error>> + Send + 'a,
		R: Into<Self::FieldValue> + Send,
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
	fn from_value(value: Option<Self::ValueType>) -> Self {
		FieldFuture::Value(value.map(FieldValue::from))
	}
}

// impl<'a> FieldFuture<'a> {
// 	pub fn to_graphql(self) -> GraphQLFieldFuture<'a> {
// 		match self {
// 			FieldFuture::Value(value) => match value {
// 				Some(data) => GraphQLFieldFuture::Value(Some(data.to_graphql())),
// 				None => GraphQLFieldFuture::Value(Some(GraphQLFieldValue::NULL)),
// 			},
// 			FieldFuture::Future(fut) => GraphQLFieldFuture::Future(
// 				async move {
// 					match fut.await {
// 						Ok(data) => Ok(match data {
// 							Some(data) => Some(data.to_graphql()),
// 							None => GraphQLFieldValue::NONE,
// 						}),
// 						Err(err) => Err(GraphQLError::from(err)),
// 					}
// 				}
// 				.boxed(),
// 			),
// 		}
// 	}
// }

// impl<'a> FieldFuture<'a> {
// 	/// Create a `FieldFuture` from a `Future`
// 	pub fn new<Fut, R>(future: Fut) -> Self
// 	where
// 		Fut: Future<Output = Result<Option<R>>> + Send + 'a,
// 		R: Into<FieldValue<'a>> + Send,
// 	{
// 		FieldFuture::Future(
// 			async move {
// 				let res = future.await?;
// 				Ok(res.map(Into::into))
// 			}
// 			.boxed(),
// 		)
// 	}

// 	/// Create a `FieldFuture` from a `Value`
// 	pub fn from_value(value: Option<Value>) -> Self {
// 		FieldFuture::Value(value.map(FieldValue::from))
// 	}
// }

pub(crate) type BoxResolverFn =
	Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)>;

pub(crate) type BoxFieldFuture<'a> =
	Pin<Box<dyn Future<Output = SeaResult<(Value, Value)>> + 'a + Send>>;
