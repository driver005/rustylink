use std::collections::BTreeMap;

use super::IO;
use crate::{
	BoxResolverFn, FieldFuture, ResolverContext, TypeRefTrait,
	prelude::{GraphQLField, GraphQLTypeRef, ProtoField, ProtoTypeRef},
};

// pub trait BoxResolverFn: Send + Sync {
// 	type Error: ErrorTrait;
// 	type Value: ValueTrait<'a, FieldValue = Self::FieldValue>;
// 	type FieldValue: FieldValueTrait;
// 	type Context: ResolverContextDyn;
// 	type Future: FieldFutureTrait<
// 			'a,
// 			Error = Self::Error,
// 			ValueType = Self::Value,
// 			FieldValue = Self::FieldValue,
// 		>;

// 	fn call(&self, ctx: Self::Context) -> Self::Future;
// }

// pub(crate) type ProtoBoxResolverFn =
// 	Box<dyn for Fn(ProtoResolverContext) -> ProtoFieldFuture + Send + Sync>;

// impl BoxResolverFn for ProtoBoxResolverFn {
// 	type Error = ProtoError;
// 	type Value = ProtoValue;
// 	type FieldValue = ProtoFieldValue;
// 	type Context = ProtoResolverContext;
// 	type Future = ProtoFieldFuture;

// 	fn call(&self, ctx: Self::Context) -> Self::Future {
// 		(self)(ctx)
// 	}
// }

// pub(crate) type GraphQLBoxResolverFn =
// 	Box<dyn for Fn(GraphQLResolverContext) -> GraphQLFieldFuture + Send + Sync>;

// impl BoxResolverFn for GraphQLBoxResolverFn {
// 	type Error = GraphQLError;
// 	type Value = GraphQLValue;
// 	type FieldValue = GraphQLFieldValue;
// 	type Context = GraphQLResolverContext;
// 	type Future = GraphQLFieldFuture;

// 	fn call(&self, ctx: Self::Context) -> Self::Future {
// 		(self)(ctx)
// 	}
// }

// pub(crate) type ProtoBoxResolverFn =
// 	Box<(dyn for Fn(ProtoResolverContext) -> ProtoFieldFuture + Send + Sync)>;

// pub(crate) type GraphQLBoxResolverFn =
// 	Box<(dyn for Fn(GraphQLResolverContext) -> GraphQLFieldFuture + Send + Sync)>;

// pub(crate) type BoxResolverFn =
// 	Box<(dyn for Fn(ResolverContext) -> FieldFuture + Send + Sync)>;

pub struct Field<T>
where
	T: TypeRefTrait,
{
	pub(crate) arguments: BTreeMap<String, Field<T>>,
	pub(crate) name: String,
	pub(crate) ty: T,
	pub(crate) tag: u32,
	pub(crate) resolver_fn: Option<BoxResolverFn>,
}

impl<T> Field<T>
where
	T: TypeRefTrait,
{
	pub fn get_name(&self) -> &str {
		self.name.as_str()
	}

	/// Create a new Protobuf input field
	pub fn input(name: impl Into<String>, tag: u32, ty: T) -> Self {
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
	pub fn argument(mut self, input_value: Field<T>) -> Self {
		self.arguments.insert(input_value.name.to_owned(), input_value);
		self
	}

	pub fn output<F>(name: impl Into<String>, tag: u32, ty: T, resolver_fn: F) -> Self
	where
		F: for<'b> Fn(ResolverContext<'b>) -> FieldFuture<'b> + Send + Sync + 'static,
	{
		Self {
			name: name.into(),
			arguments: Default::default(),
			ty,
			tag,
			resolver_fn: Some(Box::new(resolver_fn)),
		}
	}
}

impl Field<GraphQLTypeRef> {
	pub(crate) fn to_field(self, io: &IO) -> GraphQLField {
		match io {
			IO::Input => GraphQLField::input(self.name, self.ty),
			IO::Output => {
				if let Some(resolver_fn) = self.resolver_fn {
					self.arguments.into_iter().fold(
						GraphQLField::output(self.name, self.ty, move |ctx| {
							resolver_fn(ctx.into())
						}),
						|builder, (_, field)| builder.argument(field.to_field(&IO::Input)),
					)
				} else {
					panic!("resolver_fn not found")
				}
			}
		}
	}
}

impl Field<ProtoTypeRef> {
	pub(crate) fn to_field(self, io: &IO) -> ProtoField {
		match io {
			IO::Input => ProtoField::input(self.name, self.tag, self.ty),
			IO::Output => {
				if let Some(resolver_fn) = self.resolver_fn {
					self.arguments.into_iter().fold(
						ProtoField::output(self.name, self.tag, self.ty, move |ctx| {
							resolver_fn(ctx.into())
						}),
						|builder, (_, field)| builder.argument(field.to_field(&IO::Input)),
					)
				} else {
					panic!("resolver_fn not found")
				}
			}
		}
	}
}
