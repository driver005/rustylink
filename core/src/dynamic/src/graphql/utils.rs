use super::{Argument, Field, JuniperField, Registry, TYPE_REGISTRY, TypeRef};
use crate::{FieldFuture, ObjectAccessor, Value};
use juniper::{ID, LookAheadSelection};
use std::{
	borrow::Cow,
	collections::{BTreeMap, BTreeSet},
};

pub fn type_name(type_name: String) -> (String, Field) {
	(
		type_name.clone(),
		Field::output("__typename", TypeRef::named_nn(TypeRef::STRING), move |_| {
			FieldFuture::from_value(Some(Value::from(type_name.to_owned())))
		}),
	)
}

pub fn to_object_accessor<'a>(look_ahead: LookAheadSelection<'a, Value>) -> ObjectAccessor<'a> {
	let mut btreemap = BTreeMap::new();
	for argument in look_ahead.arguments() {
		btreemap.insert(Value::from(argument.name()), Value::from(argument.value()));
	}
	ObjectAccessor(Cow::Owned(btreemap))
}

#[derive(Clone, Debug)]
pub enum NodeType {
	Query,
	Mutation,
	Subscription,
}

impl NodeType {
	pub(crate) fn type_name(&self) -> &str {
		match self {
			NodeType::Query => "Query",
			NodeType::Mutation => "Mutation",
			NodeType::Subscription => "Subscription",
		}
	}
}

#[derive(Clone, Debug)]
pub struct NodeInfo {
	pub(crate) name: String,
	pub(crate) node_type: NodeType,
}

