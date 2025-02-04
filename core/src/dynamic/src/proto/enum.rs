use super::{BoxFieldFuture, Error, ObjectAccessor, Result, SchemaError, Value};
use crate::{
	prelude::Name, traits::EnumTrait, ObjectAccessorTrait, ProtobufEnumValue, ProtobufKind,
	Registry, ValueAccessorTrait,
};
use binary::proto::{DecoderLit, Encoder, EncoderLit};
use futures_util::FutureExt;
use indexmap::IndexMap;

/// A GraphQL enum item
#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem {
	pub(crate) name: String,
	pub(crate) tag: u32,
	pub(crate) description: Option<String>,
	pub(crate) deprecation: bool,
}

impl<T: Into<String>> From<(T, u32)> for EnumItem {
	#[inline]
	fn from((name, tag): (T, u32)) -> Self {
		EnumItem {
			name: name.into(),
			tag,
			description: None,
			deprecation: false,
		}
	}
}

impl EnumItem {
	/// Create a new EnumItem
	#[inline]
	pub fn new<N>(name: N, tag: u32) -> Self
	where
		N: Into<String>,
	{
		Self {
			name: name.into(),
			description: None,
			tag,
			deprecation: false,
		}
	}

	// impl_set_description!();
}

/// A GraphQL enum type
#[derive(Debug)]
pub struct Enum {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: IndexMap<String, EnumItem>,
}

impl EnumTrait for Enum {
	type Item = EnumItem;

	/// Create a Proto enum type
	#[inline]
	fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
		}
	}

	/// Add an item
	#[inline]
	fn item(mut self, item: impl Into<Self::Item>) -> Self {
		let field = item.into();
		self.fields.insert(field.name.clone(), field);
		self
	}

	/// Add items
	fn items(mut self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self {
		for field in fields {
			let field = field.into();
			self.fields.insert(field.name.clone(), field);
		}
		self
	}

	/// Returns the type name
	#[inline]
	fn type_name(&self) -> String {
		self.name.clone()
	}
}

impl Enum {
	/// Get an item of the object
	#[inline]
	pub fn get_item(&self, name: &str) -> Option<&EnumItem> {
		self.fields.get(name)
	}

	pub(crate) fn collect<'a>(&'a self, arguments: &'a ObjectAccessor<'a>) -> BoxFieldFuture<'a> {
		async move {
			let resolve_fut = async {
				let value = match arguments.get(&self.name) {
					Some(val) => val.as_value().to_owned(),
					None => Value::Null,
				};

				Ok::<Value, Error>(value)
			};
			futures_util::pin_mut!(resolve_fut);

			Ok((Name::new(self.name.clone()), resolve_fut.await?))
		}
		.boxed()
	}

	pub fn encode(&self, buf: &mut Vec<u8>, value: &Value, tag: u32) -> Result<usize> {
		let encoder = Encoder::default();

		match value {
			Value::Enum((_, value)) => Ok(encoder.encode((&tag, EncoderLit::Int32(value)), buf)?),
			_ => Ok(0),
		}
	}

	pub fn bytes(&self, buf: Vec<u8>) -> Result<Value> {
		Ok(i32::from(DecoderLit::Int32(buf)).into())
	}

	// pub(crate) async fn resolve(&self, value: &FieldValue<'_>) -> Result<Option<Value>> {
	// 	match value.as_value() {
	// 		Some(v) => match v {
	// 			Value::Enum((_, number)) => {
	// 				if !self.fields.contains_key(self.type_name()) {
	// 					return Err(Error::new(format!(
	// 						"internal: invalid item for enum \"{}\"",
	// 						self.name
	// 					)));
	// 				}
	// 				Ok(Some(Value::Enum((Name::new(self.type_name()), *number))))
	// 			}
	// 			Value::Int32(number) => {
	// 				if !self.fields.contains_key(self.type_name()) {
	// 					return Err(Error::new(format!(
	// 						"internal: invalid item for enum \"{}\"",
	// 						self.name
	// 					)));
	// 				}
	// 				Ok(Some(Value::Enum((Name::new(self.type_name()), *number))))
	// 			}
	// 			_ => Err(Error::new(format!(
	// 				"internal: invalid item for enum \"{}\"",
	// 				self.type_name()
	// 			))),
	// 		},
	// 		None => {
	// 			Err(Error::new(format!("internal: invalid item for enum \"{}\"", self.type_name())))
	// 		}
	// 	}
	// }

	pub(crate) fn register(&self, registry: &mut Registry) -> Result<(), SchemaError> {
		let mut fields = IndexMap::new();

		for (index, item) in self.fields.values().enumerate() {
			fields.insert(
				item.name.to_string(),
				ProtobufEnumValue {
					name: item.name.to_string(),
					description: item.description.clone(),
					tag: index as i32,
				},
			);
		}

		registry.types.proto.insert(
			self.name.to_string(),
			ProtobufKind::Enum {
				name: self.name.to_string(),
				description: self.description.clone(),
				fields,
				visible: None,
				rust_typename: None,
			},
		);

		Ok(())
	}
}
