use std::ops::Add;

use super::Field;
use crate::{
	TypeRefTrait,
	prelude::{GraphQLObject, GraphQLType, GraphQLTypeRef, ProtoMessage, ProtoType, ProtoTypeRef},
};
use indexmap::IndexMap;

#[derive(Clone)]
pub enum IO {
	Input,
	Output,
}

pub struct Object<T>
where
	T: TypeRefTrait,
{
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: IndexMap<String, Field<T>>,
	pub(crate) io: IO,
	pub(crate) namespace: Option<String>,
}

impl<T> Object<T>
where
	T: TypeRefTrait,
{
	/// Create a new Protobuf object type
	#[inline]
	pub fn new(name: impl Into<String>, io: IO) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
			io, // arguments: Default::default(),
			namespace: None,
		}
	}

	/// Add an field to the object
	#[inline]
	pub fn field(mut self, field: Field<T>) -> Self {
		assert!(!self.fields.contains_key(&field.name), "Field `{}` already exists", field.name);
		self.fields.insert(field.name.clone(), field);
		self
	}

	/// Returns the type name
	#[inline]
	pub fn field_len(&self) -> usize {
		self.fields.len()
	}

	pub fn oneof(self) -> Self {
		self
	}

	pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
		self.namespace = Some(namespace.into());
		self
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}
}

impl Object<GraphQLTypeRef> {
	pub fn to_type(self) -> GraphQLType {
		GraphQLType::Object(self.fields.into_iter().fold(
			match self.description {
				Some(description) => GraphQLObject::new(self.name).description(description),
				None => GraphQLObject::new(self.name),
			},
			|builder, (_, field)| builder.field(field.to_field(&self.io)),
		))
	}
}

impl Object<ProtoTypeRef> {
	pub fn to_type(self) -> ProtoType {
		ProtoType::Message(
			self.fields
				.into_iter()
				.enumerate()
				.filter(|(_, (_, field))| !field.name.contains("edges"))
				.fold(
					match self.description {
						Some(description) => ProtoMessage::new(self.name).description(description),
						None => ProtoMessage::new(self.name),
					},
					|builder, (index, (_, field))| {
						builder.field(field.to_field(index.add(1) as u32, &self.io))
					},
				),
		)
	}
}
