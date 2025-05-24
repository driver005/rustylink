use super::{
	Arguments, Directive, ExecutionResult, Executor, Field, FieldError, GraphQLType, GraphQLValue,
	GraphQLValueAsync, IntoResolvable, JuniperValue, MetaType, Registry, ScalarValue,
};
use crate::{BoxFieldFuture, ContextBase, FieldValue, ObjectAccessor, SeaographyError, Value};
use futures::{FutureExt, future::BoxFuture};
use juniper::FromInputValue;
use juniper::IntoFieldError;
use std::{
	borrow::Cow,
	collections::{BTreeMap, BTreeSet},
};

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
	pub(crate) oneof: bool,
	pub(crate) directives: Vec<Directive>,
	keys: Vec<String>,
	extends: bool,
	shareable: bool,
	resolvable: bool,
	inaccessible: bool,
	interface_object: bool,
	tags: Vec<String>,
	requires_scopes: Vec<String>,
}

impl Object {
	/// Create a GraphQL object type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
			implements: Default::default(),
			oneof: false,
			keys: Vec::new(),
			extends: false,
			shareable: false,
			resolvable: true,
			inaccessible: false,
			interface_object: false,
			tags: Vec::new(),
			directives: Vec::new(),
			requires_scopes: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_extends!();
	impl_set_shareable!();
	impl_set_inaccessible!();
	impl_set_interface_object!();
	impl_set_tags!();
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

	/// Add an entity key
	///
	/// # Examples
	///
	/// ```
	/// use async_graphql::{Value, dynamic::*};
	///
	/// let obj = Object::new("MyObj")
	///     .field(Field::new("a", TypeRef::named(TypeRef::INT), |_| {
	///         FieldFuture::new(async move { Ok(Some(Value::from(10))) })
	///     }))
	///     .field(Field::new("b", TypeRef::named(TypeRef::INT), |_| {
	///         FieldFuture::new(async move { Ok(Some(Value::from(20))) })
	///     }))
	///     .field(Field::new("c", TypeRef::named(TypeRef::INT), |_| {
	///         FieldFuture::new(async move { Ok(Some(Value::from(30))) })
	///     }))
	///     .key("a b")
	///     .key("c");
	/// ```
	pub fn key(mut self, fields: impl Into<String>) -> Self {
		self.keys.push(fields.into());
		self
	}

	/// Make the entity unresolvable by the current subgraph
	///
	/// Most commonly used to reference an entity without contributing fields.
	///
	/// # Examples
	///
	/// ```
	/// use async_graphql::{Value, dynamic::*};
	///
	/// let obj = Object::new("MyObj")
	///     .field(Field::new("a", TypeRef::named(TypeRef::INT), |_| {
	///         FieldFuture::new(async move { Ok(Some(Value::from(10))) })
	///     }))
	///     .unresolvable("a");
	/// ```
	///
	/// This references the `MyObj` entity with the key `a` that cannot be
	/// resolved by the current subgraph.
	pub fn unresolvable(mut self, fields: impl Into<String>) -> Self {
		self.resolvable = false;
		self.keys.push(fields.into());
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

	#[inline]
	pub(crate) fn is_entity(&self) -> bool {
		!self.keys.is_empty()
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Vec<BoxFieldFuture<'a>> {
		self.fields.iter().map(|(_, field)| field.collect(ctx, arguments, parent_value)).collect()
	}

	// pub fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
	// 	let mut fields = IndexMap::new();

	// 	for field in self.fields.values() {
	// 		let mut args = IndexMap::new();

	// 		for argument in field.arguments.values() {
	// 			args.insert(argument.type_name().to_string(), argument.to_meta_input_value());
	// 		}

	// 		fields.insert(
	// 			field.name.clone(),
	// 			MetaField {
	// 				name: field.name.clone(),
	// 				description: field.description.clone(),
	// 				args,
	// 				ty: field.ty.to_string(),
	// 				deprecation: field.deprecation.clone(),
	// 				cache_control: Default::default(),
	// 				external: field.external,
	// 				requires: field.requires.clone(),
	// 				provides: field.provides.clone(),
	// 				visible: None,
	// 				shareable: field.shareable,
	// 				inaccessible: field.inaccessible,
	// 				tags: field.tags.clone(),
	// 				override_from: field.override_from.clone(),
	// 				compute_complexity: None,
	// 				directive_invocations: to_meta_directive_invocation(field.directives.clone()),
	// 				requires_scopes: field.requires_scopes.clone(),
	// 			},
	// 		);
	// 	}

	// 	registry.types.insert(
	// 		self.name.clone(),
	// 		MetaType::Object {
	// 			name: self.name.clone(),
	// 			description: self.description.clone(),
	// 			fields,
	// 			cache_control: Default::default(),
	// 			extends: self.extends,
	// 			shareable: self.shareable,
	// 			resolvable: self.resolvable,
	// 			keys: if !self.keys.is_empty() {
	// 				Some(self.keys.clone())
	// 			} else {
	// 				None
	// 			},
	// 			visible: None,
	// 			inaccessible: self.inaccessible,
	// 			interface_object: self.interface_object,
	// 			tags: self.tags.clone(),
	// 			is_subscription: false,
	// 			rust_typename: None,
	// 			directive_invocations: to_meta_directive_invocation(self.directives.clone()),
	// 			requires_scopes: self.requires_scopes.clone(),
	// 		},
	// 	);

	// 	for interface in &self.implements {
	// 		registry.add_implements(self.type_name(), interface);
	// 	}

	// 	Ok(())
	// }
}

impl GraphQLType<Value> for Object {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		println!("Object::meta -- info.name: {:#?}", info.type_name());
		let mut output_fields = vec![];
		let mut input_fields = vec![];

		for (_, field) in &info.fields {
			if let Some(meta) = field.meta_output(registry) {
				output_fields.push(meta);
			}
			if let Some(meta) = field.meta_input(registry) {
				input_fields.push(meta);
			}
		}

		if !output_fields.is_empty() {
			return registry.build_object_type::<Self>(info, &output_fields).into_meta();
		}

		if !input_fields.is_empty() {
			return registry.build_input_object_type::<Self>(info, &input_fields).into_meta();
		}

		registry.build_object_type::<Self>(info, &vec![]).into_meta()
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
		self.type_name().to_string()
	}

	// fn resolve_field(
	// 	&self,
	// 	info: &Self::TypeInfo,
	// 	field: &str,
	// 	arguments: &Arguments<Value>,
	// 	executor: &Executor<Self::Context, Value>,
	// ) -> ExecutionResult<Value> {
	// 	match field {
	// 		"apiVersion" => {
	// 			let res: &'static str = Self::api_version();
	// 			IntoResolvable::into_resolvable(res, executor.context()).and_then(|res| match res {
	// 				Some((ctx, r)) => executor.replaced_context(ctx).resolve_with_ctx(&(), &r),
	// 				None => Ok(JuniperValue::null()),
	// 			})
	// 		}
	// 		_ => Err(FieldError::from(format!(
	// 			"Field `{}` not found on type `{}`",
	// 			field,
	// 			<Self as GraphQLType<Value>>::name(info).ok_or_else(|| "Query")?,
	// 		))),
	// 	}
	// }
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
		arguments: &'a Arguments<Value>,
		executor: &'a Executor<Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		async move {
			match self.get_field(field_name) {
				Some(field) => {
					let res = field
						.collect(
							executor.context(),
							&ObjectAccessor(Cow::Owned(BTreeMap::new())),
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
