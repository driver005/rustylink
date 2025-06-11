use super::{
	Directive, FieldError, FromInputValue, GraphQLType, GraphQLValue, GraphQLValueAsync,
	InputValue, IntoFieldError, JuniperValue, MetaType, ParseScalarResult, ParseScalarValue,
	Registry, ScalarToken, ToInputValue,
};
use crate::{
	BoxFieldFutureJson, ContextBase, ScalarValidatorFn, SeaResult, SeaographyError, Value,
};
use core::fmt;
use futures::FutureExt;
use std::{fmt::Debug, sync::Arc};

/// A GraphQL scalar type
///
/// # Examples
///
/// ```
/// use async_graphql::{dynamic::*, value, Value};
///
/// let my_scalar = Scalar::new("MyScalar");
///
/// let query = Object::new("Query").field(Field::new("value", TypeRef::named_nn(my_scalar.type_name()), |ctx| {
///     FieldFuture::new(async move { Ok(Some(Value::from("abc"))) })
/// }));
///
/// # tokio::runtime::Runtime::new().unwrap().block_on(async move {
///
/// let schema = Schema::build(query.type_name(), None, None)
///     .register(my_scalar)
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
pub struct Scalar {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) specified_by_url: Option<String>,
	pub(crate) validator: Option<ScalarValidatorFn>,
	pub(crate) inaccessible: bool,
	pub(crate) directives: Vec<Directive>,
}

impl Debug for Scalar {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Scalar")
			.field("name", &self.name)
			.field("description", &self.description)
			.field("specified_by_url", &self.specified_by_url)
			.field("inaccessible", &self.inaccessible)
			.finish()
	}
}

impl Scalar {
	/// Create a GraphQL scalar type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			name,
			description: None,
			specified_by_url: None,
			validator: None,
			inaccessible: false,
			directives: Vec::new(),
		}
	}

	impl_set_description!();
	impl_set_inaccessible!();
	impl_directive!();

	/// Set the validator
	#[inline]
	pub fn validator(self, validator: impl Fn(&Value) -> bool + Send + Sync + 'static) -> Self {
		Self {
			validator: Some(Arc::new(validator)),
			..self
		}
	}

	#[inline]
	pub(crate) fn validate(&self, value: &Value) -> bool {
		match &self.validator {
			Some(validator) => (validator)(value),
			None => true,
		}
	}

	/// Set the specified by url
	#[inline]
	pub fn specified_by_url(self, specified_by_url: impl Into<String>) -> Self {
		Self {
			specified_by_url: Some(specified_by_url.into()),
			..self
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	pub(crate) fn to_value(&self, value: &Value) -> SeaResult<Value> {
		if self.inaccessible {
			return Err(SeaographyError::new(format!(
				"scalar `{}` is inaccessible",
				self.type_name()
			)));
		}
		if self.validate(&value) {
			return Ok(value.to_owned());
		}
		Err(SeaographyError::new(format!("invalid value `{}` for scalar `{}`", value, self.name)))
	}

	pub(crate) fn collect<'a>(&'a self) -> BoxFieldFutureJson<'a> {
		async move {
			return Err(SeaographyError::new(format!(
				"invalid FieldValue for scalar `{}`, expected `FieldValue::Value`",
				self.type_name()
			)));
		}
		.boxed()
	}

	// #[doc(hidden)]
	// pub fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
	// 	registry.types.insert(
	// 		self.name.clone(),
	// 		MetaType::Scalar {
	// 			name: self.name.clone(),
	// 			description: self.description.clone(),
	// 			is_valid: self.validator.clone(),
	// 			visible: None,
	// 			inaccessible: self.inaccessible,
	// 			tags: self.tags.clone(),
	// 			specified_by_url: self.specified_by_url.clone(),
	// 			directive_invocations: to_meta_directive_invocation(self.directives.clone()),
	// 			requires_scopes: self.requires_scopes.clone(),
	// 		},
	// 	);
	// 	Ok(())
	// }
}

