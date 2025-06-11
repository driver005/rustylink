use std::{collections::BTreeMap, ops::Add};

use super::IO;
use crate::{
	BoxResolverFn, FieldFuture, ResolverContext, TypeRefTrait,
	prelude::{GraphQLField, GraphQLTypeRef, ProtoField, ProtoTypeRef},
};

pub struct Field<T>
where
	T: TypeRefTrait,
{
	pub(crate) arguments: BTreeMap<String, Field<T>>,
	pub(crate) name: String,
	pub(crate) ty: T,
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
	pub fn input(name: impl Into<String>, ty: T) -> Self {
		Self {
			name: name.into(),
			arguments: Default::default(),
			ty,
			resolver_fn: None,
		}
	}

	/// Add an argument to the field
	#[inline]
	pub fn argument(mut self, input_value: Field<T>) -> Self {
		self.arguments.insert(input_value.name.to_owned(), input_value);
		self
	}

	pub fn output<F>(name: impl Into<String>, ty: T, resolver_fn: F) -> Self
	where
		F: for<'b> Fn(ResolverContext<'b>) -> FieldFuture<'b> + Send + Sync + 'static,
	{
		Self {
			name: name.into(),
			arguments: Default::default(),
			ty,
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
	pub(crate) fn to_field(self, tag: u32, io: &IO) -> ProtoField {
		match io {
			IO::Input => ProtoField::input(self.name, tag, self.ty),
			IO::Output => {
				if let Some(resolver_fn) = self.resolver_fn {
					self.arguments.into_iter().enumerate().fold(
						ProtoField::output(self.name, tag, self.ty, move |ctx| {
							resolver_fn(ctx.into())
						}),
						|builder, (index, (_, field))| {
							builder.argument(field.to_field(index.add(1) as u32, &IO::Input))
						},
					)
				} else {
					panic!("resolver_fn not found")
				}
			}
		}
	}
}