impl NodeInfo {
	pub fn new(name: String, node_type: NodeType) -> Self {
		Self {
			name,
			node_type,
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.name
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelectionSet {
	pub(crate) name: String,
	pub(crate) alias: Option<String>,
	pub(crate) childs: BTreeSet<SelectionSet>,
}

impl SelectionSet {
	pub fn new(name: String, alias: Option<String>, childs: BTreeSet<SelectionSet>) -> Self {
		Self {
			name,
			alias,
			childs,
		}
	}

	pub fn from_look_ahead(look_ahead: &juniper::LookAheadSelection<'_, Value>) -> Self {
		let name = look_ahead.field_original_name().to_string();
		let alias = look_ahead.field_alias().map(|s| s.to_string());
		let mut childs = BTreeSet::new();
		if !look_ahead.children().is_empty() {
			for child in look_ahead.children().iter() {
				childs.insert(Self::from_look_ahead(child));
			}
		}
		Self::new(name, alias, childs)
	}
}

pub struct TypeRefToMeta<'a> {
	name: &'a str,
	type_name: String,
	list: bool,
	non_null: bool,
	list_non_null: bool,
}

impl<'a> TypeRefToMeta<'a> {
	pub fn new(name: &'a str) -> Self {
		Self {
			name,
			type_name: "".to_string(),
			list: false,
			non_null: false,
			list_non_null: false,
		}
	}

	/// Returns the type name
	#[inline]
	pub fn type_name(&self) -> &str {
		&self.type_name
	}

	pub fn from_type_ref(&mut self, root_ref: &TypeRef) {
		self.from_type_ref_inner(root_ref, false);
	}

	fn from_type_ref_inner(&mut self, root_ref: &TypeRef, inner_vec: bool) {
		match root_ref {
			TypeRef::NonNull(type_ref) => {
				if inner_vec {
					self.list_non_null = true;
				} else {
					self.non_null = true;
				}
				self.from_type_ref_inner(type_ref, inner_vec);
			}
			TypeRef::List(type_ref) => {
				self.list = true;
				self.from_type_ref_inner(type_ref, true);
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
		if self.non_null {
			if self.list {
				if self.list_non_null {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.arg::<Vec<i32>>(self.name, &()),
						TypeRef::FLOAT => registry.arg::<Vec<f64>>(self.name, &()),
						TypeRef::STRING => registry.arg::<Vec<String>>(self.name, &()),
						TypeRef::BOOLEAN => registry.arg::<Vec<bool>>(self.name, &()),
						TypeRef::ID => registry.arg::<Vec<ID>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.argument_vec(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				} else {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.arg::<Vec<Option<i32>>>(self.name, &()),
						TypeRef::FLOAT => registry.arg::<Vec<Option<f64>>>(self.name, &()),
						TypeRef::STRING => registry.arg::<Vec<Option<String>>>(self.name, &()),
						TypeRef::BOOLEAN => registry.arg::<Vec<Option<bool>>>(self.name, &()),
						TypeRef::ID => registry.arg::<Vec<Option<ID>>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.argument_null_vec(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				}
			}
			return match self.type_name.as_str() {
				TypeRef::INT => registry.arg::<i32>(self.name, &()),
				TypeRef::FLOAT => registry.arg::<f64>(self.name, &()),
				TypeRef::STRING => registry.arg::<String>(self.name, &()),
				TypeRef::BOOLEAN => registry.arg::<bool>(self.name, &()),
				TypeRef::ID => registry.arg::<ID>(self.name, &()),
				name => match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.argument(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			};
		} else {
			if self.list {
				if self.list_non_null {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.arg::<Option<Vec<i32>>>(self.name, &()),
						TypeRef::FLOAT => registry.arg::<Option<Vec<f64>>>(self.name, &()),
						TypeRef::STRING => registry.arg::<Option<Vec<String>>>(self.name, &()),
						TypeRef::BOOLEAN => registry.arg::<Option<Vec<bool>>>(self.name, &()),
						TypeRef::ID => registry.arg::<Option<Vec<ID>>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.argument_vec_null(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				} else {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.arg::<Option<Vec<Option<i32>>>>(self.name, &()),
						TypeRef::FLOAT => registry.arg::<Option<Vec<Option<f64>>>>(self.name, &()),
						TypeRef::STRING => {
							registry.arg::<Option<Vec<Option<String>>>>(self.name, &())
						}
						TypeRef::BOOLEAN => {
							registry.arg::<Option<Vec<Option<bool>>>>(self.name, &())
						}
						TypeRef::ID => registry.arg::<Option<Vec<Option<ID>>>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.argument_null_vec_null(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				}
			}
			match self.type_name.as_str() {
				TypeRef::INT => registry.arg::<Option<i32>>(self.name, &()),
				TypeRef::FLOAT => registry.arg::<Option<f64>>(self.name, &()),
				TypeRef::STRING => registry.arg::<Option<String>>(self.name, &()),
				TypeRef::BOOLEAN => registry.arg::<Option<bool>>(self.name, &()),
				TypeRef::ID => registry.arg::<Option<ID>>(self.name, &()),
				name => match TYPE_REGISTRY.get(name) {
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
		if self.non_null {
			if self.list {
				if self.list_non_null {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.field::<Vec<i32>>(self.name, &()),
						TypeRef::FLOAT => registry.field::<Vec<f64>>(self.name, &()),
						TypeRef::STRING => registry.field::<Vec<String>>(self.name, &()),
						TypeRef::BOOLEAN => registry.field::<Vec<bool>>(self.name, &()),
						TypeRef::ID => registry.field::<Vec<ID>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.field_vec(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				} else {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.field::<Vec<Option<i32>>>(self.name, &()),
						TypeRef::FLOAT => registry.field::<Vec<Option<f64>>>(self.name, &()),
						TypeRef::STRING => registry.field::<Vec<Option<String>>>(self.name, &()),
						TypeRef::BOOLEAN => registry.field::<Vec<Option<bool>>>(self.name, &()),
						TypeRef::ID => registry.field::<Vec<Option<ID>>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.field_null_vec(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				}
			}
			return match self.type_name.as_str() {
				TypeRef::INT => registry.field::<i32>(self.name, &()),
				TypeRef::FLOAT => registry.field::<f64>(self.name, &()),
				TypeRef::STRING => registry.field::<String>(self.name, &()),
				TypeRef::BOOLEAN => registry.field::<bool>(self.name, &()),
				TypeRef::ID => registry.field::<ID>(self.name, &()),
				name => match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.field(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			};
		} else {
			if self.list {
				if self.list_non_null {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.field::<Option<Vec<i32>>>(self.name, &()),
						TypeRef::FLOAT => registry.field::<Option<Vec<f64>>>(self.name, &()),
						TypeRef::STRING => registry.field::<Option<Vec<String>>>(self.name, &()),
						TypeRef::BOOLEAN => registry.field::<Option<Vec<bool>>>(self.name, &()),
						TypeRef::ID => registry.field::<Option<Vec<ID>>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.field_vec_null(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				} else {
					return match self.type_name.as_str() {
						TypeRef::INT => registry.field::<Option<Vec<Option<i32>>>>(self.name, &()),
						TypeRef::FLOAT => {
							registry.field::<Option<Vec<Option<f64>>>>(self.name, &())
						}
						TypeRef::STRING => {
							registry.field::<Option<Vec<Option<String>>>>(self.name, &())
						}
						TypeRef::BOOLEAN => {
							registry.field::<Option<Vec<Option<bool>>>>(self.name, &())
						}
						TypeRef::ID => registry.field::<Option<Vec<Option<ID>>>>(self.name, &()),
						name => match TYPE_REGISTRY.get(name) {
							Some(ty) => ty.field_null_vec_null(self.name, registry),
							None => panic!("Unsupported type for non-null argument: {}", name),
						},
					};
				}
			}
			match self.type_name.as_str() {
				TypeRef::INT => registry.field::<Option<i32>>(self.name, &()),
				TypeRef::FLOAT => registry.field::<Option<f64>>(self.name, &()),
				TypeRef::STRING => registry.field::<Option<String>>(self.name, &()),
				TypeRef::BOOLEAN => registry.field::<Option<bool>>(self.name, &()),
				TypeRef::ID => registry.field::<Option<ID>>(self.name, &()),
				name => match TYPE_REGISTRY.get(name) {
					Some(ty) => ty.field_null(self.name, registry),
					None => panic!("Unsupported type for non-null argument: {}", name),
				},
			}
		}
	}
}
