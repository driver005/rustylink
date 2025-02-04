use crate::{
	interface::{Error, ObjectAccessors, Result},
	prelude::{
		GraphQLContext, GraphQLFieldValue, GraphQLResolverContext, ProtoFieldValue,
		ProtoResolverContext,
	},
	traits::FieldValueTrait,
};
use fnv::FnvHashMap;
use std::{
	any::{Any, TypeId},
	fmt::{self, Debug, Formatter},
	ops::Deref,
};

pub enum ResolverContextType<'a> {
	GraphQL(&'a GraphQLContext<'a>),
	Proto(&'a Context<'a>),
}

impl<'a> DataContext<'a> for ResolverContextType<'a> {
	fn data<D: Any + Send + Sync>(&self) -> Result<&'a D> {
		match self {
			ResolverContextType::GraphQL(context) => {
				context.data::<D>().map_err(|err| Error::new(err.message))
			}
			ResolverContextType::Proto(context) => context.data::<D>(),
		}
	}

	fn data_unchecked<D: Any + Send + Sync>(&self) -> &'a D {
		match self {
			ResolverContextType::GraphQL(context) => context.data_unchecked::<D>(),
			ResolverContextType::Proto(context) => context.data_unchecked::<D>(),
		}
	}

	fn data_opt<D: Any + Send + Sync>(&self) -> Option<&'a D> {
		match self {
			ResolverContextType::GraphQL(context) => context.data_opt::<D>(),
			ResolverContextType::Proto(context) => context.data_opt::<D>(),
		}
	}
}

pub enum ResolverFieldValue<'a> {
	GraphQL(&'a GraphQLFieldValue<'a>),
	Proto(&'a ProtoFieldValue<'a>),
}

impl<'a> ResolverFieldValue<'a> {
	//TODO: Implement
	// /// If the FieldValue is a value, returns the associated
	// /// Value. Returns `None` otherwise.
	// #[inline]
	// pub fn as_value(&self) -> Option<&Value> {
	// 	match self {
	// 		ResolverFieldValue::GraphQL(field_value) => field_value.as_value(),
	// 		ResolverFieldValue::Proto(field_value) => field_value.as_value(),
	// 	}
	// }

	// /// Like `as_value`, but returns `Result`.
	// #[inline]
	// pub fn try_to_value(&self) -> Result<&Value> {
	// 	match self {
	// 		ResolverFieldValue::GraphQL(field_value) => field_value.try_to_value(),
	// 		ResolverFieldValue::Proto(field_value) => field_value.try_to_value(),
	// 	}
	// }

	// /// If the FieldValue is a list, returns the associated
	// /// vector. Returns `None` otherwise.
	// #[inline]
	// pub fn as_list(&self) -> Option<&[FieldValue]> {
	// 	match self {
	// 		ResolverFieldValue::GraphQL(field_value) => field_value.as_list(),
	// 		ResolverFieldValue::Proto(field_value) => field_value.as_list(),
	// 	}
	// }

	// /// Like `as_list`, but returns `Result`.
	// #[inline]
	// pub fn try_to_list(&self) -> Result<&[FieldValue]> {
	// 	match self {
	// 		ResolverFieldValue::GraphQL(field_value) => field_value.try_to_list(),
	// 		ResolverFieldValue::Proto(field_value) => field_value.try_to_list(),
	// 	}
	// }

	/// If the FieldValue is a any, returns the associated
	/// vector. Returns `None` otherwise.
	#[inline]
	pub fn downcast_ref<T: Any>(self) -> Option<&'a T> {
		match self {
			ResolverFieldValue::GraphQL(field_value) => field_value.downcast_ref::<T>(),
			ResolverFieldValue::Proto(field_value) => field_value.downcast_ref::<T>(),
		}
	}

	/// Like `downcast_ref`, but returns `Result`.
	#[inline]
	pub fn try_downcast_ref<T: Any>(self) -> Result<&'a T> {
		match self {
			ResolverFieldValue::GraphQL(field_value) => {
				field_value.try_downcast_ref::<T>().map_err(|err| Error::new(err.message))
			}
			ResolverFieldValue::Proto(field_value) => {
				field_value.try_downcast_ref::<T>().map_err(|err| Error::new(err.message))
			}
		}
	}
}

/// A context for resolver function
pub struct ResolverContext<'a> {
	pub api_type: ApiType,

	pub ctx: ResolverContextType<'a>,
	pub args: ObjectAccessors<'a>,
	pub parent_value: ResolverFieldValue<'a>,
}

impl<'a> ResolverContext<'a> {
	// pub fn new(
	// 	api_type: ApiType,
	// 	ctx: ResolverContextType<'a>,
	// 	args: ObjectAccessors<'a>,
	// 	parent_value: ResolverFieldValue<'a>,
	// ) -> Self {
	// 	Self {
	// 		api_type,
	// 		ctx: &ctx,
	// 		args,
	// 		parent_value: &parent_value,
	// 	}
	// }
	pub fn data<D: Any + Send + Sync>(&self) -> Result<&'a D> {
		self.ctx.data::<D>()
	}

	pub fn data_unchecked<D: Any + Send + Sync>(&self) -> &'a D {
		self.ctx.data_unchecked::<D>()
	}

	pub fn data_opt<D: Any + Send + Sync>(&self) -> Option<&'a D> {
		self.ctx.data_opt::<D>()
	}
}