impl GraphQLType<Value> for Scalar {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		let mut meta_type = registry.build_scalar_type::<Self>(info);

		if let Some(description) = &info.description {
			meta_type = meta_type.description(description);
		}

		meta_type.into_meta()
	}
}

impl ParseScalarValue<Value> for Scalar {
	fn from_str(value: ScalarToken<'_>) -> ParseScalarResult<Value> {
		<String as ParseScalarValue<Value>>::from_str(value)
			.or_else(|_| <i32 as ParseScalarValue<Value>>::from_str(value))
	}
}

impl ToInputValue<Value> for Scalar {
	fn to_input_value(&self) -> InputValue<Value> {
		let v = JuniperValue::scalar(self.name.clone());
		ToInputValue::to_input_value(&v)
	}
}

impl FromInputValue<Value> for Scalar {
	type Error = FieldError<Value>;
	fn from_input_value(input: &InputValue<Value>) -> Result<Self, Self::Error> {
		println!("input: {:?}", input);
		input
			.as_string_value()
			.map(|s| Scalar::new(s))
			.ok_or_else(|| format!("Expected `String` or `Int`, found: {input}"))
			.map_err(IntoFieldError::<Value>::into_field_error)
		// 	.as_string_value()
		// 	.map(|s| Self::String(s.into()))
		// 	.or_else(|| v.as_int_value().map(Self::Int))
		// 	.ok_or_else(|| format!("Expected `String` or `Int`, found: {v}"))
		// 	.map_err(IntoFieldError::<Value>::into_field_error)
	}
}

impl GraphQLValue<Value> for Scalar {
	type Context = ContextBase;
	type TypeInfo = Self;
	fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
		Some(info.type_name())
	}
}

impl GraphQLValueAsync<Value> for Scalar
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
		graphql::{Field, IntoFieldError, JuniperValue, Object, Scalar, Schema, TypeRef},
	};
	use juniper::{
		ExecutionError,
		http::{GraphQLRequest, GraphQLResponse},
		parser::SourcePosition,
	};

	#[tokio::test]
	async fn custom_scalar() {
		let scalar = Scalar::new("MyScalar");
		let query = Object::new("Query").field(Field::output(
			"value",
			TypeRef::named_nn(scalar.type_name()),
			|_| {
				FieldFuture::new(async move {
					Ok(Some(Value::from(BTreeMap::from_iter(vec![
						(Value::from("a"), Value::from(1)),
						(Value::from("b"), Value::from("abc")),
					]))))
				})
			},
		));

		let schema = Schema::build(query.type_name(), None, None)
			.register(query)
			.register(scalar)
			.finish()
			.unwrap();

		let res = schema.executer(GraphQLRequest::new("{ value }".to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::object(juniper::Object::from_iter(vec![(
					"value",
					JuniperValue::scalar(Value::Map(BTreeMap::from_iter(vec![
						(Value::from("a"), Value::from(1)),
						(Value::from("b"), Value::from("abc")),
					])))
				),])),
				vec![]
			)))
		);
	}

	#[tokio::test]
	async fn invalid_scalar_value() {
		let scalar = Scalar::new("MyScalar");
		let query = Object::new("Query").field(Field::output(
			"value",
			TypeRef::named_nn(scalar.type_name()),
			|_| FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(10i32))) }),
		));

		let schema = Schema::build(query.type_name(), None, None)
			.register(query)
			.register(scalar)
			.finish()
			.unwrap();

		let res = schema.executer(GraphQLRequest::new("{ value }".to_string(), None, None)).await;

		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::null(),
				vec![ExecutionError::new(
					SourcePosition::new(2, 0, 2),
					&["value"],
					SeaographyError::new(
						"invalid FieldValue for scalar `MyScalar`, expected `FieldValue::Value`"
					)
					.into_field_error()
				)]
			)))
		);
	}
}
