use super::{
	Arguments, ExecutionResult, Executor, FieldError, GraphQLType, GraphQLValue, GraphQLValueAsync,
	MetaType, Registry,
};
use crate::{ContextBase, Value, graphql::get_type};
use futures::{FutureExt, future::BoxFuture};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Info {
	pub(crate) name: String,
}

impl Info {
	pub fn new(name: String) -> Self {
		Self {
			name,
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}
}

pub struct QueryWrapper {}

impl QueryWrapper {
	pub fn new() -> Self {
		Self {}
	}
}

#[async_trait::async_trait]
impl GraphQLType<Value> for QueryWrapper {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		match get_type(info.type_name()) {
			Some(obj) => obj.meta(registry),
			None => {
				panic!("Query with name `{}` not found", info.type_name());
			}
		}
	}
}

impl GraphQLValue<Value> for QueryWrapper {
	type Context = ContextBase;
	type TypeInfo = Info;
	fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
		Some(info.type_name())
	}

	fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
		info.type_name().to_string()
	}

	// fn resolve_field(
	// 	&self,
	// 	info: &Self::TypeInfo,
	// 	field: &str,
	// 	arguments: &Arguments<Value>,
	// 	executor: &Executor<Self::Context, Value>,
	// ) -> ExecutionResult<Value> {
	// 	match get_type(field) {
	// 		Some(obj) => {
	// 			let res: &'static str = Self::api_version();
	// 			IntoResolvable::into_resolvable(res, executor.context()).and_then(|res| match res {
	// 				Some((ctx, r)) => executor.replaced_context(ctx).resolve_with_ctx(&(), &r),
	// 				None => Ok(JuniperValue::null()),
	// 			})
	// 		}
	// 		None => Err(FieldError::from(format!(
	// 			"Field `{}` not found on type `{}`",
	// 			field,
	// 			<Self as GraphQLType<Value>>::name(info).ok_or_else(|| "QueryWrapper")?,
	// 		))),
	// 	}
	// }
}

impl GraphQLValueAsync<Value> for QueryWrapper
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
	fn resolve_field_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		field_name: &'a str,
		arguments: &'a Arguments<Value>,
		executor: &'a Executor<Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		async move {
			match get_type(info.type_name()) {
				Some(obj) => obj.resolve(field_name, arguments, executor).await,
				None => Err(FieldError::from(format!(
					"Query with name `{}` not found",
					info.type_name()
				))),
			}
		}
		.boxed()
	}
}