impl<'a> From<GraphQLResolverContext<'a>> for ResolverContext<'a> {
	fn from(value: GraphQLResolverContext<'a>) -> Self {
		Self {
			api_type: ApiType::GraphQL,
			ctx: ResolverContextType::GraphQL(value.ctx),
			args: ObjectAccessors::GraphQL(value.args),
			parent_value: ResolverFieldValue::GraphQL(value.parent_value),
		}
	}
}

impl<'a> From<ProtoResolverContext<'a>> for ResolverContext<'a> {
	fn from(value: ProtoResolverContext<'a>) -> Self {
		Self {
			api_type: ApiType::Proto,
			ctx: ResolverContextType::Proto(value.ctx),
			args: ObjectAccessors::Proto(value.args),
			parent_value: ResolverFieldValue::Proto(value.parent_value),
		}
	}
}

/// Context object for resolve field
pub type Context<'a> = ContextBase<'a>;

/// Data related functions of the context.
pub trait DataContext<'a> {
	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// If both `Schema` and `Query` have the same data type, the data in the
	/// `Query` is obtained.
	///
	/// # Errors
	///
	/// Returns a `Error` if the specified type data does not exist.
	fn data<D: Any + Send + Sync>(&self) -> Result<&'a D>;

	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// # Panics
	///
	/// It will panic if the specified data type does not exist.
	fn data_unchecked<D: Any + Send + Sync>(&self) -> &'a D;

	/// Gets the global data defined in the `Context` or `Schema` or `None` if
	/// the specified type data does not exist.
	fn data_opt<D: Any + Send + Sync>(&self) -> Option<&'a D>;
}

/// Schema/Context data.
///
/// This is a type map, allowing you to store anything inside it.
#[derive(Default)]
pub struct Data(FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>);

impl Deref for Data {
	type Target = FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Data {
	/// Insert data.
	pub fn insert<D: Any + Send + Sync>(&mut self, data: D) {
		self.0.insert(TypeId::of::<D>(), Box::new(data));
	}

	pub(crate) fn merge(&mut self, other: Data) {
		self.0.extend(other.0);
	}
}

impl Debug for Data {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("Data").finish()
	}
}

#[derive(Debug, Clone)]
pub enum ApiType {
	GraphQL,
	Proto,
}

/// Query context.
///
/// **This type is not stable and should not be used directly.**
#[derive(Debug, Clone)]
pub struct ContextBase<'a> {
	pub r#type: ApiType,
	// /// The current path node being resolved.
	// pub path_node: Option<QueryPathNode<'a>>,
	// /// If `true` means the current field is for introspection.
	// pub(crate) is_for_introspection: bool,
	// #[doc(hidden)]
	// pub item: T,
	// #[doc(hidden)]
	// pub schema_env: &'a SchemaEnv,
	// #[doc(hidden)]
	// pub query_env: &'a QueryEnv,
	// #[doc(hidden)]
	pub execute_data: Option<&'a Data>,
}

impl<'a> DataContext<'a> for ContextBase<'a> {
	fn data<D: Any + Send + Sync>(&self) -> Result<&'a D> {
		ContextBase::data::<D>(self)
	}

	fn data_unchecked<D: Any + Send + Sync>(&self) -> &'a D {
		ContextBase::data_unchecked::<D>(self)
	}

	fn data_opt<D: Any + Send + Sync>(&self) -> Option<&'a D> {
		ContextBase::data_opt::<D>(self)
	}
}

impl<'a> ContextBase<'a> {
	pub fn new(r#type: ApiType) -> Self {
		Self {
			r#type,
			execute_data: None,
		}
	}
	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// If both `Schema` and `Query` have the same data type, the data in the
	/// `Query` is obtained.
	///
	/// # Errors
	///
	/// Returns a `Error` if the specified type data does not exist.
	pub fn data<D: Any + Send + Sync>(&self) -> Result<&'a D> {
		self.data_opt::<D>().ok_or_else(|| {
			Error::new(format!("Data `{}` does not exist.", std::any::type_name::<D>()))
		})
	}

	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// # Panics
	///
	/// It will panic if the specified data type does not exist.
	pub fn data_unchecked<D: Any + Send + Sync>(&self) -> &'a D {
		self.data_opt::<D>()
			.unwrap_or_else(|| panic!("Data `{}` does not exist.", std::any::type_name::<D>()))
	}

	/// Gets the global data defined in the `Context` or `Schema` or `None` if
	/// the specified type data does not exist.
	pub fn data_opt<D: Any + Send + Sync>(&self) -> Option<&'a D> {
		self.execute_data
			.as_ref()
			.and_then(|execute_data| execute_data.get(&TypeId::of::<D>()))
			// .or_else(|| self.query_env.query_data.0.get(&TypeId::of::<D>()))
			// .or_else(|| self.query_env.session_data.0.get(&TypeId::of::<D>()))
			// .or_else(|| self.schema_env.data.0.get(&TypeId::of::<D>()))
			.and_then(|d| d.downcast_ref::<D>())
	}
}
