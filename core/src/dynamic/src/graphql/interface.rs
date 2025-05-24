use super::{
	Arguments, Deprecation, Directive, ExecutionResult, Executor, Field, FieldError, GraphQLType,
	GraphQLValue, GraphQLValueAsync, IntoResolvable, JuniperField, MetaType, Registry, ScalarValue,
	TypeRef, TypeRefToMeta, get_type,
};
use crate::{ContextBase, Value};
use futures::future::BoxFuture;
use futures::{FutureExt, future};
use std::collections::{BTreeMap, BTreeSet};

/// A GraphQL interface field type
///
/// # Examples
///
/// ```
/// use async_graphql::{dynamic::*, value, Value};
///
/// let obj_a = Object::new("MyObjA")
///     .implement("MyInterface")
///     .field(Field::new("a", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(100))) })
///     }))
///     .field(Field::new("b", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(200))) })
///     }));
///
/// let obj_b = Object::new("MyObjB")
///     .implement("MyInterface")
///     .field(Field::new("a", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(300))) })
///     }))
///     .field(Field::new("c", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(400))) })
///     }));
///
/// let interface = Interface::new("MyInterface").field(InterfaceField::new("a", TypeRef::named_nn(TypeRef::INT)));
///
/// let query = Object::new("Query")
///     .field(Field::new("valueA", TypeRef::named_nn(interface.type_name()), |_| {
///         FieldFuture::new(async {
///             Ok(Some(FieldValue::with_type(FieldValue::NULL, "MyObjA")))
///         })
///     }))
///     .field(Field::new("valueB", TypeRef::named_nn(interface.type_name()), |_| {
///         FieldFuture::new(async {
///             Ok(Some(FieldValue::with_type(FieldValue::NULL, "MyObjB")))
///         })
///     }));
///
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
///
/// let schema = Schema::build(query.type_name(), None, None)
///     .register(obj_a)
///     .register(obj_b)
///     .register(interface)
///     .register(query)
///     .finish()?;
///
/// let query = r#"
///     fragment A on MyObjA { b }
///
///     fragment B on MyObjB { c }
///
///     {
///         valueA { a ...A ...B }
///         valueB { a ...A ...B }
///     }
/// "#;
///
/// assert_eq!(
///     schema.execute(query).await.into_result().unwrap().data,
///     value!({
///         "valueA": {
///             "a": 100,
///             "b": 200,
///         },
///         "valueB": {
///             "a": 300,
///             "c": 400,
///         }
///     })
/// );
///
/// # Ok::<_, SchemaError>(())
/// # }).unwrap();
/// ```
#[derive(Debug)]
pub struct InterfaceField {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) arguments: BTreeMap<String, Field>,
	pub(crate) ty: TypeRef,
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
}

impl InterfaceField {
	/// Create a GraphQL interface field type
	pub fn new(name: impl Into<String>, ty: impl Into<TypeRef>) -> Self {
		Self {
			name: name.into(),
			description: None,
			arguments: Default::default(),
			ty: ty.into(),
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

	pub fn meta<'r>(&self, registry: &mut Registry<'r, Value>) -> JuniperField<'r, Value> {
		let mut type_ref = TypeRefToMeta::new(self.type_name());
		type_ref.from_type_ref(&self.ty);
		let mut field = type_ref.to_ouput_meta(registry);

		for (_, arg) in self.arguments.iter().filter(|(_, p)| !p.inaccessible) {
			let mut type_ref = TypeRefToMeta::new(arg.type_name());
			type_ref.from_type_ref(&arg.ty);
			let arg = type_ref.to_input_meta(registry);
			field = field.argument(arg);
		}

		field
	}
}

/// A GraphQL interface type
#[derive(Debug)]
pub struct Interface {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: BTreeMap<String, InterfaceField>,
	pub(crate) implements: BTreeSet<String>,
	keys: Vec<String>,
	extends: bool,
	inaccessible: bool,
	tags: Vec<String>,
	pub(crate) directives: Vec<Directive>,
	requires_scopes: Vec<String>,
}

