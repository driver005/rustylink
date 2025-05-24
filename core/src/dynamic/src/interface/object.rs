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

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}
}

impl Object<GraphQLTypeRef> {
	pub fn to_type(self) -> GraphQLType {
		GraphQLType::Object(
			self.fields.into_iter().fold(GraphQLObject::new(self.name), |builder, (_, field)| {
				builder.field(field.to_field(&self.io))
			}),
		)
	}
}

impl Object<ProtoTypeRef> {
	pub fn to_type(self) -> ProtoType {
		ProtoType::Message(
			self.fields.into_iter().fold(ProtoMessage::new(self.name), |builder, (_, field)| {
				builder.field(field.to_field(&self.io))
			}),
		)
	}
}
