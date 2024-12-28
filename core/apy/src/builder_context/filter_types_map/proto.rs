use super::{FilterInfo, FilterOperation};
use crate::{
	prepare_enumeration_condition, ActiveEnumFilterInputBuilder, BuilderContext,
	EntityObjectBuilder, SeaResult, TypesMapHelper,
};
use dynamic::prelude::{
	ProtoField, ProtoMessage, ProtoObjectAccessor, ProtoTypeRef, ProtoValueAccessor,
};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait};
use std::{
	collections::{BTreeMap, BTreeSet},
	ops::Add,
};

pub type FnFilterCondition =
	Box<dyn Fn(Condition, &ProtoObjectAccessor) -> SeaResult<Condition> + Send + Sync>;

/// The configuration for FilterTypesMapHelper
pub struct FilterTypesMapConfig {
	/// used to map entity_name.column_name to a custom filter type
	pub overwrites: BTreeMap<String, Option<FilterType>>,
	/// used to map entity_name.column_name to a custom condition function
	pub condition_functions: BTreeMap<String, FnFilterCondition>,
	// basic int32 filter
	pub int32_filter_info: FilterInfo,
	// basic int64 filter
	pub int64_filter_info: FilterInfo,
	// basic uint32 filter
	pub uint32_filter_info: FilterInfo,
	// basic uint64 filter
	pub uint64_filter_info: FilterInfo,
	// basic sint32 filter
	pub sint32_filter_info: FilterInfo,
	// basic sint64 filter
	pub sint64_filter_info: FilterInfo,
	// basic float filter
	pub float_filter_info: FilterInfo,
	// basic double filter
	pub double_filter_info: FilterInfo,
	// basic boolean filter
	pub boolean_filter_info: FilterInfo,
	// basic string filter
	pub string_filter_info: FilterInfo,
	// basic binary filter
	pub binary_filter_info: FilterInfo,
}

