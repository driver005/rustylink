use super::{Error, TypeRef, IO};
use crate::{
	interface::{Result, Value},
	prelude::{
		GraphQLField, GraphQLFieldFuture, GraphQLFieldValue, GraphQLInputValue, ProtoField,
		ProtoFieldFuture, ProtoFieldValue,
	},
	ApiType, FieldValueTrait, ResolverContext,
};
use futures_util::future::{Future, FutureExt};
use indexmap::IndexMap;
use std::any::Any;
use std::borrow::Cow;

/// A value returned from the resolver function
#[derive(Debug)]
pub struct FieldValue<'a>(pub(crate) FieldValueInner<'a>);

impl<'a> FieldValue<'a> {
	pub fn to_graphql(self) -> GraphQLFieldValue<'a> {
		self.0.to_graphql()
	}

	pub fn to_proto(self) -> ProtoFieldValue<'a> {
		self.0.to_proto()
	}
}

impl<'a> From<()> for FieldValue<'a> {
	#[inline]
	fn from(_: ()) -> Self {
		Self(FieldValueInner::Value(Value::null()))
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

impl<'a> FieldValueInner<'a> {
	fn to_graphql(self) -> GraphQLFieldValue<'a> {
		match self {
			FieldValueInner::Value(value) => GraphQLFieldValue::value(value.graphql),
			FieldValueInner::BorrowedAny(any) => GraphQLFieldValue::borrowed_any(any),
			FieldValueInner::OwnedAny(any) => GraphQLFieldValue::boxed_any(any),
			FieldValueInner::List(vec) => {
				GraphQLFieldValue::list(vec.into_iter().map(|val| val.to_graphql()))
			}
			FieldValueInner::WithType {
				value,
				ty,
			} => value.to_graphql().with_type(ty),
		}
	}

	fn to_proto(self) -> ProtoFieldValue<'a> {
		match self {
			FieldValueInner::Value(value) => ProtoFieldValue::value(value.proto),
			FieldValueInner::BorrowedAny(any) => ProtoFieldValue::borrowed_any(any),
			FieldValueInner::OwnedAny(any) => ProtoFieldValue::boxed_any(any),
			FieldValueInner::List(vec) => {
				ProtoFieldValue::list(vec.into_iter().map(|val| val.to_proto()))
			}
			FieldValueInner::WithType {
				value,
				ty,
			} => value.to_proto().with_type(ty),
		}
	}
}

impl<'a> FieldValueTrait<'a> for FieldValue<'a> {
	type Value = Value;
	type Error = Error;

	/// A null value equivalent to `FieldValue::Value(Value::Null)`
	const NULL: Self = Self(FieldValueInner::Value(Value::null()));

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
	///         let data =ctx.parent_value.try_downcast_ref::<MyObjData>()?;
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
		self.as_value().ok_or_else(|| Error::new("internal: not a Value"))
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
		self.as_list().ok_or_else(|| Error::new("internal: not a list"))
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
			Error::new(format!("internal: not type \"{}\"", std::any::type_name::<T>()))
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

// type BoxResolveFut<'a> = BoxFuture<'a, Result<Option<FieldValueInner<'a>>>>;

// /// A context for resolver function
// pub struct ResolverContext<'a> {
// 	// /// GraphQL context
// 	pub ctx: Context<'a>,
// 	/// Field arguments
// 	pub args: ObjectAccessors<'a>,
// 	/// Parent value
// 	pub parent_value: FieldValue<'a>,
// }

// impl<'a> ResolverContext<'a> {
// 	pub fn new(ctx: Context<'a>, args: ObjectAccessors<'a>, parent_value: FieldValue<'a>) -> Self {
// 		Self {
// 			ctx,
// 			args,
// 			parent_value,
// 		}
// 	}
// }

// impl<'a> ResolverContextDyn<'a> for ResolverContext<'a> {
// type Context;
// type ObjectAccessor = ObjectAccessors<'a>;
// type FieldValue = FieldValue<'a>;

// fn args(self) -> Self::ObjectAccessor {
// 	match self {
// 		ResolverContext::Graphql(resolver_context) => {
// 			ObjectAccessors::GraphQL(resolver_context.args)
// 		}
// 		ResolverContext::Proto(resolver_context) => {
// 			ObjectAccessors::Proto(resolver_context.args)
// 		}
// 	}
// }

// fn ctx(self) -> &'a Self::Context {
// 	todo!()
// }

// fn parent_value(self) -> &'a Self::FieldValue {
// 	match self {
// 		ResolverContext::Graphql(resolver_context) => {
// 			FieldValue::GraphQL(resolver_context.parent_value)
// 		}
// 		ResolverContext::Proto(resolver_context) => {
// 			ObjectAccessors::Proto(resolver_context.args)
// 		}
// 	}
// }

