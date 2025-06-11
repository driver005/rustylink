use super::{
	Argument, DeprecationStatus, JuniperField, Registry, SelectionSet, TYPE_REGISTRY, TypeRef,
	TypeRefToMeta,
};
use crate::{
	BoxFieldFutureJson, BoxResolverFn, ContextBase, FieldFuture, FieldValue, FieldValueInner,
	ObjectAccessor, ResolverContext, SeaResult, SeaographyError, Value,
};
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
	pub(crate) resolver_fn: Option<BoxResolverFn>,
	pub(crate) deprecation: DeprecationStatus,
	pub(crate) inaccessible: bool,
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
			ty,
			default_value: None,
			inaccessible: false,
			deprecation: DeprecationStatus::Current,
			arguments: Default::default(),
			resolver_fn: None,
		}
	}

	/// Create a GraphQL field
	pub fn output<N, T, F>(name: N, ty: T, resolver_fn: F) -> Self
	where
		N: Into<String>,
		T: Into<TypeRef>,
		F: for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync + 'static,
	{
		let ty = ty.into();
		Self {
			name: name.into(),
			description: None,
			arguments: Default::default(),
			ty,
			resolver_fn: Some(Box::new(resolver_fn)),
			deprecation: DeprecationStatus::Current,
			inaccessible: false,
			default_value: None,
		}
	}

	impl_set_description!();
	impl_set_deprecation!();
	impl_set_inaccessible!();

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

	pub(crate) async fn to_value<'a>(
		&self,
		ctx: &'a ContextBase,
		selection_set: &'a SelectionSet,
		val: &'a FieldValue<'a>,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> SeaResult<Value> {
		match &val.0 {
			FieldValueInner::Value(val) => match self.ty.type_name() {
				TypeRef::INT
				| TypeRef::FLOAT
				| TypeRef::STRING
				| TypeRef::BOOLEAN
				| TypeRef::ID => Ok(val.to_owned()),
				name => match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.to_value(val),
					None => {
						Err(SeaographyError::new(format!("Unsupported type for field `{}`", name)))
					}
				},
			},
			FieldValueInner::List(values) => {
				let mut list = Vec::new();
				for value in values.iter() {
					list.push(
						Box::pin(self.to_value(ctx, selection_set, value, arguments, parent_value))
							.await?,
					);
				}

				Ok(Value::List(list))
			}
			FieldValueInner::OwnedAny(..) => match TYPE_REGISTRY.get(self.ty.type_name()) {
				Some(inner) => {
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, selection_set, arguments, Some(val)) {
						let (name, res) = field.await?;

						data.insert(name, res);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::BorrowedAny(..) => match TYPE_REGISTRY.get(self.ty.type_name()) {
				Some(inner) => {
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, selection_set, arguments, Some(val)) {
						let (name, res) = field.await?;

						data.insert(name, res);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
			FieldValueInner::WithType {
				value,
				ty,
			} => match TYPE_REGISTRY.get(ty) {
				Some(inner) => {
					inner.check(self.ty.type_name())?;
					let mut data = BTreeMap::new();
					for field in inner.collect(ctx, selection_set, arguments, Some(value)) {
						let (name, res) = field.await?;

						data.insert(name, res);
					}

					Ok(Value::Map(data))
				}
				None => Ok(Value::Null),
			},
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		selection_set: &'a SelectionSet,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> BoxFieldFutureJson<'a> {
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
					Some(val) => {
						self.to_value(ctx, selection_set, &val, arguments, parent_value).await?
					}
					None => match &self.default_value {
						Some(value) => value.clone(),
						None => Value::Null,
					},
				};
				Ok::<Value, SeaographyError>(value)
			};
			futures::pin_mut!(resolve_fut);

			let name = match &selection_set.alias {
				Some(alias) => alias,
				None => &self.name,
			};

			Ok((Value::from(name.clone()), resolve_fut.await?))
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
