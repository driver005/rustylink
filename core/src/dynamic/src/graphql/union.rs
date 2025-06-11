use super::{
	Directive, GraphQLType, GraphQLValue, GraphQLValueAsync, MetaType, Registry, TYPE_REGISTRY,
};
use crate::{BoxFieldFutureJson, ContextBase, SeaResult, SeaographyError, Value};
use futures::FutureExt;
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
	pub(crate) inaccessible: bool,
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
			directives: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_inaccessible!();
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

	pub(crate) fn check(&self, type_name: &str) -> SeaResult<()> {
		if self.inaccessible {
			return Err(SeaographyError::new(format!(
				"union `{}` is inaccessible",
				self.type_name()
			)));
		}
		if !self.possible_types.contains(type_name) {
			return Err(SeaographyError::new(format!(
				"union `{}` has no possible type `{}`",
				self.name, type_name,
			)));
		}
		Ok(())
	}

	pub(crate) fn collect<'a>(&'a self) -> BoxFieldFutureJson<'a> {
		async move {
			return Err(SeaographyError::new(format!(
				"invalid FieldValue for union `{}`, expected `FieldValue::with_type`",
				self.type_name()
			)));
		}
		.boxed()
	}
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
			let ty = match TYPE_REGISTRY.get(name) {
				Some(ty) => ty.get_type(registry),
				None => panic!("Type {} not found", name),
			};
			types.push(ty);
		}

		let mut meta_type = registry.build_union_type::<Self>(info, &types);

		if let Some(description) = &info.description {
			meta_type = meta_type.description(description);
		}

		meta_type.into_meta()
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
}

