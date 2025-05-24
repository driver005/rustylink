use super::{
	enums::EnumValue, fields::Field, input_fields::InputField, node_as_string, setup_output_dir,
	Enum, InputObject, Interface, NameString, Object, Scalar, TypeDef, Union, FILE_HEADER_COMMENT,
	PRESERVED_SCALARS, SUPPRESS_LINT,
};
use crate::{
	graphql::{Ignore, Phase, RendererConfig},
	OutputFile,
};
use anyhow::Result;
use async_graphql_parser::{types as async_gql_types, Positioned as AsyncGqlPositioned};
use std::collections::{HashMap, HashSet};

macro_rules! is_ignore_type {
	($typ:ident,
      $object_names_set:ident,
      $enum_names_set:ident,
      $input_object_names_set:ident,
      $scalar_names_set:ident,
      $union_names_set:ident,
      $inerface_names_set:ident) => {
		match $typ {
			TypeDef::Object(Object {
				name,
				..
			}) => $object_names_set.contains(&name),
			TypeDef::Enum(Enum {
				name,
				..
			}) => $enum_names_set.contains(&name),
			TypeDef::InputObject(InputObject {
				name,
				..
			}) => $input_object_names_set.contains(&name),
			TypeDef::Scalar(Scalar {
				name,
				..
			}) => $scalar_names_set.contains(&name),
			TypeDef::Union(Union {
				name,
				..
			}) => $union_names_set.contains(&name),
			TypeDef::Interface(Interface {
				name,
				..
			}) => $inerface_names_set.contains(&name),
			_ => false,
		}
	};
}

macro_rules! accumulate_remove_target {
	( $component:ident,
      $each_field:ident,
      $target_map:ident,
      $structured_schema:ident,
      $object_names_set:ident,
      $enum_names_set:ident,
      $input_object_names_set:ident,
      $scalar_names_set:ident,
      $union_names_set:ident,
      $inerface_names_set:ident
     ) => {{
		match $each_field.typ.element_value_type_def(&$structured_schema.definitions) {
			Err(e) => {
				println!("WARN: ignore unknown element. {:?}, error {}", $each_field.typ, e);
			}
			Ok(typ) => {
				let is_ignore_target = is_ignore_type!(
					typ,
					$object_names_set,
					$enum_names_set,
					$input_object_names_set,
					$scalar_names_set,
					$union_names_set,
					$inerface_names_set
				);

				if is_ignore_target {
					$target_map
						.entry($component.name.clone())
						.or_insert(HashSet::new())
						.insert($each_field.name.clone());
				}
			}
		}
	}};
}

#[derive(Debug, PartialEq)]
pub struct StructuredSchema {
	pub query_name: Option<String>,
	pub mutation_name: Option<String>,
	pub subscription_name: Option<String>,
	pub definitions: Definitions,
}

impl StructuredSchema {
	pub(crate) fn new(
		service_document: async_gql_types::ServiceDocument,
		config: &RendererConfig,
	) -> Result<Self> {
		let mut query_name: Option<String> = Some("Query".to_string());
		let mut mutation_name: Option<String> = Some("Mutation".to_string());
		let mut subscription_name: Option<String> = None;

		let mut definitions = Definitions::default();

		for each_node in service_document.definitions {
			match each_node {
				async_gql_types::TypeSystemDefinition::Schema(schema_def) => {
					query_name = schema_def.node.query.map(|query| node_as_string!(query));
					mutation_name =
						schema_def.node.mutation.map(|mutation| node_as_string!(mutation));

					subscription_name = schema_def
						.node
						.subscription
						.map(|subscription| node_as_string!(subscription));
				}

				async_gql_types::TypeSystemDefinition::Type(type_def) => {
					definitions.add_definition(Definition::new(type_def, config));
				}

				async_gql_types::TypeSystemDefinition::Directive(directive_def) => {
					log::warn!("directive not supported yet :{}", directive_def.node.name.node);
				}
			}
		}

		Ok(Self {
			query_name,
			mutation_name,
			subscription_name,
			definitions,
		})
	}

	pub(crate) fn is_query(&self, obj_name: &str) -> bool {
		match self.query_name.as_ref() {
			Some(query) => *query == *obj_name,
			None => false,
		}
	}

	pub(crate) fn is_mutation(&self, obj_name: &str) -> bool {
		match self.mutation_name.as_ref() {
			Some(mutation) => *mutation == *obj_name,
			None => false,
		}
	}

	pub(crate) fn remove_ignored(&mut self, config: &RendererConfig) -> Result<()> {
		match &config.ignore {
			None => Ok(()),
			Some(ignore) => {
				if ignore.is_empty() {
					Ok(())
				} else {
					self.inner_remove_ignored(&ignore)
				}
			}
		}
	}

