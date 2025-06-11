use super::{
	DeprecationStatus, Directive, Field, GraphQLType, GraphQLValue, GraphQLValueAsync,
	JuniperField, MetaType, Registry, TYPE_REGISTRY, Type, TypeRef, TypeRefToMeta,
};
use crate::{BoxFieldFutureJson, ContextBase, SeaResult, SeaographyError, Value};
use futures::FutureExt;
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
	pub(crate) deprecation: DeprecationStatus,
	pub(crate) inaccessible: bool,
	pub(crate) directives: Vec<Directive>,
}

impl InterfaceField {
	/// Create a GraphQL interface field type
	pub fn new(name: impl Into<String>, ty: impl Into<TypeRef>) -> Self {
		Self {
			name: name.into(),
			description: None,
			arguments: Default::default(),
			ty: ty.into(),
			deprecation: DeprecationStatus::Current,
			inaccessible: false,
			directives: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_deprecation!();
	impl_set_inaccessible!();
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
	pub(crate) directives: Vec<Directive>,
	pub(crate) inaccessible: bool,
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
			directives: Vec::new(),
			inaccessible: false,
		}
	}

	impl_set_description!();
	impl_set_inaccessible!();
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

	pub(crate) fn check(&self, type_name: &str) -> SeaResult<()> {
		if self.inaccessible {
			return Err(SeaographyError::new(format!(
				"interface `{}` is inaccessible",
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
										"interface `{}` field `{}` has different type `{}` than implemented type `{}`",
										self.name, field.name, field.ty, interface_field.ty
									)));
								}
								for arg in interface_field.arguments.values() {
									if let None = field.arguments.get(&arg.name) {
										return Err(SeaographyError::new(format!(
											"interface `{}` field `{}` does not implement argument `{}`",
											self.name, interface_field.name, arg.name
										)));
									}
								}
							}
							None => {
								return Err(SeaographyError::new(format!(
									"interface `{}` does not implement field `{}`",
									self.name, interface_field.name
								)));
							}
						}
					}
				} else {
					return Err(SeaographyError::new(format!(
						"interface `{}` does not implement interface `{}`",
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

	pub(crate) fn collect<'a>(&'a self) -> BoxFieldFutureJson<'a> {
		async move {
			return Err(SeaographyError::new(format!(
				"invalid FieldValue for interface `{}`, expected `FieldValue::with_type`",
				self.type_name()
			)));
		}
		.boxed()
	}
}

impl GraphQLType<Value> for Interface {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		let implements = TYPE_REGISTRY.get_filtered(|_, ty| match &**ty {
			Type::Object(obj) => obj.implements.contains(&info.name),
			Type::Interface(interface) => interface.implements.contains(&info.name),
			_ => false,
		});

		for ty in implements.values() {
			ty.get_type(registry);
		}

		let mut fields = vec![];

		for (_, field) in info.fields.iter().filter(|(_, p)| !p.inaccessible) {
			let mut interface_field = field.meta(registry);
			if let Some(description) = &field.description {
				interface_field = interface_field.description(description);
			}
			if let DeprecationStatus::Deprecated(reason) = &field.deprecation {
				interface_field = interface_field.deprecated(match reason {
					None => None,
					Some(reason) => Some(reason),
				});
			}
			fields.push(interface_field);
		}

		let mut types = vec![];

		for name in &info.implements {
			let ty = match TYPE_REGISTRY.get(name) {
				Some(ty) => ty.get_type(registry),
				None => panic!("Type {} not found", name),
			};
			types.push(ty);
		}

		let mut meta_type = registry.build_interface_type::<Self>(info, &fields).interfaces(&types);

		if let Some(description) = &info.description {
			meta_type = meta_type.description(description);
		}

		meta_type.into_meta()
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
}

impl GraphQLValueAsync<Value> for Interface
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;

	use crate::{
		FieldFuture, FieldValue, SeaographyError, Value,
		graphql::{
			Field, Interface, InterfaceField, IntoFieldError, JuniperValue, Object, Schema, TypeRef,
		},
	};
	use juniper::{
		ExecutionError,
		http::{GraphQLRequest, GraphQLResponse},
		parser::SourcePosition,
	};

	#[tokio::test]
	async fn basic_interface() {
		let obj_a = Object::new("MyObjA")
			.implement("MyInterface")
			.field(Field::output("a", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(100))) })
			}))
			.field(Field::output("b", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(200))) })
			}));

		let obj_b = Object::new("MyObjB")
			.implement("MyInterface")
			.field(Field::output("a", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(300))) })
			}))
			.field(Field::output("c", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(400))) })
			}));

		let interface = Interface::new("MyInterface")
			.field(InterfaceField::new("a", TypeRef::named(TypeRef::INT)));

		let query = Object::new("Query")
			.field(Field::output("valueA", TypeRef::named_nn(interface.type_name()), |_| {
				FieldFuture::new(async { Ok(Some(FieldValue::NULL.with_type("MyObjA"))) })
			}))
			.field(Field::output("valueB", TypeRef::named_nn(interface.type_name()), |_| {
				FieldFuture::new(async { Ok(Some(FieldValue::NULL.with_type("MyObjB"))) })
			}));

		let schema = Schema::build(query.type_name(), None, None)
			.register(obj_a)
			.register(obj_b)
			.register(interface)
			.register(query)
			.finish()
			.unwrap();

		let query = r#"
		fragment A on MyObjA {
		    b
		}

		fragment B on MyObjB {
		    c
		}

		{
		    valueA { __typename a ...A ...B }
		    valueB { __typename a ...A ...B }
		}
        "#;

		let res = schema.executer(GraphQLRequest::new(query.to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::object(juniper::Object::from_iter(vec![
					(
						"valueA",
						JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![
							(Value::from("__typename"), Value::from("MyObjA")),
							(Value::from("a"), Value::from(100)),
							(Value::from("b"), Value::from(200)),
						])))
					),
					(
						"valueB",
						JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![
							(Value::from("__typename"), Value::from("MyObjB")),
							(Value::from("a"), Value::from(300)),
							(Value::from("c"), Value::from(400)),
						])))
					),
				])),
				vec![]
			)))
		);
	}

	#[tokio::test]
	async fn does_not_implement() {
		let obj_a = Object::new("MyObjA")
			.field(Field::output("a", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(100))) })
			}))
			.field(Field::output("b", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(200))) })
			}));

		let interface = Interface::new("MyInterface")
			.field(InterfaceField::new("a", TypeRef::named(TypeRef::INT)));

		let query = Object::new("Query").field(Field::output(
			"valueA",
			TypeRef::named_nn(interface.type_name()),
			|_| FieldFuture::new(async { Ok(Some(FieldValue::NULL.with_type("MyObjA"))) }),
		));

		let schema = Schema::build(query.type_name(), None, None)
			.register(obj_a)
			.register(interface)
			.register(query)
			.finish()
			.unwrap();

		let query = r#"
        {
            valueA { a }
        }
        "#;

		let res = schema.executer(GraphQLRequest::new(query.to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::null(),
				vec![ExecutionError::new(
					SourcePosition::new(23, 2, 12),
					&["valueA"],
					SeaographyError::new(
						"object `MyObjA` does not implement interface `MyInterface`"
					)
					.into_field_error()
				)]
			)))
		);
	}
	#[tokio::test]
	async fn query_type_condition() {
		struct MyObjA;
		let obj_a = Object::new("MyObjA")
			.implement("MyInterface")
			.field(Field::output("a", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(100))) })
			}))
			.field(Field::output("b", TypeRef::named(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(200))) })
			}));
		let interface = Interface::new("MyInterface")
			.field(InterfaceField::new("a", TypeRef::named(TypeRef::INT)));
		let query = Object::new("Query");
		let query =
			query.field(Field::output("valueA", TypeRef::named_nn(obj_a.type_name()), |_| {
				FieldFuture::new(async { Ok(Some(FieldValue::owned_any(MyObjA))) })
			}));
		let schema = Schema::build(query.type_name(), None, None)
			.register(interface)
			.register(obj_a)
			.register(query)
			.finish()
			.unwrap();
		let query = r#"
	    {
	        valueA { __typename
	        b
	        ... on MyInterface { a } }
	    }
	    "#;

		let res = schema.executer(GraphQLRequest::new(query.to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::object(juniper::Object::from_iter(vec![(
					"valueA",
					JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![
						(Value::from("__typename"), Value::from("MyObjA")),
						(Value::from("b"), Value::from(200)),
						(Value::from("a"), Value::from(100)),
					])))
				)])),
				vec![]
			)))
		);
	}
}
