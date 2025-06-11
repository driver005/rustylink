use crate::{SeaResult, SeaographyError};
use fnv::FnvHashMap;
use std::{
	any::{Any, TypeId},
	fmt::{self, Debug, Formatter},
	ops::Deref,
	sync::Arc,
};

// /// A context for resolver function
// pub struct ResolverContext<'a> {
// 	pub api_type: ApiType,

// 	pub ctx: ResolverContextType<'a>,
// 	pub args: ObjectAccessors<'a>,
// 	pub parent_value: ResolverFieldValue<'a>,
// }

// impl<'a> ResolverContext<'a> {
// 	// pub fn new(
// 	// 	api_type: ApiType,
// 	// 	ctx: ResolverContextType<'a>,
// 	// 	args: ObjectAccessors<'a>,
// 	// 	parent_value: ResolverFieldValue<'a>,
// 	// ) -> Self {
// 	// 	Self {
// 	// 		api_type,
// 	// 		ctx: &ctx,
// 	// 		args,
// 	// 		parent_value: &parent_value,
// 	// 	}
// 	// }
// 	pub fn data<D: Any + Send + Sync>(&self) -> Result<&'a D> {
// 		self.ctx.data::<D>()
// 	}

// 	pub fn data_unchecked<D: Any + Send + Sync>(&self) -> &'a D {
// 		self.ctx.data_unchecked::<D>()
// 	}

// 	pub fn data_opt<D: Any + Send + Sync>(&self) -> Option<&'a D> {
// 		self.ctx.data_opt::<D>()
// 	}
// }

// impl<'a> From<GraphQLResolverContext<'a>> for ResolverContext<'a> {
// 	fn from(value: GraphQLResolverContext<'a>) -> Self {
// 		Self {
// 			api_type: ApiType::GraphQL,
// 			ctx: ResolverContextType::GraphQL(value.ctx),
// 			args: ObjectAccessors::GraphQL(value.args),
// 			parent_value: ResolverFieldValue::GraphQL(value.parent_value),
// 		}
// 	}
// }

// impl<'a> From<ProtoResolverContext<'a>> for ResolverContext<'a> {
// 	fn from(value: ProtoResolverContext<'a>) -> Self {
// 		Self {
// 			api_type: ApiType::Proto,
// 			ctx: ResolverContextType::Proto(value.ctx),
// 			args: ObjectAccessors::Proto(value.args),
// 			parent_value: ResolverFieldValue::Proto(value.parent_value),
// 		}
// 	}
// }

/// Data related functions of the context.
pub trait DataContext {
	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// If both `Schema` and `Query` have the same data type, the data in the
	/// `Query` is obtained.
	///
	/// # Errors
	///
	/// Returns a `Error` if the specified type data does not exist.
	fn data<D: Any + Send + Sync>(&self) -> SeaResult<&D>;

	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// # Panics
	///
	/// It will panic if the specified data type does not exist.
	fn data_unchecked<D: Any + Send + Sync>(&self) -> &D;

	/// Gets the global data defined in the `Context` or `Schema` or `None` if
	/// the specified type data does not exist.
	fn data_opt<D: Any + Send + Sync>(&self) -> Option<&D>;
}

/// Schema/Context data.
///
/// This is a type map, allowing you to store anything inside it.
#[derive(Default)]
pub struct Data(pub(crate) FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>);

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
#[derive(Debug)]
pub struct ContextBase {
	pub r#type: ApiType,
	pub execute_data: Option<Arc<Data>>,
}

impl juniper::Context for ContextBase {}

impl DataContext for ContextBase {
	fn data<D: Any + Send + Sync>(&self) -> SeaResult<&D> {
		ContextBase::data::<D>(self)
	}

	fn data_unchecked<D: Any + Send + Sync>(&self) -> &D {
		ContextBase::data_unchecked::<D>(self)
	}

	fn data_opt<D: Any + Send + Sync>(&self) -> Option<&D> {
		ContextBase::data_opt::<D>(self)
	}
}

impl ContextBase {
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
	pub fn data<D: Any + Send + Sync>(&self) -> SeaResult<&D> {
		self.data_opt::<D>().ok_or_else(|| {
			SeaographyError::new(format!("Data `{}` does not exist.", std::any::type_name::<D>()))
		})
	}

	/// Gets the global data defined in the `Context` or `Schema`.
	///
	/// # Panics
	///
	/// It will panic if the specified data type does not exist.
	pub fn data_unchecked<D: Any + Send + Sync>(&self) -> &D {
		self.data_opt::<D>()
			.unwrap_or_else(|| panic!("Data `{}` does not exist.", std::any::type_name::<D>()))
	}

	/// Gets the global data defined in the `Context` or `Schema` or `None` if
	/// the specified type data does not exist.
	pub fn data_opt<D: Any + Send + Sync>(&self) -> Option<&D> {
		self.execute_data
			.as_ref()
			.and_then(|execute_data| execute_data.get(&TypeId::of::<D>()))
			// .or_else(|| self.query_env.query_data.0.get(&TypeId::of::<D>()))
			// .or_else(|| self.query_env.session_data.0.get(&TypeId::of::<D>()))
			// .or_else(|| self.schema_env.data.0.get(&TypeId::of::<D>()))
			.and_then(|d| d.downcast_ref::<D>())
	}
}