	fn inner_remove_ignored(&mut self, ignore: &Ignore) -> Result<()> {
		let mut ignore_enums_set: HashSet<&String> = HashSet::new();
		let mut ignore_object_set: HashSet<&String> = HashSet::new();
		let mut ignore_input_object_set: HashSet<&String> = HashSet::new();
		let mut ignore_union_set: HashSet<&String> = HashSet::new();
		let mut ignore_interface_set: HashSet<&String> = HashSet::new();
		let mut ignore_scalar_set: HashSet<&String> = HashSet::new();

		if let Some(ignore_enums) = &ignore.r#enum {
			ignore_enums_set = ignore_enums.iter().collect();
		}

		if let Some(ignore_objects) = &ignore.object {
			ignore_object_set = ignore_objects.iter().collect();
		}

		if let Some(ignore_input_object) = &ignore.input_object {
			ignore_input_object_set = ignore_input_object.iter().collect();
		}

		if let Some(ignore_union) = &ignore.union {
			ignore_union_set = ignore_union.iter().collect();
		}

		if let Some(ignore_intrerface) = &ignore.interface {
			ignore_interface_set = ignore_intrerface.iter().collect();
		}

		if let Some(ignore_scalar) = &ignore.scalar {
			ignore_scalar_set = ignore_scalar.iter().collect();
		}

		// === accumlate remove object fields =====================================================================
		let mut remove_object_field_map: HashMap<String, HashSet<String>> = HashMap::new();
		let mut remove_object_field_argument_map: HashMap<
			String,
			HashMap<String, HashSet<String>>,
		> = HashMap::new();
		for object in self.definitions.objects.values() {
			for each_field in object.fields.iter() {
				accumulate_remove_target!(
					object,
					each_field,
					remove_object_field_map,
					self,
					ignore_object_set,
					ignore_enums_set,
					ignore_input_object_set,
					ignore_scalar_set,
					ignore_union_set,
					ignore_interface_set
				);

				for argument in each_field.arguments.iter() {
					match argument.typ.element_value_type_def(&self.definitions) {
						Err(e) => {
							println!(
								"WARN: could not ignore argument. element type {:?}, error {}",
								argument.typ, e
							);
						}
						Ok(typ) => {
							let is_ignore_target = is_ignore_type!(
								typ,
								ignore_object_set,
								ignore_enums_set,
								ignore_input_object_set,
								ignore_scalar_set,
								ignore_union_set,
								ignore_interface_set
							);

							if is_ignore_target {
								remove_object_field_argument_map
									.entry(object.name.clone())
									.or_insert(HashMap::new())
									.entry(each_field.name.clone())
									.or_insert(HashSet::new())
									.insert(argument.name.clone());
							}
						}
					}
				}
			}
		}

		// === accumulate remove input object fields =====================================================================
		let mut remove_input_object_field_map: HashMap<String, HashSet<String>> = HashMap::new();
		for input_object in self.definitions.input_objects.values() {
			for each_field in input_object.fields.iter() {
				accumulate_remove_target!(
					input_object,
					each_field,
					remove_input_object_field_map,
					self,
					ignore_object_set,
					ignore_enums_set,
					ignore_input_object_set,
					ignore_scalar_set,
					ignore_union_set,
					ignore_interface_set
				);
			}
		}

		// remove object and field

		for object in self.definitions.objects.values_mut() {
			remove_object_field_argument_map.get(&object.name).map(|remove_field_argument_map| {
				for each_field in object.fields.iter_mut() {
					let field_name = each_field.name.clone();
					if let Some(remove_arguments) = remove_field_argument_map.get(&field_name) {
						each_field
							.arguments
							.retain(|argument| !remove_arguments.contains(&argument.name));
					}
				}
			});

			remove_object_field_map.get(&object.name).map(|remove_fields| {
				object.fields.retain(|field| !remove_fields.contains(&field.name));
			});
		}

		// remove input object and field
		for input_object in self.definitions.input_objects.values_mut() {
			remove_input_object_field_map.get(&input_object.name).map(|remove_fields| {
				input_object.fields.retain(|field| !remove_fields.contains(&field.name));
			});
		}

		// remove from definitions
		self.definitions.enums.retain(|name, _| !ignore_enums_set.contains(name));

		self.definitions.objects.retain(|name, _| !ignore_object_set.contains(name));

		self.definitions.input_objects.retain(|name, _| !ignore_input_object_set.contains(name));

