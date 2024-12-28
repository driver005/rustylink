use crate::proto::{Error, Result};
use fnv::FnvHashMap;
use std::{
	any::{Any, TypeId},
	fmt::{self, Debug, Formatter},
	ops::Deref,
};

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

/// Query context.
///
/// **This type is not stable and should not be used directly.**
#[derive(Clone)]
pub struct ContextBase<'a> {
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
	pub fn new() -> Self {
		Self {
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