// pub fn to_graphql(self) -> GraphQLResolverContext<'a> {
// 	match self {
// 		ResolverContext::Graphql(resolver_context) => resolver_context,
// 		ResolverContext::Proto(_) => {
// 			panic!("resolver_context is of type proto not graphql")
// 		}
// 	}
// }

// pub fn to_proto(self) -> ProtoResolverContext<'a> {
// 	match self {
// 		ResolverContext::Graphql(_) => {
// 			panic!("resolver_context is of type graphql not proto")
// 		}
// 		ResolverContext::Proto(resolver_context) => resolver_context,
// 	}
// }
// }

/// A future that returned from field resolver
pub enum FieldFuture<'a> {
	GraphQL(GraphQLFieldFuture<'a>),
	Proto(ProtoFieldFuture<'a>),
}

impl<'a> FieldFuture<'a> {
	/// Create a `FieldFuture` from a `Future`
	pub fn new<Fut, R>(api_type: ApiType, future: Fut) -> Self
	where
		Fut: Future<Output = Result<Option<R>>> + Send + 'a,
		R: Into<FieldValue<'a>> + Send + 'a,
	{
		match api_type {
			ApiType::GraphQL => Self::GraphQL(GraphQLFieldFuture::Future(
				async move {
					let res = future.await.map_err(|err| err.to_graphql())?;
					Ok(res.map(|r| r.into().to_graphql()))
				}
				.boxed(),
			)),
			ApiType::Proto => Self::Proto(ProtoFieldFuture::Future(
				async move {
					let res = future.await.map_err(|err| err.to_proto())?;
					Ok(res.map(|r| r.into().to_proto()))
				}
				.boxed(),
			)),
		}
	}

	// /// Create a `FieldFuture` from a `Value`
	// pub fn from_value(value: Option<Value>) -> Self {
	// 	value.map(|val| {
	// 		return Self {
	// 			graphql: GraphQLFieldFuture::Value(Some(GraphQLFieldValue::from(val.graphql))),
	// 			proto: ProtoFieldFuture::Value(Some(ProtoFieldValue::from(val.proto))),
	// 		};
	// 	});

	// 	Self {
	// 		graphql: GraphQLFieldFuture::Value(None),
	// 		proto: ProtoFieldFuture::Value(None),
	// 	}
	// }

	pub fn to_graphql(self) -> GraphQLFieldFuture<'a> {
		match self {
			FieldFuture::GraphQL(field_future) => field_future,
			FieldFuture::Proto(_) => panic!("FieldFuture is not of type GraphQL"),
		}
	}

	pub fn to_proto(self) -> ProtoFieldFuture<'a> {
		match self {
			FieldFuture::GraphQL(_) => panic!("FieldFuture is not of type Proto"),
			FieldFuture::Proto(field_future) => field_future,
		}
	}
}

pub(crate) type BoxResolverFn =
	Box<(dyn for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync)>;

pub struct Field {
	pub(crate) arguments: IndexMap<String, Field>,
	pub(crate) name: String,
	pub(crate) ty: TypeRef,
	pub(crate) tag: u32,
	pub(crate) resolver_fn: Option<BoxResolverFn>,
}

impl Field {
	pub fn output<N, F>(name: N, tag: u32, ty: TypeRef, resolver_fn: F) -> Self
	where
		N: Into<String>,
		F: for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync + 'static,
	{
		Self {
			name: name.into(),
			arguments: Default::default(),
			ty,
			tag,
			resolver_fn: Some(Box::new(resolver_fn)),
		}
	}

	/// Create a new Protobuf input field
	pub fn input<N>(name: N, tag: u32, ty: TypeRef) -> Self
	where
		N: Into<String>,
	{
		Self {
			name: name.into(),
			arguments: Default::default(),
			ty,
			tag,
			resolver_fn: None,
		}
	}

	/// Add an argument to the field
	#[inline]
	pub fn argument(mut self, input_value: Field) -> Self {
		self.arguments.insert(input_value.name.clone(), input_value);
		self
	}

	pub(crate) fn to_graphql_input(self) -> GraphQLInputValue {
		GraphQLInputValue::new(self.name, self.ty.to_graphql())
	}

	pub(crate) fn to_graphql_output(self) -> GraphQLField {
		if let Some(resolver_fn) = self.resolver_fn {
			GraphQLField::new(self.name, self.ty.to_graphql(), move |ctx| {
				resolver_fn(ctx.into()).to_graphql()
			})
		} else {
			panic!("resolver_fn not found")
		}
	}

	pub(crate) fn to_proto(self, io: IO) -> ProtoField {
		match io {
			IO::Input => ProtoField::input(self.name, self.tag, self.ty.to_proto()),
			IO::Output => {
				if let Some(resolver_fn) = self.resolver_fn {
					ProtoField::output(self.name, self.tag, self.ty.to_proto(), move |ctx| {
						resolver_fn(ctx.into()).to_proto()
					})
				} else {
					panic!("resolver_fn not found")
				}
			}
		}
	}
}