		self.definitions.unions.retain(|name, _| !ignore_union_set.contains(name));

		self.definitions.interfaces.retain(|name, _| !ignore_interface_set.contains(name));

		self.definitions.scalars.retain(|name, _| !ignore_scalar_set.contains(name));

		Ok(())
	}

	pub(crate) fn output(&self, config: RendererConfig, output_dir: &str) -> Result<()> {
		setup_output_dir(output_dir)?;

		let objects_written = if config.phases.is_empty() || config.phases.contains(&Phase::Objects)
		{
			Object::write(self, &config, output_dir)?
		} else {
			false
		};

		let input_objects_written =
			if config.phases.is_empty() || config.phases.contains(&Phase::InputObjects) {
				InputObject::write(self, &config, output_dir)?
			} else {
				false
			};

		let union_written = if config.phases.is_empty() || config.phases.contains(&Phase::Unions) {
			Union::write(self, output_dir)?
		} else {
			false
		};

		let scalar_written = if config.phases.is_empty() || config.phases.contains(&Phase::Scalars)
		{
			Scalar::write(self, output_dir)?
		} else {
			false
		};

		let interface_written =
			if config.phases.is_empty() || config.phases.contains(&Phase::Interfaces) {
				Interface::write(self, &config, output_dir)?
			} else {
				false
			};

		let enum_written = if config.phases.is_empty() || config.phases.contains(&Phase::Enums) {
			Enum::write(self, &config, output_dir)?
		} else {
			false
		};

		let log = ModInfo {
			objects_written,
			input_objects_written,
			union_written,
			scalar_written,
			interface_written,
			enum_written,
		};

		log.write(output_dir)?;

		Ok(())
	}
}

pub enum Definition {
	Scalar(Scalar),
	Object(Object),
	Interface(Interface),
	Union(Union),
	Enum(Enum),
	InputObject(InputObject),
}