impl std::default::Default for FilterTypesMapConfig {
	fn default() -> Self {
		Self {
			overwrites: BTreeMap::default(),
			condition_functions: BTreeMap::default(),
			int32_filter_info: FilterInfo {
				type_name: "Int32FilterInput".into(),
				base_type: ProtoTypeRef::INT32.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			int64_filter_info: FilterInfo {
				type_name: "Int64FilterInput".into(),
				base_type: ProtoTypeRef::INT64.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			uint32_filter_info: FilterInfo {
				type_name: "Uint32FilterInput".into(),
				base_type: ProtoTypeRef::UINT32.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			uint64_filter_info: FilterInfo {
				type_name: "Uint64FilterInput".into(),
				base_type: ProtoTypeRef::UINT64.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			sint32_filter_info: FilterInfo {
				type_name: "Sint32FilterInput".into(),
				base_type: ProtoTypeRef::SINT32.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			sint64_filter_info: FilterInfo {
				type_name: "Sint64FilterInput".into(),
				base_type: ProtoTypeRef::SINT64.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			float_filter_info: FilterInfo {
				type_name: "FloatFilterInput".into(),
				base_type: ProtoTypeRef::FLOAT.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			double_filter_info: FilterInfo {
				type_name: "DoubleFilterInput".into(),
				base_type: ProtoTypeRef::DOUBLE.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			boolean_filter_info: FilterInfo {
				type_name: "BooleanFilterInput".into(),
				base_type: ProtoTypeRef::BOOL.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
				]),
			},
			string_filter_info: FilterInfo {
				type_name: "StringFilterInput".into(),
				base_type: ProtoTypeRef::STRING.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Contains,
					FilterOperation::StartsWith,
					FilterOperation::EndsWith,
					FilterOperation::Like,
					FilterOperation::NotLike,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
			binary_filter_info: FilterInfo {
				type_name: "BinaryFilterInput".into(),
				base_type: ProtoTypeRef::BYTES.into(),
				supported_operations: BTreeSet::from([
					FilterOperation::Equals,
					FilterOperation::NotEquals,
					FilterOperation::GreaterThan,
					FilterOperation::GreaterThanEquals,
					FilterOperation::LessThan,
					FilterOperation::LessThanEquals,
					FilterOperation::IsIn,
					FilterOperation::IsNotIn,
					FilterOperation::IsNull,
					FilterOperation::IsNotNull,
					FilterOperation::Contains,
					FilterOperation::StartsWith,
					FilterOperation::EndsWith,
					FilterOperation::Like,
					FilterOperation::NotLike,
					FilterOperation::Between,
					FilterOperation::NotBetween,
				]),
			},
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterType {
	Int32,
	Int64,
	UInt32,
	UInt64,
	SInt32,
	SInt64,
	Float,
	Double,
	Boolean,
	String,
	Binary,
	Enumeration(String),
	Custom(String),
}

/// The configuration for FilterType
pub struct FilterTypesMapHelper {
	pub context: &'static BuilderContext,
}

impl FilterTypesMapHelper {
	pub fn to_filter_type(&self, column_type: &ColumnType) -> Option<FilterType> {
		match column_type {
			ColumnType::Char(_) => Some(FilterType::String),
			ColumnType::String(_) => Some(FilterType::String),
			ColumnType::Text => Some(FilterType::String),
			ColumnType::TinyInteger => Some(FilterType::Int32),
			ColumnType::SmallInteger => Some(FilterType::Int32),
			ColumnType::Integer => Some(FilterType::Int32),
			ColumnType::BigInteger => Some(FilterType::Int64),
			ColumnType::TinyUnsigned => Some(FilterType::UInt32),
			ColumnType::SmallUnsigned => Some(FilterType::UInt32),
			ColumnType::Unsigned => Some(FilterType::UInt32),
			ColumnType::BigUnsigned => Some(FilterType::UInt64),
			ColumnType::Float => Some(FilterType::Float),
			ColumnType::Double => Some(FilterType::Float),
			ColumnType::Decimal(_) => Some(FilterType::String),
			ColumnType::DateTime => Some(FilterType::String),
			ColumnType::Timestamp => Some(FilterType::String),
			ColumnType::TimestampWithTimeZone => Some(FilterType::String),
			ColumnType::Time => Some(FilterType::String),
			ColumnType::Date => Some(FilterType::String),
			ColumnType::Year => Some(FilterType::Int32),
			ColumnType::Interval(_, _) => Some(FilterType::String),
			ColumnType::Binary(_) => Some(FilterType::Binary),
			ColumnType::VarBinary(_) => Some(FilterType::Binary),
			ColumnType::Bit(_) => Some(FilterType::Binary),
			ColumnType::VarBit(_) => Some(FilterType::Binary),
			ColumnType::Blob => None,
			ColumnType::Boolean => Some(FilterType::Boolean),
			ColumnType::Money(_) => Some(FilterType::String),
			ColumnType::Json => None,
			ColumnType::JsonBinary => None,
			ColumnType::Uuid => Some(FilterType::String),
			ColumnType::Custom(name) => Some(FilterType::Custom(name.to_string())),
			ColumnType::Enum {
				name,
				variants: _,
			} => Some(FilterType::Enumeration(name.to_string())),
			ColumnType::Array(_) => None,
			ColumnType::Cidr => Some(FilterType::String),
			ColumnType::Inet => Some(FilterType::String),
			ColumnType::MacAddr => Some(FilterType::String),
			_ => None,
		}
	}

	pub fn prepare<T>(
		&self,
		filter_type: &Option<FilterType>,
		mut condition: Condition,
		filter: &ProtoObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let filter_info = match filter_type {
			Some(filter_type) => match filter_type {
				FilterType::Int32 => &self.context.filter_types_proto.int32_filter_info,
				FilterType::Int64 => &self.context.filter_types_proto.int64_filter_info,
				FilterType::UInt32 => &self.context.filter_types_proto.uint32_filter_info,
				FilterType::UInt64 => &self.context.filter_types_proto.uint64_filter_info,
				FilterType::SInt32 => &self.context.filter_types_proto.sint32_filter_info,
				FilterType::SInt64 => &self.context.filter_types_proto.sint64_filter_info,
				FilterType::Float => &self.context.filter_types_proto.float_filter_info,
				FilterType::Double => &self.context.filter_types_proto.double_filter_info,
				FilterType::Boolean => &self.context.filter_types_proto.boolean_filter_info,
				FilterType::String => &self.context.filter_types_proto.string_filter_info,
				FilterType::Binary => &self.context.filter_types_proto.binary_filter_info,
				FilterType::Enumeration(_) => {
					return prepare_enumeration_condition::<T, ProtoObjectAccessor>(
						filter, column, condition,
					)
				}
				FilterType::Custom(_) => {
					let entity_object_builder = EntityObjectBuilder {
						context,
					};

					let entity_name = entity_object_builder.type_name::<T>();
					let column_name = entity_object_builder.column_name::<T>(column);

					if let Some(filter_condition_fn) = context
						.filter_types_proto
						.condition_functions
						.get(&format!("{entity_name}.{column_name}"))
					{
						return filter_condition_fn(condition, filter);
					} else {
						// FIXME: add log warning to console
						return Ok(condition);
					}
				}
			},
			None => return Ok(condition),
		};

		for operation in filter_info.supported_operations.iter() {
			match operation {
				FilterOperation::Equals => {
					if let Some(value) = filter.get("eq") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.eq(value));
					}
				}
				FilterOperation::NotEquals => {
					if let Some(value) = filter.get("ne") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.ne(value));
					}
				}
				FilterOperation::GreaterThan => {
					if let Some(value) = filter.get("gt") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.gt(value));
					}
				}
				FilterOperation::GreaterThanEquals => {
					if let Some(value) = filter.get("gte") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.gte(value));
					}
				}
				FilterOperation::LessThan => {
					if let Some(value) = filter.get("lt") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.lt(value));
					}
				}
				FilterOperation::LessThanEquals => {
					if let Some(value) = filter.get("lte") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.lte(value));
					}
				}
				FilterOperation::IsIn => {
					if let Some(value) = filter.get("is_in") {
						let value = value
							.list()?
							.iter()
							.map(|v| {
								types_map_helper
									.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &v)
							})
							.collect::<SeaResult<Vec<_>>>()?;
						condition = condition.add(column.is_in(value));
					}
				}
				FilterOperation::IsNotIn => {
					if let Some(value) = filter.get("is_not_in") {
						let value = value
							.list()?
							.iter()
							.map(|v| {
								types_map_helper
									.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &v)
							})
							.collect::<SeaResult<Vec<_>>>()?;
						condition = condition.add(column.is_not_in(value));
					}
				}
				FilterOperation::IsNull => {
					if filter.get("is_null").is_some() {
						condition = condition.add(column.is_null());
					}
				}
				FilterOperation::IsNotNull => {
					if filter.get("is_not_null").is_some() {
						condition = condition.add(column.is_not_null());
					}
				}
				FilterOperation::Contains => {
					if let Some(value) = filter.get("contains") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.contains(value.to_string()));
					}
				}
				FilterOperation::StartsWith => {
					if let Some(value) = filter.get("starts_with") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.starts_with(value.to_string()));
					}
				}
				FilterOperation::EndsWith => {
					if let Some(value) = filter.get("ends_with") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.ends_with(value.to_string()));
					}
				}
				FilterOperation::Like => {
					if let Some(value) = filter.get("like") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.like(value.to_string()));
					}
				}
				FilterOperation::NotLike => {
					if let Some(value) = filter.get("not_like") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &value)?;
						condition = condition.add(column.not_like(value.to_string()));
					}
				}
				FilterOperation::Between => {
					if let Some(value) = filter.get("between") {
						let value = value
							.list()?
							.iter()
							.map(|v| {
								types_map_helper
									.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &v)
							})
							.collect::<SeaResult<Vec<_>>>()?;

						let a = value[0].clone();
						let b = value[1].clone();

						condition = condition.add(column.between(a, b));
					}
				}
				FilterOperation::NotBetween => {
					if let Some(value) = filter.get("not_between") {
						let value = value
							.list()?
							.iter()
							.map(|v| {
								types_map_helper
									.value_to_sea_orm_value::<T, ProtoValueAccessor>(column, &v)
							})
							.collect::<SeaResult<Vec<_>>>()?;

						let a = value[0].clone();
						let b = value[1].clone();

						condition = condition.add(column.not_between(a, b));
					}
				}
			}
		}

		Ok(condition)
	}

	/// used to get all basic input filter objects
	pub fn get_input_filters(&self) -> Vec<ProtoMessage> {
		vec![
			self.generate_filter_input(&self.context.filter_types_proto.int32_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.int64_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.uint32_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.uint64_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.sint32_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.sint64_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.float_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.double_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.boolean_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.string_filter_info),
			self.generate_filter_input(&self.context.filter_types_proto.binary_filter_info),
		]
	}

	/// used to convert a filter input info struct into input object
	pub fn generate_filter_input(&self, filter_info: &FilterInfo) -> ProtoMessage {
		filter_info.supported_operations.iter().enumerate().fold(
			ProtoMessage::new(filter_info.type_name.to_string()),
			|object, (index, cur)| {
				let tag = index.add(1) as u32;
				let field = match cur {
					FilterOperation::Equals => ProtoField::input(
						"eq",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::NotEquals => ProtoField::input(
						"ne",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::GreaterThan => ProtoField::input(
						"gt",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::GreaterThanEquals => ProtoField::input(
						"gte",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::LessThan => ProtoField::input(
						"lt",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::LessThanEquals => ProtoField::input(
						"lte",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::IsIn => ProtoField::input(
						"is_in",
						tag,
						ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
					FilterOperation::IsNotIn => ProtoField::input(
						"is_not_in",
						tag,
						ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
					FilterOperation::IsNull => ProtoField::input(
						"is_null",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::IsNotNull => ProtoField::input(
						"is_not_null",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::Contains => ProtoField::input(
						"contains",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::StartsWith => ProtoField::input(
						"starts_with",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::EndsWith => ProtoField::input(
						"ends_with",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::Like => ProtoField::input(
						"like",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::NotLike => ProtoField::input(
						"not_like",
						tag,
						ProtoTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::Between => ProtoField::input(
						"between",
						tag,
						ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
					FilterOperation::NotBetween => ProtoField::input(
						"not_between",
						tag,
						ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
				};
				object.field(field)
			},
		)
	}

	pub fn to_value(
		&self,
		column_name: &String,
		tag: u32,
		filter_type: &FilterType,
		context: &'static BuilderContext,
	) -> ProtoField {
		match filter_type {
			FilterType::Int32 => {
				let info = &self.context.filter_types_proto.int32_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::Int64 => {
				let info = &self.context.filter_types_proto.int64_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::UInt32 => {
				let info = &self.context.filter_types_proto.uint32_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::UInt64 => {
				let info = &self.context.filter_types_proto.uint64_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::SInt32 => {
				let info = &self.context.filter_types_proto.sint32_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::SInt64 => {
				let info = &self.context.filter_types_proto.sint64_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::Float => {
				let info = &self.context.filter_types_proto.float_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::Double => {
				let info = &self.context.filter_types_proto.double_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::Boolean => {
				let info = &self.context.filter_types_proto.boolean_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::String => {
				let info = &self.context.filter_types_proto.string_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::Binary => {
				let info = &self.context.filter_types_proto.binary_filter_info;
				ProtoField::input(column_name, tag, ProtoTypeRef::named(info.type_name.clone()))
			}
			FilterType::Enumeration(name) => {
				let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
					context,
				};

				ProtoField::input(
					column_name,
					tag,
					ProtoTypeRef::named(
						active_enum_filter_input_builder.type_name_from_string(&name),
					),
				)
			}
			FilterType::Custom(type_name) => {
				ProtoField::input(column_name, tag, ProtoTypeRef::named(type_name))
			}
		}
	}
}
