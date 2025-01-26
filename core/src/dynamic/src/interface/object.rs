use super::Field;
use indexmap::IndexMap;

pub struct Object {
	pub(crate) name: String,
	pub(crate) description: Option<String>,
	pub(crate) fields: IndexMap<String, Field>,
}

impl Object {
	/// Create a new Protobuf object type
	#[inline]
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: None,
			fields: Default::default(),
			// arguments: Default::default(),
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
}
