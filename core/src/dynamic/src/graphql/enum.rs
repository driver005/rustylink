use super::{
	DeprecationStatus, Directive, EnumValue, FromInputValue, GraphQLType, GraphQLValue,
	GraphQLValueAsync, InputValue, MetaType, Registry, ScalarValue,
};
use crate::{
	BoxFieldFutureJson, ContextBase, EnumItemTrait, EnumTrait, SeaResult, SeaographyError, Value,
};
use futures::FutureExt;
use std::collections::BTreeMap;

/// A GraphQL enum item
#[derive(Debug)]
pub struct EnumItem {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) deprecation: DeprecationStatus,
	pub(crate) inaccessible: bool,
	pub(crate) directives: Vec<Directive>,
}

impl<T: Into<String>> From<T> for EnumItem {
	#[inline]
	fn from(name: T) -> Self {
		EnumItem {
			name: name.into(),
			description: None,
			deprecation: DeprecationStatus::Current,
			inaccessible: false,
			directives: Vec::new(),
		}
	}
}

impl EnumItem {
	/// Create a new EnumItem
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		name.into().into()
	}

	impl_set_description!();
	impl_set_deprecation!();
	impl_set_inaccessible!();
	impl_directive!();

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> &str {
		&self.name
	}
}

impl EnumItemTrait for EnumItem {
	/// Create a new EnumItem
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		EnumItem::new(name)
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> &str {
		self.type_name()
	}
}

/// A GraphQL enum type
#[derive(Debug)]
pub struct Enum {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) enum_values: BTreeMap<String, EnumItem>,
	pub(crate) inaccessible: bool,
	pub(crate) directives: Vec<Directive>,
}

impl Enum {
	/// Create a GraphqL enum type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			enum_values: Default::default(),
			inaccessible: false,
			directives: Vec::new(),
		}
	}

	impl_set_description!();
	impl_directive!();

	/// Add an item
	#[inline]
	pub fn item(mut self, item: impl Into<EnumItem>) -> Self {
		let item = item.into();
		self.enum_values.insert(item.name.clone(), item);
		self
	}

	/// Add items
	pub fn items(mut self, items: impl IntoIterator<Item = impl Into<EnumItem>>) -> Self {
		for item in items {
			let item = item.into();
			self.enum_values.insert(item.name.clone(), item);
		}
		self
	}

	impl_set_inaccessible!();

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	pub(crate) fn to_value(&self, value: &Value) -> SeaResult<Value> {
		if self.inaccessible {
			return Err(SeaographyError::new(format!(
				"enum `{}` is inaccessible",
				self.type_name()
			)));
		}
		if let Some(name) = value.as_string() {
			if let Some(item) = self.enum_values.get(&name) {
				if item.inaccessible {
					return Err(SeaographyError::new(format!(
						"enum `{}` with field `{}` is inaccessible",
						self.type_name(),
						item.type_name()
					)));
				}
				return Ok(value.to_owned());
			}
		}
		Err(SeaographyError::new(format!("enum `{}` has no value of `{}`", self.name, value)))
	}

	pub(crate) fn collect<'a>(&'a self) -> BoxFieldFutureJson<'a> {
		async move {
			return Err(SeaographyError::new(format!(
				"invalid FieldValue for enum `{}`, expected `FieldValue::Value`",
				self.type_name()
			)));
		}
		.boxed()
	}
}

impl EnumTrait for Enum {
	type Item = EnumItem;

	/// Create a GraphqL enum type
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		Enum::new(name)
	}

	/// Add an item
	#[inline]
	fn item(self, item: impl Into<Self::Item>) -> Self {
		self.item(item)
	}

	/// Add items
	fn items(self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self {
		self.items(fields)
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> &str {
		self.type_name()
	}
}

impl GraphQLType<Value> for Enum {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		let mut variants = vec![];

		for (_, item) in info.enum_values.iter() {
			let mut enum_val = EnumValue::new(&item.name);
			if let Some(description) = &item.description {
				enum_val = enum_val.description(description);
			}
			if let DeprecationStatus::Deprecated(reason) = &item.deprecation {
				enum_val = enum_val.deprecated(match reason {
					None => None,
					Some(reason) => Some(reason),
				});
			}
			variants.push(enum_val);
		}

		let mut meta_type = registry.build_enum_type::<Self>(info, &variants);

		if let Some(description) = &info.description {
			meta_type = meta_type.description(description);
		}

		meta_type.into_meta()
	}
}

impl FromInputValue<Value> for Enum {
	type Error = SeaographyError;

	fn from_input_value(v: &juniper::InputValue<Value>) -> Result<Self, Self::Error> {
		println!("v: {:?}", v);
		if let InputValue::Enum(name) = v {
			Ok(Self::new(name))
		} else {
			Err(SeaographyError::new(format!("internal: expected enum value, got {:?}", v)))
		}
	}
}

impl GraphQLValue<Value> for Enum {
	type Context = ContextBase;
	type TypeInfo = Self;
	fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
		Some(info.type_name())
	}

	fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
		info.type_name().to_string()
	}
}

impl GraphQLValueAsync<Value> for Enum
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
}

#[cfg(test)]
mod tests {
	use crate::{
		FieldFuture, FieldValue, SeaographyError, Value,
		graphql::{Enum, Field, IntoFieldError, JuniperValue, Object, Schema, TypeRef},
	};
	use juniper::{
		ExecutionError,
		http::{GraphQLRequest, GraphQLResponse},
		parser::SourcePosition,
	};

	#[tokio::test]
	async fn enum_type() {
		let my_enum = Enum::new("MyEnum").item("A").item("B");

		let query = Object::new("Query")
			.field(Field::output("value", TypeRef::named_nn(my_enum.type_name()), |_| {
				FieldFuture::new(async { Ok(Some(Value::from("A"))) })
			}))
			.field(
				Field::output("value2", TypeRef::named_nn(my_enum.type_name()), |ctx| {
					FieldFuture::new(async move {
						Ok(Some(FieldValue::value(ctx.args.try_get("input")?.enum_name()?)))
					})
				})
				.argument(Field::input("input", TypeRef::named_nn(my_enum.type_name()))),
			);

		let schema = Schema::build(query.type_name(), None, None)
			.register(my_enum)
			.register(query)
			.finish()
			.unwrap();

		let res = schema
			.executer(GraphQLRequest::new("{ value value2(input: B) }".to_string(), None, None))
			.await;
		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::object(juniper::Object::from_iter(
					vec![
						("value", JuniperValue::scalar(Value::from("A"))),
						("value2", JuniperValue::scalar(Value::from("B"))),
					]
					.into_iter()
				)),
				vec![]
			)))
		);
	}

	#[tokio::test]
	async fn enum_wrong_value() {
		let my_enum = Enum::new("MyEnum").item("A").item("B");

		let query = Object::new("Query").field(Field::output(
			"errValue",
			TypeRef::named_nn(my_enum.type_name()),
			|_| FieldFuture::new(async { Ok(Some(Value::from("C"))) }),
		));

		let schema = Schema::build(query.type_name(), None, None)
			.register(my_enum)
			.register(query)
			.finish()
			.unwrap();

		let res =
			schema.executer(GraphQLRequest::new("{ errValue }".to_string(), None, None)).await;
		assert_eq!(
			res,
			GraphQLResponse::from_result(Ok((
				JuniperValue::null(),
				vec![ExecutionError::new(
					SourcePosition::new(2, 0, 2),
					&["errValue"],
					SeaographyError::new("enum `MyEnum` has no value of `String: (\"C\")`")
						.into_field_error()
				)]
			)))
		);
	}
}