impl GraphQLValueAsync<Value> for Union
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
			Field, Interface, InterfaceField, IntoFieldError, JuniperValue, Object, Schema,
			TypeRef, Union,
		},
	};
	use juniper::{
		ExecutionError,
		http::{GraphQLRequest, GraphQLResponse},
		parser::SourcePosition,
	};

	#[tokio::test]
	async fn basic_union() {
		let obj_a = Object::new("MyObjA")
			.field(Field::output("a", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(100))) })
			}))
			.field(Field::output("b", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(200))) })
			}));

		let obj_b = Object::new("MyObjB")
			.field(Field::output("c", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(300))) })
			}))
			.field(Field::output("d", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(400))) })
			}));

		let union =
			Union::new("MyUnion").possible_type(obj_a.type_name()).possible_type(obj_b.type_name());

		let query = Object::new("Query")
			.field(Field::output("valueA", TypeRef::named_nn(union.type_name()), |_| {
				FieldFuture::new(async { Ok(Some(FieldValue::NULL.with_type("MyObjA"))) })
			}))
			.field(Field::output("valueB", TypeRef::named_nn(union.type_name()), |_| {
				FieldFuture::new(async { Ok(Some(FieldValue::NULL.with_type("MyObjB"))) })
			}));

		let schema = Schema::build(query.type_name(), None, None)
			.register(obj_a)
			.register(obj_b)
			.register(union)
			.register(query)
			.finish()
			.unwrap();

		let query = r#"
            {
                valueA { __typename ... on MyObjA { a b } ... on MyObjB { c d } }
                valueB { __typename ... on MyObjA { a b } ... on MyObjB { c d } }
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
							(Value::from("c"), Value::from(300)),
							(Value::from("d"), Value::from(400)),
						])))
					),
				])),
				vec![]
			)))
		);
	}

	#[tokio::test]
	async fn does_not_contain() {
		let obj_a = Object::new("MyObjA")
			.field(Field::output("a", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(100))) })
			}))
			.field(Field::output("b", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(200))) })
			}));

		let obj_b = Object::new("MyObjB")
			.field(Field::output("c", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(300))) })
			}))
			.field(Field::output("d", TypeRef::named_nn(TypeRef::INT), |_| {
				FieldFuture::new(async { Ok(Some(Value::from(400))) })
			}));

		let union = Union::new("MyUnion").possible_type(obj_a.type_name());

		let query = Object::new("Query").field(Field::output(
			"valueA",
			TypeRef::named_nn(union.type_name()),
			|_| FieldFuture::new(async { Ok(Some(FieldValue::NULL.with_type("MyObjB"))) }),
		));

		let schema = Schema::build(query.type_name(), None, None)
			.register(obj_a)
			.register(obj_b)
			.register(union)
			.register(query)
			.finish()
			.unwrap();

		let query = r#"
            {
                valueA { ... on MyObjA { a b } }
            }
        "#;

		let res = schema.executer(GraphQLRequest::new(query.to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::null(),
				vec![ExecutionError::new(
					SourcePosition::new(31, 2, 16),
					&["valueA"],
					SeaographyError::new("union `MyUnion` has no possible type `MyObjB`")
						.into_field_error()
				)]
			)))
		);
	}

	#[tokio::test]
	async fn test_query() {
		struct Dog;
		struct Cat;
		struct Snake;
		// enum
		#[allow(dead_code)]
		enum Animal {
			Dog(Dog),
			Cat(Cat),
			Snake(Snake),
		}
		struct Query<'a> {
			pet: &'a Animal,
		}

		impl Animal {
			fn to_field_value<'a>(&'a self) -> FieldValue<'a> {
				match self {
					Animal::Dog(dog) => FieldValue::borrowed_any(dog).with_type("Dog"),
					Animal::Cat(cat) => FieldValue::borrowed_any(cat).with_type("Cat"),
					Animal::Snake(snake) => FieldValue::borrowed_any(snake).with_type("Snake"),
				}
			}
		}
		fn create_schema() -> Schema {
			// interface
			let named = Interface::new("Named");
			let named =
				named.field(InterfaceField::new("name", TypeRef::named_nn(TypeRef::STRING)));
			// dog
			let dog = Object::new("Dog");
			let dog =
				dog.field(Field::output("name", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
					FieldFuture::new(async move { Ok(Some(Value::from("dog"))) })
				}));
			let dog = dog.field(Field::output("power", TypeRef::named_nn(TypeRef::INT), |_ctx| {
				FieldFuture::new(async move { Ok(Some(Value::from(100))) })
			}));
			let dog = dog.implement("Named");
			// cat
			let cat = Object::new("Cat");
			let cat =
				cat.field(Field::output("name", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
					FieldFuture::new(async move { Ok(Some(Value::from("cat"))) })
				}));
			let cat = cat.field(Field::output("life", TypeRef::named_nn(TypeRef::INT), |_ctx| {
				FieldFuture::new(async move { Ok(Some(Value::from(9))) })
			}));
			let cat = cat.implement("Named");
			// snake
			let snake = Object::new("Snake");
			let snake =
				snake.field(Field::output("length", TypeRef::named_nn(TypeRef::INT), |_ctx| {
					FieldFuture::new(async move { Ok(Some(Value::from(200))) })
				}));
			// animal
			let animal = Union::new("Animal");
			let animal = animal.possible_type("Dog");
			let animal = animal.possible_type("Cat");
			let animal = animal.possible_type("Snake");
			// query

			let query = Object::new("Query");
			let query = query.field(Field::output("pet", TypeRef::named_nn("Animal"), |_| {
				FieldFuture::new(async move {
					let root = Query {
						pet: &Animal::Dog(Dog),
					};
					Ok(Some(root.pet.to_field_value()))
				})
			}));

			let schema = Schema::build(query.type_name(), None, None);
			let schema = schema
				.register(query)
				.register(named)
				.register(dog)
				.register(cat)
				.register(snake)
				.register(animal);

			schema.finish().unwrap()
		}

		let schema = create_schema();
		let query = r#"
            query {
                dog: pet {
                    ... on Dog {
                        __dog_typename: __typename
                        name
                        power
                    }
                }
                named: pet {
                    ... on Named {
                        __named_typename: __typename
                        name
                    }
                }
            }
        "#;
		let res = schema.executer(GraphQLRequest::new(query.to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::object(juniper::Object::from_iter(vec![
					(
						"dog",
						JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![
							(Value::from("__dog_typename"), Value::from("Dog")),
							(Value::from("name"), Value::from("dog")),
							(Value::from("power"), Value::from(100)),
						])))
					),
					(
						"named",
						JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![
							(Value::from("__named_typename"), Value::from("Dog")),
							(Value::from("name"), Value::from("dog")),
						])))
					),
				])),
				vec![]
			)))
		);
	}
}
