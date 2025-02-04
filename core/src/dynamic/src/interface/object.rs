use super::Field;
use crate::prelude::{GraphQLInputObject, GraphQLObject, GraphQLType, ProtoMessage, ProtoType};
use indexmap::IndexMap;

#[derive(Clone)]
pub enum IO {
	Input,
	Output,
}

pub struct Object {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: IndexMap<String, Field>,
	pub(crate) io: IO,
}

impl Object {
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
	pub fn field(mut self, field: Field) -> Self {
		assert!(
			!self.fields.contains_key(&field.name),
			"Field `{}` already exists",
			field.name.as_str()
		);
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

	pub fn to_graphql(self) -> GraphQLType {
		match self.io {
			IO::Input => GraphQLType::InputObject(
				self.fields
					.into_iter()
					.fold(GraphQLInputObject::new(self.name), |builder, (_, field)| {
						builder.field(field.to_graphql_input())
					}),
			),
			IO::Output => GraphQLType::Object(
				self.fields
					.into_iter()
					.fold(GraphQLObject::new(self.name), |builder, (_, field)| {
						builder.field(field.to_graphql_output())
					}),
			),
		}
	}

	pub fn to_proto(self) -> ProtoType {
		ProtoType::Message(
			self.fields.into_iter().fold(ProtoMessage::new(self.name), |builder, (_, field)| {
				builder.field(field.to_proto(self.io.clone()))
			}),
		)
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}
}