impl Definition {
	fn new(
		type_def: AsyncGqlPositioned<async_gql_types::TypeDefinition>,
		config: &RendererConfig,
	) -> Self {
		let line_pos = type_def.pos.line;
		let type_def = type_def.node;

		let type_def_name = node_as_string!(type_def.name);
		let description = type_def.description.map(|desc| node_as_string!(desc));
		let resolver_settings = config.resolver_setting();
		let field_settings = config.field_setting();

		match type_def.kind {
			async_gql_types::TypeKind::Scalar => Self::Scalar(Scalar {
				name: type_def_name,
				line_pos,
			}),
			async_gql_types::TypeKind::Object(object_type) => {
				let fields_resolver_setting = resolver_settings.get(&type_def_name);
				let fields_setting = field_settings.get(&type_def_name);

				let fields = Field::new_from_list(
					&object_type.fields,
					fields_setting,
					fields_resolver_setting,
				);

				let object = Object {
					name: type_def_name,
					fields,
					description,
					line_pos,
					impl_interface_name: object_type
						.implements
						.into_iter()
						.map(|implement| node_as_string!(implement))
						.collect(),
				};

				Self::Object(object)
			}
			async_gql_types::TypeKind::Interface(interface) => {
				let fields_setting = field_settings.get(&type_def_name);
				let fields = Field::new_from_list(&interface.fields, fields_setting, None);

				let intf = Interface {
					name: type_def_name,
					//TODO(tacogips)concrete_type_names  always be empty?
					concrete_type_names: interface
						.implements
						.into_iter()
						.map(|i| node_as_string!(i))
						.collect(),
					fields,
					line_pos,
					description,
				};

				Self::Interface(intf)
			}
			async_gql_types::TypeKind::Union(union_type) => {
				let line_pos = union_type.members.first().map_or(0, |member| member.pos.line);

				let type_names =
					union_type.members.into_iter().map(|member| node_as_string!(member)).collect();

				let union = Union {
					name: type_def_name,
					type_names,
					line_pos,
					description,
				};

				Self::Union(union)
			}
			async_gql_types::TypeKind::Enum(enum_type) => {
				let enum_values =
					enum_type.values.iter().map(|enum_value| EnumValue::new(enum_value)).collect();

				let enum_def = Enum {
					name: type_def_name,
					values: enum_values,
					line_pos,
					description,
				};

				Self::Enum(enum_def)
			}
			async_gql_types::TypeKind::InputObject(input_type) => {
				let fields_setting = field_settings.get(&type_def_name);
				let input_fields = input_type
					.fields
					.iter()
					.map(|input_field| InputField::new(input_field, fields_setting))
					.collect();

				let input_object = InputObject {
					name: type_def_name,
					fields: input_fields,
					description,
					line_pos,
				};

				Self::InputObject(input_object)
			}
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Definitions {
	pub input_objects: HashMap<String, InputObject>,
	pub objects: HashMap<String, Object>,
	pub scalars: HashMap<String, Scalar>,
	pub unions: HashMap<String, Union>,
	pub enums: HashMap<String, Enum>,
	pub interfaces: HashMap<String, Interface>,
}

impl Definitions {
	fn add_definition(&mut self, definition: Definition) {
		match definition {
			Definition::Scalar(v) => {
				if !PRESERVED_SCALARS.contains(v.name.as_str()) {
					self.scalars.insert(v.name_string(), v);
				}
			}
			Definition::Object(v) => {
				self.objects.insert(v.name_string(), v);
			}
			Definition::Interface(v) => {
				self.interfaces.insert(v.name_string(), v);
			}
			Definition::Union(v) => {
				self.unions.insert(v.name_string(), v);
			}
			Definition::Enum(v) => {
				self.enums.insert(v.name_string(), v);
			}
			Definition::InputObject(v) => {
				self.input_objects.insert(v.name_string(), v);
			}
		}
	}
}

impl Default for Definitions {
	fn default() -> Self {
		Self {
			input_objects: HashMap::<String, InputObject>::new(),
			objects: HashMap::<String, Object>::new(),
			scalars: HashMap::<String, Scalar>::new(),
			unions: HashMap::<String, Union>::new(),
			enums: HashMap::<String, Enum>::new(),
			interfaces: HashMap::<String, Interface>::new(),
		}
	}
}

pub struct RenderContext<'a> {
	pub parent: TypeDef<'a>,
}

impl<'a> RenderContext<'a> {
	fn parent_name(&self) -> String {
		match self.parent {
			TypeDef::Object(obj) => format!("{}", obj.name_string()),
			TypeDef::Enum(obj) => format!("{}", obj.name_string()),
			TypeDef::InputObject(obj) => format!("{}", obj.name_string()),
			TypeDef::Union(obj) => format!("{}", obj.name_string()),
			TypeDef::Interface(obj) => format!("{}", obj.name_string()),
			_ => panic!("invalid parent : {:?}", self.parent),
		}
	}
}

struct ModInfo {
	objects_written: bool,
	input_objects_written: bool,
	union_written: bool,
	scalar_written: bool,
	interface_written: bool,
	enum_written: bool,
}

impl ModInfo {
	fn write(&self, output_dir: &str) -> Result<()> {
		let mut dest_file = OutputFile::new("mod.rs", SUPPRESS_LINT.to_string(), output_dir);

		dest_file.add_content(FILE_HEADER_COMMENT);

		if self.objects_written {
			dest_file.add_content("mod objects; pub use objects::*;");
		}

		if self.input_objects_written {
			dest_file.add_content("mod input_objects; pub use input_objects::*;");
		}

		if self.union_written {
			dest_file.add_content("mod unions; pub use unions::*;");
		}

		if self.scalar_written {
			dest_file.add_content("mod scalars; pub use scalars::*;");
		}

		if self.interface_written {
			dest_file.add_content("mod interfaces; pub use interfaces::*;");
		}

		if self.enum_written {
			dest_file.add_content("mod enums; pub use enums::*;");
		}

		dest_file.create()?;

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::graphql::{
		parse_schema,
		parser::typ::{ListValue, NamedValue, ValueTypeDef},
	};

	#[test]
	pub fn parse_schema_input_1() {
		let schema = r#"
        input SampleInput {
          id: String
          rec:[Int],
        }
        "#;
		let result = parse_schema(schema, &RendererConfig::default()).unwrap();
		let mut definitions = Definitions::default();
		definitions.input_objects.insert(
			"SampleInput".to_string(),
			InputObject {
				name: "SampleInput".to_string(),
				fields: vec![
					InputField {
						name: "id".to_string(),
						description: None,
						typ: ValueTypeDef::Named(NamedValue {
							value_type_name: "String".to_string(),
							is_nullable: true,
						}),
						line_pos: 3,
					},
					InputField {
						name: "rec".to_string(),
						description: None,
						typ: ValueTypeDef::List(ListValue {
							inner: Box::new(ValueTypeDef::Named(NamedValue {
								value_type_name: "Int".to_string(),
								is_nullable: true,
							})),
							is_nullable: true,
						}),
						line_pos: 4,
					},
				],
				description: None,
				line_pos: 2,
			},
		);

		let expected = StructuredSchema {
			query_name: None,
			mutation_name: None,
			subscription_name: None,
			definitions,
		};

		assert_eq!(result, expected);
	}
}
