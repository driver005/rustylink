use super::{
	Directive, EnumValue, ExecutionResult, Executor, FromInputValue, GraphQLType, GraphQLValue,
	GraphQLValueAsync, InputValue, JuniperValue, MetaType, Registry, ScalarValue, Selection,
	ToInputValue,
};
use crate::{
	BoxFieldFuture, ContextBase, EnumItemTrait, EnumTrait, ObjectAccessor, ObjectAccessorTrait,
	SeaographyError, Value, ValueAccessorTrait,
};
use async_graphql::registry::Deprecation;
use futures::{
	FutureExt,
	future::{self, BoxFuture},
};
use std::collections::BTreeMap;

/// A GraphQL enum item
#[derive(Debug)]
pub struct EnumItem {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) deprecation: Deprecation,
	inaccessible: bool,
	tags: Vec<String>,
	pub(crate) directives: Vec<Directive>,
}

impl<T: Into<String>> From<T> for EnumItem {
	#[inline]
	fn from(name: T) -> Self {
		EnumItem {
			name: name.into(),
			description: None,
			deprecation: Deprecation::NoDeprecated,
			inaccessible: false,
			tags: Vec::new(),
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
	impl_set_tags!();
	impl_directive!();
}

impl EnumItemTrait for EnumItem {
	/// Create a new EnumItem
	#[inline]
	fn new(name: impl Into<String>, _tag: u32) -> Self {
		EnumItem::new(name)
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> &str {
		&self.name
	}
}

/// A GraphQL enum type
#[derive(Debug)]
pub struct Enum {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) enum_values: BTreeMap<String, EnumItem>,
	inaccessible: bool,
	tags: Vec<String>,
	pub(crate) directives: Vec<Directive>,
	requires_scopes: Vec<String>,
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
			tags: Vec::new(),
			directives: Vec::new(),
			requires_scopes: Vec::new(),
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
	impl_set_tags!();

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}

	pub(crate) fn collect<'a>(&'a self, arguments: &'a ObjectAccessor<'a>) -> BoxFieldFuture<'a> {
		async move {
			if !self.enum_values.contains_key(&self.name) {
				return Err(SeaographyError::new(format!(
					"internal: invalid item for enum \"{}\"",
					self.name
				)));
			}
			let resolve_fut = async {
				let value = match arguments.get(self.name.as_str()) {
					Some(val) => val.as_value().to_owned(),
					None => Value::Null,
				};

				Ok::<Value, SeaographyError>(value)
			};
			futures_util::pin_mut!(resolve_fut);

			Ok((Value::from(self.name.clone()), resolve_fut.await?))
		}
		.boxed()
	}

	// pub(crate) fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
	// 	let mut enum_values = IndexMap::new();

	// 	for item in self.enum_values.values() {
	// 		enum_values.insert(
	// 			item.name.clone(),
	// 			MetaEnumValue {
	// 				name: item.name.as_str().into(),
	// 				description: item.description.clone(),
	// 				deprecation: item.deprecation.clone(),
	// 				visible: None,
	// 				inaccessible: item.inaccessible,
	// 				tags: item.tags.clone(),
	// 				directive_invocations: to_meta_directive_invocation(item.directives.clone()),
	// 			},
	// 		);
	// 	}

	// 	registry.types.insert(
	// 		self.name.clone(),
	// 		MetaType::Enum {
	// 			name: self.name.clone(),
	// 			description: self.description.clone(),
	// 			enum_values,
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
			variants.push(EnumValue::new(&item.name));
		}

		registry.build_enum_type::<Self>(info, &variants).into_meta()
	}
}

impl FromInputValue<Value> for Enum {
	type Error = SeaographyError;

	fn from_input_value(v: &juniper::InputValue<Value>) -> Result<Self, Self::Error> {
		println!("v: {:?}", v);
		todo!()
	}
}

impl ToInputValue<Value> for Enum {
	fn to_input_value(&self) -> InputValue<Value> {
		let v = JuniperValue::scalar(self.name.clone());
		ToInputValue::to_input_value(&v)
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

	fn resolve(
		&self,
		info: &Self::TypeInfo,
		selection_set: Option<&[Selection<Value>]>,
		executor: &Executor<Self::Context, Value>,
	) -> ExecutionResult<Value> {
		todo!()
	}
}

impl GraphQLValueAsync<Value> for Enum
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
	fn resolve_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		selection_set: Option<&'a [Selection<Value>]>,
		executor: &'a Executor<Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		Box::pin(future::ready(GraphQLValue::resolve(self, info, selection_set, executor)))
	}
}
