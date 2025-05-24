use super::{Argument, Directive, JuniperField, Registry, TypeRef, TypeRefToMeta, get_type};
use crate::{
	BoxFieldFuture, BoxResolverFn, ContextBase, FieldFuture, FieldValue, FieldValueInner,
	FieldValueTrait, ObjectAccessor, ResolverContext, SeaResult, SeaographyError, Value,
};
use async_graphql::registry::Deprecation;
use futures::FutureExt;
use std::{
	collections::BTreeMap,
	fmt::{self, Debug},
};

/// A GraphQL field
pub struct Field {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) arguments: BTreeMap<String, Field>,
	pub(crate) ty: TypeRef,
	pub(crate) ty_str: String,
	pub(crate) resolver_fn: Option<BoxResolverFn>,
	pub(crate) deprecation: Deprecation,
	pub(crate) external: bool,
	pub(crate) requires: Option<String>,
	pub(crate) provides: Option<String>,
	pub(crate) shareable: bool,
	pub(crate) inaccessible: bool,
	pub(crate) tags: Vec<String>,
	pub(crate) override_from: Option<String>,
	pub(crate) directives: Vec<Directive>,
	pub(crate) requires_scopes: Vec<String>,
	pub(crate) default_value: Option<Value>,
}

impl Debug for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Field")
			.field("name", &self.name)
			.field("description", &self.description)
			.field("arguments", &self.arguments)
			.field("ty", &self.ty)
			.field("deprecation", &self.deprecation)
			.finish()
	}
}

impl Field {
	/// Create a GraphQL input value type
	#[inline]
	pub fn input<N, T>(name: N, ty: T) -> Self
	where
		N: Into<String>,
		T: Into<TypeRef>,
	{
		let ty = ty.into();
		Self {
			name: name.into(),
			description: None,
			ty_str: ty.to_string(),
			ty,
			default_value: None,
			inaccessible: false,
			tags: Vec::new(),
			directives: vec![],
			deprecation: Deprecation::NoDeprecated,
			arguments: Default::default(),
			resolver_fn: None,
			external: false,
			requires: None,
			provides: None,
			shareable: false,
			override_from: None,
			requires_scopes: Vec::new(),
		}
	}

	/// Create a GraphQL field
	pub fn output<N, T, F>(name: N, ty: T, resolver_fn: F) -> Self
	where
		N: Into<String>,
		T: Into<TypeRef>,
		F: for<'b> Fn(ResolverContext<'b>) -> FieldFuture<'b> + Send + Sync + 'static,
	{
		let ty = ty.into();
		Self {
			name: name.into(),
			description: None,
			arguments: Default::default(),
			ty_str: ty.to_string(),
			ty,
			resolver_fn: Some(Box::new(resolver_fn)),
			deprecation: Deprecation::NoDeprecated,
			external: false,
			requires: None,
			provides: None,
			shareable: false,
			inaccessible: false,
			tags: Vec::new(),
			override_from: None,
			directives: Vec::new(),
			requires_scopes: Vec::new(),
			default_value: None,
		}
	}

	impl_set_description!();
	impl_set_deprecation!();
	impl_set_external!();
	impl_set_requires!();
	impl_set_provides!();
	impl_set_shareable!();
	impl_set_inaccessible!();
	impl_set_tags!();
	impl_set_override_from!();
	impl_directive!();

	/// Set the default value
	#[inline]
	pub fn default_value(self, value: impl Into<Value>) -> Self {
		Self {
			default_value: Some(value.into()),
			..self
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	/// Add an argument to the field
	#[inline]
	pub fn argument(mut self, input_value: Field) -> Self {
		self.arguments.insert(input_value.name.clone(), input_value);
		self
	}

	pub fn required(self, name: &str) -> bool {
		if let Some(val) = &self.requires {
			if val.contains(name) {
				return true;
			}
		}

		false
	}

	pub(crate) async fn to_value<'a>(
		&self,
		ctx: &'a ContextBase,
		val: &'a FieldValue<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> SeaResult<Value> {
		match &val.0 {
			FieldValueInner::Value(val) => Ok(val.clone()),
			FieldValueInner::List(values) => {
				let mut list = Vec::new();
				for value in values.iter() {
					list.push(Box::pin(self.to_value(ctx, value, arguments, parent_value)).await?);
				}

				Ok(Value::List(list))
			}
			FieldValueInner::OwnedAny(_) => match get_type(self.ty.type_name()) {
				Some(inner) => {
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, arguments, Some(val)) {
						let (name, value) = field.await?;

						data.insert(name, value);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::BorrowedAny(_) => match get_type(self.ty.type_name()) {
				Some(inner) => {
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, arguments, Some(val)) {
						let (name, value) = field.await?;

						data.insert(name, value);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::WithType {
				value,
				..
			} => Box::pin(self.to_value(ctx, value, arguments, parent_value)).await,
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> BoxFieldFuture<'a> {
		async move {
			let resolve_fut = async {
				let parent_val = match parent_value {
					Some(val) => val,
					None => &FieldValue::NULL,
				};
				let field_future = match &self.resolver_fn {
					Some(resolver_fn) => (resolver_fn)(ResolverContext {
						ctx,
						args: arguments.clone(),
						parent_value: parent_val,
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
				Ok::<Value, SeaographyError>(value)
			};
			futures_util::pin_mut!(resolve_fut);

			Ok((Value::from(self.name.clone()), resolve_fut.await?))
		}
		.boxed()
	}

	pub fn meta_input<'r>(
		&self,
		registry: &mut Registry<'r, Value>,
	) -> Option<Argument<'r, Value>> {
		if self.resolver_fn.is_none() {
			let mut type_ref = TypeRefToMeta::new(self.type_name());
			type_ref.from_type_ref(&self.ty);
			Some(type_ref.to_input_meta(registry))
		} else {
			None
		}
	}

	pub fn meta_output<'r>(
		&self,
		registry: &mut Registry<'r, Value>,
	) -> Option<JuniperField<'r, Value>> {
		if self.resolver_fn.is_some() {
			let mut type_ref = TypeRefToMeta::new(self.type_name());
			type_ref.from_type_ref(&self.ty);
			let mut field = type_ref.to_ouput_meta(registry);

			for (_, arg) in self.arguments.iter().filter(|(_, p)| !p.inaccessible) {
				let mut type_ref = TypeRefToMeta::new(arg.type_name());
				type_ref.from_type_ref(&arg.ty);
				let arg = type_ref.to_input_meta(registry);
				field = field.argument(arg);
			}

			Some(field)
		} else {
			None
		}
	}
}
