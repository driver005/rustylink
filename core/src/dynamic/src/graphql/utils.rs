use super::{Argument, JuniperField, Registry, TypeRef, get_type};
use crate::Value;
use juniper::ID;

pub struct TypeRefToMeta<'a> {
	name: &'a str,
	type_name: String,
	list: bool,
	non_null: bool,
}

impl<'a> TypeRefToMeta<'a> {
	pub fn new(name: &'a str) -> Self {
		Self {
			name,
			type_name: "".to_string(),
			list: false,
			non_null: false,
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.type_name
	}

	pub fn from_type_ref(&mut self, root_ref: &TypeRef) {
		match root_ref {
			TypeRef::NonNull(type_ref) => {
				self.non_null = true;
				self.from_type_ref(type_ref);
			}
			TypeRef::List(type_ref) => {
				self.list = true;
				self.from_type_ref(type_ref);
			}
			TypeRef::Named(type_ref) => {
				self.type_name = type_ref.to_string();
			}
		}
	}

	pub(crate) fn to_input_meta<'r>(
		&self,
		registry: &mut Registry<'r, Value>,
	) -> Argument<'r, Value> {
		if self.list {
			if self.non_null {
				return match self.type_name.as_str() {
					TypeRef::INT => registry.arg::<Vec<i32>>(self.name, &()),
					TypeRef::FLOAT => registry.arg::<Vec<f64>>(self.name, &()),
					TypeRef::STRING => registry.arg::<Vec<String>>(self.name, &()),
					TypeRef::BOOLEAN => registry.arg::<Vec<bool>>(self.name, &()),
					TypeRef::ID => registry.arg::<Vec<ID>>(self.name, &()),
					name => match get_type(name) {
						Some(ty) => ty.argument_vec(self.name, registry),
						None => panic!("Unsupported type for non-null argument: {}", name),
					},
				};
			} else {
				return match self.type_name.as_str() {
					TypeRef::INT => registry.arg::<Option<Vec<i32>>>(self.name, &()),
					TypeRef::FLOAT => registry.arg::<Option<Vec<f64>>>(self.name, &()),
					TypeRef::STRING => registry.arg::<Option<Vec<String>>>(self.name, &()),
					TypeRef::BOOLEAN => registry.arg::<Option<Vec<bool>>>(self.name, &()),
					TypeRef::ID => registry.arg::<Option<Vec<ID>>>(self.name, &()),
					name => match get_type(name) {
						Some(ty) => ty.argument_vec_null(self.name, registry),
						None => panic!("Unsupported type for non-null argument: {}", name),
					},
				};
			}
		}
		if self.non_null {
			return match self.type_name.as_str() {
				TypeRef::INT => registry.arg::<i32>(self.name, &()),
				TypeRef::FLOAT => registry.arg::<f64>(self.name, &()),
				TypeRef::STRING => registry.arg::<String>(self.name, &()),
				TypeRef::BOOLEAN => registry.arg::<bool>(self.name, &()),
				TypeRef::ID => registry.arg::<ID>(self.name, &()),
				name => match get_type(name) {
					Some(ty) => ty.argument(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			};
		} else {
			match self.type_name.as_str() {
				TypeRef::INT => registry.arg::<Option<i32>>(self.name, &()),
				TypeRef::FLOAT => registry.arg::<Option<f64>>(self.name, &()),
				TypeRef::STRING => registry.arg::<Option<String>>(self.name, &()),
				TypeRef::BOOLEAN => registry.arg::<Option<bool>>(self.name, &()),
				TypeRef::ID => registry.arg::<Option<ID>>(self.name, &()),
				name => match get_type(name) {
					Some(ty) => ty.argument_null(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			}
		}
	}

	pub(crate) fn to_ouput_meta<'r>(
		&self,
		registry: &mut Registry<'r, Value>,
	) -> JuniperField<'r, Value> {
		if self.list {
			if self.non_null {
				return match self.type_name.as_str() {
					TypeRef::INT => registry.field::<Vec<i32>>(self.name, &()),
					TypeRef::FLOAT => registry.field::<Vec<f64>>(self.name, &()),
					TypeRef::STRING => registry.field::<Vec<String>>(self.name, &()),
					TypeRef::BOOLEAN => registry.field::<Vec<bool>>(self.name, &()),
					TypeRef::ID => registry.field::<Vec<ID>>(self.name, &()),
					name => match get_type(name) {
						Some(ty) => ty.field_vec(self.name, registry),
						None => panic!("Unsupported type for non-null argument: {}", name),
					},
				};
			} else {
				return match self.type_name.as_str() {
					TypeRef::INT => registry.field::<Option<Vec<i32>>>(self.name, &()),
					TypeRef::FLOAT => registry.field::<Option<Vec<f64>>>(self.name, &()),
					TypeRef::STRING => registry.field::<Option<Vec<String>>>(self.name, &()),
					TypeRef::BOOLEAN => registry.field::<Option<Vec<bool>>>(self.name, &()),
					TypeRef::ID => registry.field::<Option<Vec<ID>>>(self.name, &()),
					name => match get_type(name) {
						Some(ty) => ty.field_vec_null(self.name, registry),
						None => panic!("Unsupported type for non-null argument: {}", name),
					},
				};
			}
		}
		if self.non_null {
			return match self.type_name.as_str() {
				TypeRef::INT => registry.field::<i32>(self.name, &()),
				TypeRef::FLOAT => registry.field::<f64>(self.name, &()),
				TypeRef::STRING => registry.field::<String>(self.name, &()),
				TypeRef::BOOLEAN => registry.field::<bool>(self.name, &()),
				TypeRef::ID => registry.field::<ID>(self.name, &()),
				name => match get_type(name) {
					Some(ty) => ty.field(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			};
		} else {
			match self.type_name.as_str() {
				TypeRef::INT => registry.field::<Option<i32>>(self.name, &()),
				TypeRef::FLOAT => registry.field::<Option<f64>>(self.name, &()),
				TypeRef::STRING => registry.field::<Option<String>>(self.name, &()),
				TypeRef::BOOLEAN => registry.field::<Option<bool>>(self.name, &()),
				TypeRef::ID => registry.field::<Option<ID>>(self.name, &()),
				name => match get_type(name) {
					Some(ty) => ty.field_null(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			}
		}
	}
}
