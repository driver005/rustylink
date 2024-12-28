use crate::proto::ScalarValidatorFn as ProtoScalarValidatorFn;
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Write;

// Define MetaVisibleFn type
pub type MetaVisibleFn = fn() -> bool;

pub struct ProtobufField {
	pub name: String,
	pub description: Option<String>,
	pub field_type: String,
	pub tag: u32,
	pub label: Option<ProtobufFieldLabel>,
}

pub enum ProtobufFieldLabel {
	Optional,
	Required,
	Repeated,
}

pub struct ProtobufOneofGroup {
	pub name: String,
	pub fields: IndexMap<String, ProtobufField>,
}

pub struct ProtobufEnumValue {
	pub name: String,
	pub description: Option<String>,
	pub tag: i32,
}

pub struct ProtobufMethod {
	pub name: String,
	pub description: Option<String>,
	pub input_type: String,
	pub output_type: String,
	pub client_streaming: bool,
	pub server_streaming: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProtobufTypeId {
	Scalar,
	Message,
	Enum,
	Service,
}

pub enum ProtobufKind {
	Scalar {
		name: String,
		description: Option<String>,
		is_valid: Option<ProtoScalarValidatorFn>,
		visible: Option<MetaVisibleFn>,
	},
	Message {
		name: String,
		description: Option<String>,
		fields: IndexMap<String, ProtobufField>,
		oneof_groups: Vec<ProtobufOneofGroup>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Enum {
		name: String,
		description: Option<String>,
		fields: IndexMap<String, ProtobufEnumValue>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Service {
		name: String,
		description: Option<String>,
		methods: IndexMap<String, ProtobufMethod>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
}

impl ProtobufKind {
	#[inline]
	pub fn type_id(&self) -> ProtobufTypeId {
		match self {
			ProtobufKind::Scalar {
				..
			} => ProtobufTypeId::Scalar,
			ProtobufKind::Message {
				..
			} => ProtobufTypeId::Message,
			ProtobufKind::Enum {
				..
			} => ProtobufTypeId::Enum,
			ProtobufKind::Service {
				..
			} => ProtobufTypeId::Service,
		}
	}

	#[inline]
	pub fn field_by_name(&self, name: &str) -> Option<&ProtobufField> {
		self.fields().and_then(|fields| fields.get(name))
	}

	// #[inline]
	// pub fn is_visible(&self, ctx: &Context<'_>) -> bool {
	// 	let visible = match self {
	// 		MetaType::Scalar {
	// 			visible,
	// 			..
	// 		} => visible,
	// 		MetaType::Object {
	// 			visible,
	// 			..
	// 		} => visible,
	// 		MetaType::Interface {
	// 			visible,
	// 			..
	// 		} => visible,
	// 		MetaType::Union {
	// 			visible,
	// 			..
	// 		} => visible,
	// 		MetaType::Enum {
	// 			visible,
	// 			..
	// 		} => visible,
	// 		MetaType::InputObject {
	// 			visible,
	// 			..
	// 		} => visible,
	// 	};
	// 	is_visible(ctx, visible)
	// }

	#[inline]
	pub fn name(&self) -> &str {
		match self {
			ProtobufKind::Scalar {
				name,
				..
			} => name,
			ProtobufKind::Message {
				name,
				..
			} => name,
			ProtobufKind::Enum {
				name,
				..
			} => name,
			ProtobufKind::Service {
				name,
				..
			} => name,
		}
	}

	#[inline]
	pub fn fields(&self) -> Option<&IndexMap<String, ProtobufField>> {
		match self {
			ProtobufKind::Message {
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
			ProtobufKind::Enum { .. } | ProtobufKind::Scalar { .. } | ProtobufKind::Message { .. }
		)
	}

	pub fn rust_typename(&self) -> Option<&'static str> {
		match self {
			ProtobufKind::Scalar {
				..
			} => None,
			ProtobufKind::Message {
				rust_typename,
				..
			} => *rust_typename,
			ProtobufKind::Enum {
				rust_typename,
				..
			} => *rust_typename,
			ProtobufKind::Service {
				rust_typename,
				..
			} => *rust_typename,
		}
	}
}

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
		is_valid: Option<ProtoScalarValidatorFn>,
		visible: Option<MetaVisibleFn>,
	},
	Struct {
		name: String,
		description: Option<String>,
		fields: IndexMap<String, ThriftField>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Union {
		name: String,
		description: Option<String>,
		fields: IndexMap<String, ThriftField>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Exception {
		name: String,
		description: Option<String>,
		fields: IndexMap<String, ThriftField>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Enum {
		name: String,
		description: Option<String>,
		fields: IndexMap<String, ThriftEnumValue>,
		visible: Option<MetaVisibleFn>,
		rust_typename: Option<&'static str>,
	},
	Service {
		name: String,
		description: Option<String>,
		methods: IndexMap<String, ThriftMethod>,
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
	pub fn fields(&self) -> Option<&IndexMap<String, ThriftField>> {
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

#[derive(Default)]
pub struct MetaType {
	pub proto: BTreeMap<String, ProtobufKind>,
	pub thrift: BTreeMap<String, ThriftKind>,
}

impl MetaType {
	/// Add an field to the object
	#[inline]
	pub fn proto(mut self, proto: ProtobufKind) -> Self {
		let name = proto.name().to_string();
		assert!(!self.proto.contains_key(&name), "ProtobufKind `{}` already exists", name);
		self.proto.insert(name, proto);
		self
	}

	#[inline]
	pub fn thrift(mut self, thrift: ThriftKind) -> Self {
		let name = thrift.name().to_string();
		assert!(!self.thrift.contains_key(&name), "ThriftKind `{}` already exists", name);
		self.thrift.insert(name, thrift);
		self
	}
}

/// A type registry for build schemas
pub struct Registry {
	pub types: MetaType,
	pub proto_type: String,
	// pub proto_enum_type: String,
	// pub mutation_type: Option<String>,
	// pub subscription_type: Option<String>,
	pub ignore_name_conflicts: HashSet<String>,
}

impl Registry {
	pub fn new(proto_type: String) -> Self {
		Self {
			types: MetaType::default(),
			proto_type,
			ignore_name_conflicts: HashSet::new(),
		}
	}

	pub fn build_proto(&self) -> String {
		let mut sdl = String::new();

		writeln!(sdl, "syntax = \"proto3\";").ok();

		self.types.proto.iter().for_each(|(_, kind)| match kind {
			ProtobufKind::Enum {
				name,
				description,
				fields,
				visible,
				rust_typename,
			} => {
				let line = fields
					.iter()
					.map(|(name, value)| format!("{} = {};", value.name, value.tag))
					.fold(String::new(), |start, data| format!("{}\n{}", start, data));

				writeln!(sdl, "enum {} {{{}}}", name, line).ok();
			}
			ProtobufKind::Message {
				name,
				description,
				fields,
				oneof_groups,
				visible,
				rust_typename,
			} => {
				let line = fields
					.iter()
					.map(|(name, field)| {
						format!("{} {} = {};", field.field_type, field.name, field.tag)
					})
					.fold(String::new(), |start, data| format!("{}\n{}", start, data));

				writeln!(sdl, "message {} {{{}}}", name, line).ok();
			}
			ProtobufKind::Scalar {
				name,
				description,
				is_valid,
				visible,
			} => {}
			ProtobufKind::Service {
				name,
				description,
				methods,
				visible,
				rust_typename,
			} => {
				let line = methods
					.iter()
					.map(|(name, method)| {
						format!(
							"rpc {}({}) returns ({}) {{}};",
							method.name, method.input_type, method.output_type
						)
					})
					.fold(String::new(), |start, data| format!("{}\n{}", start, data));

				writeln!(sdl, "service {} {{{}}}", name, line).ok();
			}
		});
		// println!("{}", sdl);
		sdl
	}
}
