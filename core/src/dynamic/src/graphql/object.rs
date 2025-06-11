use super::{
	Arguments, DeprecationStatus, Directive, ExecutionResult, Executor, Field, FieldError,
	GraphQLType, GraphQLValue, GraphQLValueAsync, JuniperValue, MetaType, Registry, SelectionSet,
	TYPE_REGISTRY, to_object_accessor, type_name,
};
use crate::SeaResult;
use crate::{BoxFieldFutureJson, ContextBase, FieldValue, ObjectAccessor, SeaographyError, Value};
use futures::{FutureExt, future::BoxFuture};
use juniper::FromInputValue;
use juniper::IntoFieldError;
use std::collections::{BTreeMap, BTreeSet};

/// A GraphQL object type
///
/// # Examples
///
/// ```
/// use async_graphql::{dynamic::*, value, Value};
///
/// let query = Object::new("Query").field(Field::new("value", TypeRef::named_nn(TypeRef::STRING), |ctx| {
///     FieldFuture::new(async move { Ok(Some(Value::from("abc"))) })
/// }));
///
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
///
/// let schema = Schema::build(query.type_name(), None, None)
///     .register(query)
///     .finish()?;
///
/// assert_eq!(
///    schema
///        .execute("{ value }")
///        .await
///        .into_result()
///        .unwrap()
///        .data,
///    value!({ "value": "abc" })
/// );
///
/// # Ok::<_, SchemaError>(())
/// # }).unwrap();
/// ```
#[derive(Debug)]
pub struct Object {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: BTreeMap<String, Field>,
	pub(crate) implements: BTreeSet<String>,
	pub(crate) inaccessible: bool,
	pub(crate) directives: Vec<Directive>,
}

impl Object {
	/// Create a GraphQL object type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			name: name.clone(),
			description: None,
			fields: BTreeMap::from_iter(vec![type_name(name)]),
			implements: Default::default(),
			inaccessible: false,
			directives: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_inaccessible!();
	impl_directive!();

	/// Add an field to the object
	#[inline]
	pub fn field(mut self, field: Field) -> Self {
		assert!(!self.fields.contains_key(&field.name), "Field `{}` already exists", field.name);
		self.fields.insert(field.name.clone(), field);
		self
	}

	/// Add an implement to the object
	#[inline]
	pub fn implement(mut self, interface: impl Into<String>) -> Self {
		let interface = interface.into();
		assert!(!self.implements.contains(&interface), "Implement `{}` already exists", interface);
		self.implements.insert(interface);
		self
	}

	/// Get an field of the object
	#[inline]
	pub fn get_field(&self, name: &str) -> Option<&Field> {
		self.fields.get(name)
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	pub(crate) fn check(&self, type_name: &str) -> SeaResult<()> {
		if self.inaccessible {
			return Err(SeaographyError::new(format!(
				"object `{}` is inaccessible",
				self.type_name()
			)));
		}
		if let Some(ty) = TYPE_REGISTRY.get(type_name) {
			if let Some(interface) = ty.as_interface() {
				if self.implements.get(interface.type_name()).is_some() {
					for interface_field in interface.fields.values() {
						match self.fields.get(&interface_field.name) {
							Some(field) => {
								if field.ty != interface_field.ty {
									return Err(SeaographyError::new(format!(
										"object `{}` field `{}` has different type `{}` than implemented type `{}`",
										self.name, field.name, field.ty, interface_field.ty
									)));
								}
								for arg in interface_field.arguments.values() {
									if let None = field.arguments.get(&arg.name) {
										return Err(SeaographyError::new(format!(
											"object `{}` field `{}` does not implement argument `{}`",
											self.name, interface_field.name, arg.name
										)));
									}
								}
							}
							None => {
								return Err(SeaographyError::new(format!(
									"object `{}` does not implement field `{}`",
									self.name, interface_field.name
								)));
							}
						}
					}
				} else {
					return Err(SeaographyError::new(format!(
						"object `{}` does not implement interface `{}`",
						self.name,
						interface.type_name()
					)));
				}
			}

			if let Some(union) = ty.as_union() {
				return union.check(self.type_name());
			}

			Ok(())
		} else {
			return Err(SeaographyError::new(format!(
				"object `{}` implements unknown CUSTOM type `{}`",
				self.name, type_name
			)));
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		selection_set: &'a SelectionSet,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Vec<BoxFieldFutureJson<'a>> {
		if self.inaccessible {
			return vec![
				async move {
					Err(SeaographyError::new(format!(
						"object `{}` is inaccessible",
						self.type_name()
					)))
				}
				.boxed(),
			];
		}
		let mut futures = Vec::new();
		for field in self.fields.values() {
			if let Some(child) = selection_set.childs.iter().find(|child| child.name == field.name)
			{
				futures.push(field.collect(ctx, child, arguments, parent_value));
			}
		}
		futures
	}
}

impl GraphQLType<Value> for Object {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		let mut output_fields = vec![];
		let mut input_fields = vec![];

		for (_, field) in &info.fields {
			if let Some(mut meta) = field.meta_output(registry) {
				if let Some(description) = &field.description {
					meta = meta.description(description);
				}
				if let DeprecationStatus::Deprecated(reason) = &field.deprecation {
					meta = meta.deprecated(match reason {
						None => None,
						Some(reason) => Some(reason),
					});
				}
				output_fields.push(meta);
			}
			if let Some(mut meta) = field.meta_input(registry) {
				if let Some(description) = &field.description {
					meta = meta.description(description);
				}
				input_fields.push(meta);
			}
		}

		// UNDER NO CIRCUMSTANCES SHOULD THIS ORDER BE CHANGED
		if !input_fields.is_empty() {
			let mut meta_type = registry.build_input_object_type::<Self>(info, &input_fields);

			if let Some(description) = &info.description {
				meta_type = meta_type.description(description);
			}

			return meta_type.into_meta();
		} else if !output_fields.is_empty() {
			let mut types = vec![];

			for name in &info.implements {
				let ty = match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.get_type(registry),
					None => panic!("Type `{}` not found", name),
				};
				types.push(ty);
			}

			let mut meta_type =
				registry.build_object_type::<Self>(info, &output_fields).interfaces(&types);

			if let Some(description) = &info.description {
				meta_type = meta_type.description(description);
			}

			return meta_type.into_meta();
		} else {
			let mut meta_type = registry.build_object_type::<Self>(info, &vec![]);

			if let Some(description) = &info.description {
				meta_type = meta_type.description(description);
			}

			meta_type.into_meta()
		}
	}
}

impl FromInputValue<Value> for Object {
	type Error = SeaographyError;

