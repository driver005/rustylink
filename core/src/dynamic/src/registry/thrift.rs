use std::collections::{BTreeMap, HashSet};

use crate::ScalarValidatorFn;

use super::MetaVisibleFn;

pub struct ThriftField {
	pub name: String,
	pub description: Option<String>,
	pub field_type: String,
	pub key: i16,
	pub required: bool,
}

pub struct ThriftEnumValue {
	pub name: String,
	pub description: Option<String>,
	pub value: i32,
}

pub struct ThriftMethod {
	pub name: String,
	pub description: Option<String>,
	pub return_type: String,
	pub arguments: Vec<ThriftField>,
	pub throws: Vec<ThriftField>,
	pub oneway: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ThriftTypeId {
	Scalar,
	Struct,
	Union,
	Exception,
	Enum,
	Service,
}

pub enum ThriftKind {
	Scalar {
		name: String,
		description: Option<String>,
		is_valid: Option<ScalarValidatorFn>,
		visible: Option<MetaVisibleFn>,
	},
	Struct {
		name: String,
		description: Option<String>,
		fields: BTreeMap<String, ThriftField>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Union {
		name: String,
		description: Option<String>,
		fields: BTreeMap<String, ThriftField>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Exception {
		name: String,
		description: Option<String>,
		fields: BTreeMap<String, ThriftField>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Enum {
		name: String,
		description: Option<String>,
		fields: BTreeMap<String, ThriftEnumValue>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Service {
		name: String,
		description: Option<String>,
		methods: BTreeMap<String, ThriftMethod>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
}

impl ThriftKind {
	#[inline]
	pub fn type_id(&self) -> ThriftTypeId {
		match self {
			ThriftKind::Scalar {
				..
			} => ThriftTypeId::Scalar,
			ThriftKind::Struct {
				..
			} => ThriftTypeId::Struct,
			ThriftKind::Union {
				..
			} => ThriftTypeId::Union,
			ThriftKind::Exception {
				..
			} => ThriftTypeId::Exception,
			ThriftKind::Enum {
				..
			} => ThriftTypeId::Enum,
			ThriftKind::Service {
				..
			} => ThriftTypeId::Service,
		}
	}

	#[inline]
	pub fn field_by_name(&self, name: &str) -> Option<&ThriftField> {
		self.fields().and_then(|fields| fields.get(name))
	}

	#[inline]
	pub fn name(&self) -> &str {
		match self {
			ThriftKind::Scalar {
				name,
				..
			} => name,
			ThriftKind::Struct {
				name,
				..
			} => name,
			ThriftKind::Union {
				name,
				..
			} => name,
			ThriftKind::Exception {
				name,
				..
			} => name,
			ThriftKind::Enum {
				name,
				..
			} => name,
			ThriftKind::Service {
				name,
				..
			} => name,
		}
	}

	#[inline]
	pub fn fields(&self) -> Option<&BTreeMap<String, ThriftField>> {
		match self {
			ThriftKind::Struct {
				fields,
				..
			} => Some(fields),
			ThriftKind::Union {
				fields,
				..
			} => Some(fields),
			ThriftKind::Exception {
				fields,
				..
			} => Some(fields),
			_ => None,
		}
	}

	#[inline]
	pub fn is_input(&self) -> bool {
		matches!(
			self,
			ThriftKind::Enum { .. } | ThriftKind::Scalar { .. } | ThriftKind::Struct { .. }
		)
	}

	#[inline]
	pub fn rust_typename(&self) -> Option<&'static str> {
		match self {
			ThriftKind::Scalar {
				..
			} => None,
			ThriftKind::Struct {
				rust_typename,
				..
			} => *rust_typename,
			ThriftKind::Union {
				rust_typename,
				..
			} => *rust_typename,
			ThriftKind::Exception {
				rust_typename,
				..
			} => *rust_typename,
			ThriftKind::Enum {
				rust_typename,
				..
			} => *rust_typename,
			ThriftKind::Service {
				rust_typename,
				..
			} => *rust_typename,
		}
	}
}

/// A type registry for build schemas
pub struct ThriftRegistry {
	pub types: BTreeMap<String, ThriftKind>,
	// pub proto_type: String,
	// pub proto_enum_type: String,
	// pub mutation_type: Option<String>,
	// pub subscription_type: Option<String>,
	pub ignore_name_conflicts: HashSet<String>,
}

impl ThriftRegistry {
	pub fn new() -> Self {
		Self {
			types: BTreeMap::default(),
			ignore_name_conflicts: HashSet::new(),
		}
	}

	pub fn build(&self) -> String {
		let sdl = String::new();

		// println!("{}", sdl);
		sdl
	}
}
