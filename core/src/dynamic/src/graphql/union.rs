use super::{
	Arguments, Directive, ExecutionResult, Executor, FieldError, GraphQLType, GraphQLValue,
	GraphQLValueAsync, IntoResolvable, JuniperValue, MetaType, Registry, ScalarValue, get_type,
};
use crate::{ContextBase, Value};
use futures::future;
use std::collections::BTreeSet;

/// A GraphQL union type
///
/// # Examples
///
/// ```
/// use async_graphql::{dynamic::*, value, Value};
///
/// let obj_a = Object::new("MyObjA")
///     .field(Field::new("a", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(100))) })
///     }))
///     .field(Field::new("b", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(200))) })
///     }));
///
/// let obj_b = Object::new("MyObjB")
///     .field(Field::new("c", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(300))) })
///     }))
///     .field(Field::new("d", TypeRef::named_nn(TypeRef::INT), |_| {
///         FieldFuture::new(async { Ok(Some(Value::from(400))) })
///     }));
///
/// let union = Union::new("MyUnion")
///     .possible_type(obj_a.type_name())
///     .possible_type(obj_b.type_name());
///
/// let query = Object::new("Query")
///     .field(Field::new("valueA", TypeRef::named_nn(union.type_name()), |_| {
///         FieldFuture::new(async {
///             Ok(Some(FieldValue::with_type(FieldValue::NULL, "MyObjA")))
///         })
///     }))
///     .field(Field::new("valueB", TypeRef::named_nn(union.type_name()), |_| {
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
///     .register(union)
///     .register(query)
///     .finish()?;
///
/// let query = r#"
///     {
///         valueA { ... on MyObjA { a b } ... on MyObjB { c d } }
///         valueB { ... on MyObjA { a b } ... on MyObjB { c d } }
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
///             "c": 300,
///             "d": 400,
///         }
///     })
/// );
///
/// # Ok::<_, SchemaError>(())
/// # }).unwrap();
/// ```
#[derive(Debug)]
pub struct Union {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) possible_types: BTreeSet<String>,
	inaccessible: bool,
	tags: Vec<String>,
	pub(crate) directives: Vec<Directive>,
}

impl Union {
	/// Create a GraphQL union type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			possible_types: Default::default(),
			inaccessible: false,
			tags: Vec::new(),
			directives: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_inaccessible!();
	impl_set_tags!();
	impl_directive!();

	/// Add a possible type to the union that must be an object
	#[inline]
	pub fn possible_type(mut self, ty: impl Into<String>) -> Self {
		self.possible_types.insert(ty.into());
		self
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	// #[doc(hidden)]
	// pub fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
	// 	registry.types.insert(
	// 		self.name.clone(),
	// 		MetaType::Union {
	// 			name: self.name.clone(),
	// 			description: self.description.clone(),
	// 			possible_types: self.possible_types.clone(),
	// 			visible: None,
	// 			inaccessible: self.inaccessible,
	// 			tags: self.tags.clone(),
	// 			rust_typename: None,
	// 			directive_invocations: to_meta_directive_invocation(self.directives.clone()),
	// 		},
	// 	);
	// 	Ok(())
	// }
}

impl GraphQLType<Value> for Union {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		let mut types = vec![];

		for name in &info.possible_types {
			let ty = match get_type(name) {
				Some(ty) => ty.get_type(registry),
				None => panic!("Type {} not found", name),
			};
			types.push(ty);
		}

		registry.build_union_type::<Self>(info, &types).into_meta()
	}
}

impl GraphQLValue<Value> for Union {
	type Context = ContextBase;
	type TypeInfo = Self;
	fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
		Some(info.type_name())
	}

	fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
		info.type_name().to_string()
	}

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

impl GraphQLValueAsync<Value> for Union
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
	fn resolve_into_type_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		type_name: &str,
		selection_set: Option<&'a [juniper::Selection<'a, Value>]>,
		executor: &'a Executor<'a, 'a, Self::Context, Value>,
	) -> juniper::BoxFuture<'a, ExecutionResult<Value>> {
		Box::pin(future::ready(GraphQLValue::resolve_into_type(
			self,
			info,
			type_name,
			selection_set,
			executor,
		)))
	}
}