impl Interface {
	/// Create a GraphQL interface type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
			implements: Default::default(),
			keys: Vec::new(),
			extends: false,
			inaccessible: false,
			tags: Vec::new(),
			directives: Vec::new(),
			requires_scopes: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_extends!();
	impl_set_inaccessible!();
	impl_set_tags!();
	impl_directive!();

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	/// Add a field to the interface type
	#[inline]
	pub fn field(mut self, field: InterfaceField) -> Self {
		assert!(!self.fields.contains_key(&field.name), "Field `{}` already exists", field.name);
		self.fields.insert(field.name.clone(), field);
		self
	}

	/// Add an implement to the interface type
	#[inline]
	pub fn implement(mut self, interface: impl Into<String>) -> Self {
		let interface = interface.into();
		assert!(!self.implements.contains(&interface), "Implement `{}` already exists", interface);
		self.implements.insert(interface);
		self
	}

	/// Add an entity key
	///
	/// See also: [`Object::key`](crate::dynamic::Object::key)
	pub fn key(mut self, fields: impl Into<String>) -> Self {
		self.keys.push(fields.into());
		self
	}

	#[inline]
	#[doc(hidden)]
	pub fn is_entity(&self) -> bool {
		!self.keys.is_empty()
	}

	// #[doc(hidden)]
	// pub fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
	// 	let mut fields = BTreeMap::new();

	// 	for field in self.fields.values() {
	// 		let mut args = BTreeMap::new();

	// 		for argument in field.arguments.values() {
	// 			args.insert(argument.name.clone(), argument.to_meta_input_value());
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
	// 		MetaType::Interface {
	// 			name: self.name.clone(),
	// 			description: self.description.clone(),
	// 			fields,
	// 			possible_types: Default::default(),
	// 			extends: self.extends,
	// 			keys: if !self.keys.is_empty() {
	// 				Some(self.keys.clone())
	// 			} else {
	// 				None
	// 			},
	// 			visible: None,
	// 			inaccessible: self.inaccessible,
	// 			tags: self.tags.clone(),
	// 			rust_typename: None,
	// 			directive_invocations: to_meta_directive_invocation(self.directives.clone()),
	// 			requires_scopes: self.requires_scopes.clone(),
	// 		},
	// 	);

	// 	Ok(())
	// }
}

impl GraphQLType<Value> for Interface {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		let mut fields = vec![];

		for (_, field) in info.fields.iter().filter(|(_, p)| !p.inaccessible) {
			let field = field.meta(registry);
			fields.push(field);
		}

		let mut types = vec![];

		for name in &info.implements {
			let ty = match get_type(name) {
				Some(ty) => ty.get_type(registry),
				None => panic!("Type {} not found", name),
			};
			types.push(ty);
		}

		registry.build_interface_type::<Self>(info, &fields).interfaces(&types).into_meta()
	}
}

impl GraphQLValue<Value> for Interface {
	type Context = ContextBase;
	type TypeInfo = Self;
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

	fn resolve_into_type(
		&self,
		info: &Self::TypeInfo,
		type_name: &str,
		selection_set: Option<&[juniper::Selection<Value>]>,
		executor: &Executor<Self::Context, Value>,
	) -> ExecutionResult<Value> {
		todo!()
	}
}

impl GraphQLValueAsync<Value> for Interface
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
	fn resolve_field_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		field: &'a str,
		arguments: &'a Arguments<Value>,
		executor: &'a Executor<Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		match field {
			_ => Box::pin(async move {
				Err(FieldError::from(format!(
					"Field `{}` not found on type `{}`",
					field,
					<Self as GraphQLType<Value>>::name(info).ok_or_else(|| "Query")?,
				)))
			}),
		}
	}

	fn resolve_into_type_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		type_name: &str,
		selection_set: Option<&'a [juniper::Selection<'a, Value>]>,
		executor: &'a Executor<'a, 'a, Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		Box::pin(future::ready(GraphQLValue::resolve_into_type(
			self,
			info,
			type_name,
			selection_set,
			executor,
		)))
	}
}