	fn from_input_value(v: &juniper::InputValue<Value>) -> Result<Self, Self::Error> {
		println!("v: {:?}", v);
		todo!()
	}
}

impl GraphQLValue<Value> for Object {
	type Context = ContextBase;
	type TypeInfo = Self;
	fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
		Some(info.type_name())
	}

	fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
		info.type_name().to_string()
	}
}

impl GraphQLValueAsync<Value> for Object
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
	fn resolve_field_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		field_name: &'a str,
		_arguments: &'a Arguments<Value>,
		executor: &'a Executor<Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		async move {
			let look_ahead = executor.look_ahead();
			let selection_set = SelectionSet::from_look_ahead(&look_ahead);

			match self.get_field(field_name) {
				Some(field) => {
					let res = field
						.collect(
							executor.context(),
							&selection_set,
							&to_object_accessor(look_ahead),
							None,
						)
						.await
						.map_err(|err| err.into_field_error())?;

					Ok(JuniperValue::scalar(res.1))
				}
				None => Err(FieldError::from(format!(
					"Field `{}` not found on type `{}`",
					field_name,
					<Self as GraphQLType<Value>>::name(info).ok_or_else(|| "Query")?,
				))),
			}
		}
		.boxed()

		// match field {
		// 	"value" => {
		// 		let fut = futures::future::ready("test");
		// 		Box::pin(fut.then(move |res: &'static str| async move {
		// 			match IntoResolvable::into_resolvable(res, executor.context())? {
		// 				Some((ctx, r)) => {
		// 					let subexec = executor.replaced_context(ctx);
		// 					subexec.resolve_with_ctx_async(&(), &r).await
		// 				}
		// 				None => Ok(JuniperValue::null()),
		// 			}
		// 		}))
		// 	}
		// 	_ => ,
		// }
	}
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;

	use juniper::http::{GraphQLRequest, GraphQLResponse};

	use crate::{
		FieldFuture, FieldValue, Value,
		graphql::{Field, JuniperValue, Object, Schema, TypeRef},
	};

	#[tokio::test]
	async fn borrow_context() {
		struct MyObjData {
			value: i32,
		}

		let my_obj = Object::new("MyObj").field(Field::output(
			"value",
			TypeRef::named(TypeRef::INT),
			|ctx| {
				FieldFuture::new(async move {
					Ok(Some(Value::from(ctx.parent_value.try_downcast_ref::<MyObjData>()?.value)))
				})
			},
		));

		let query = Object::new("Query").field(Field::output(
			"obj",
			TypeRef::named_nn(my_obj.type_name()),
			|ctx| {
				FieldFuture::new(async move {
					Ok(Some(FieldValue::borrowed_any(ctx.ctx.data_unchecked::<MyObjData>())))
				})
			},
		));

		let schema = Schema::build(query.type_name(), None, None)
			.register(query)
			.register(my_obj)
			.data(MyObjData {
				value: 123,
			})
			.finish()
			.unwrap();

		let res =
			schema.executer(GraphQLRequest::new("{ obj { value } }".to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::object(juniper::Object::from_iter(vec![(
					"obj",
					JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![(
						Value::from("value"),
						Value::from(123)
					),])))
				),])),
				vec![]
			)))
		);
	}
}
