use crate::{
	ActiveEnumFilterInputBuilder, BuilderContext, EntityObjectBuilder, TypesMapHelper,
	prepare_enumeration_condition,
};
use dynamic::prelude::*;
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait};
use std::collections::{BTreeMap, BTreeSet};

pub type FnFilterCondition =
	Box<dyn Fn(Condition, &ObjectAccessor) -> SeaResult<Condition> + Send + Sync>;

#[derive(Clone, Debug)]
pub struct FilterInfo {
	pub type_name: String,
	pub base_type: String,
	pub supported_operations: BTreeSet<FilterOperation>,
}

impl FilterInfo {
	/// used to convert a filter input info struct into input object
	pub fn generate_filter_input<Ty>(&self) -> Object<Ty>
	where
		Ty: TypeRefTrait,
	{
		self.supported_operations.iter().fold(
			Object::new(self.type_name.to_string(), IO::Input),
			|object, cur| {
				let field = match cur {
					FilterOperation::Equals => {
						Field::input("eq", Ty::named(self.base_type.clone()))
					}
					FilterOperation::NotEquals => {
						Field::input("ne", Ty::named(self.base_type.clone()))
					}
					FilterOperation::GreaterThan => {
						Field::input("gt", Ty::named(self.base_type.clone()))
					}
					FilterOperation::GreaterThanEquals => {
						Field::input("gte", Ty::named(self.base_type.clone()))
					}
					FilterOperation::LessThan => {
						Field::input("lt", Ty::named(self.base_type.clone()))
					}
					FilterOperation::LessThanEquals => {
						Field::input("lte", Ty::named(self.base_type.clone()))
					}
					FilterOperation::IsIn => {
						Field::input("is_in", Ty::named_nn_list(self.base_type.clone()))
					}
					FilterOperation::IsNotIn => {
						Field::input("is_not_in", Ty::named_nn_list(self.base_type.clone()))
					}
					FilterOperation::IsNull => {
						Field::input("is_null", Ty::named(self.base_type.clone()))
					}
					FilterOperation::IsNotNull => {
						Field::input("is_not_null", Ty::named(self.base_type.clone()))
					}
					FilterOperation::Contains => {
						Field::input("contains", Ty::named(self.base_type.clone()))
					}
					FilterOperation::StartsWith => {
						Field::input("starts_with", Ty::named(self.base_type.clone()))
					}
					FilterOperation::EndsWith => {
						Field::input("ends_with", Ty::named(self.base_type.clone()))
					}
					FilterOperation::Like => {
						Field::input("like", Ty::named(self.base_type.clone()))
					}
					FilterOperation::NotLike => {
						Field::input("not_like", Ty::named(self.base_type.clone()))
					}
					FilterOperation::Between => {
						Field::input("between", Ty::named_nn_list(self.base_type.clone()))
					}
					FilterOperation::NotBetween => {
						Field::input("not_between", Ty::named_nn_list(self.base_type.clone()))
					}
				};
				object.field(field)
			},
		)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterOperation {
	Equals,
	NotEquals,
	GreaterThan,
	GreaterThanEquals,
	LessThan,
	LessThanEquals,
	IsIn,
	IsNotIn,
	IsNull,
	IsNotNull,
	Contains,
	StartsWith,
	EndsWith,
	Like,
	NotLike,
	Between,
	NotBetween,
}

/// The configuration for FilterTypesMapHelper
pub struct GraphQLFilterTypes {
	// basic string filter
	pub string_filter_info: FilterInfo,
	// basic text filter
	pub text_filter_info: FilterInfo,
	// basic integer filter
	pub integer_filter_info: FilterInfo,
	// basic float filter
	pub float_filter_info: FilterInfo,
	// basic boolean filter
	pub boolean_filter_info: FilterInfo,
	// basic id filter
	pub id_filter_info: FilterInfo,
}

impl std::default::Default for GraphQLFilterTypes {
	fn default() -> Self {
		Self {
			string_filter_info: FilterInfo {
				type_name: "StringFilterInput".into(),
				base_type: GraphQLTypeRef::STRING.into(),
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
			text_filter_info: FilterInfo {
				type_name: "TextFilterInput".into(),
				base_type: GraphQLTypeRef::STRING.into(),
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
			integer_filter_info: FilterInfo {
				type_name: "IntegerFilterInput".into(),
				base_type: GraphQLTypeRef::INT.into(),
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
				base_type: GraphQLTypeRef::FLOAT.into(),
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
				base_type: GraphQLTypeRef::BOOLEAN.into(),
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
			id_filter_info: FilterInfo {
				type_name: "IdentityFilterInput".into(),
				base_type: GraphQLTypeRef::STRING.into(),
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
		}
	}
}

/// The configuration for FilterTypesMapHelper
pub struct ProtoFilterTypes {
	// basic int32 filter
	pub int32_filter_info: FilterInfo,
	// basic i64 filter
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

impl std::default::Default for ProtoFilterTypes {
	fn default() -> Self {
		Self {
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

/// The configuration for FilterTypesMapHelper
pub struct FilterTypesMapConfig {
	/// used to map entity_name.column_name to a custom filter type
	pub overwrites: BTreeMap<String, Option<FilterTypesMapWrapper>>,
	/// used to map entity_name.column_name to a custom condition function
	pub condition_functions: BTreeMap<String, FnFilterCondition>,

	pub proto: ProtoFilterTypes,
	pub graphql: GraphQLFilterTypes,
}

impl std::default::Default for FilterTypesMapConfig {
	fn default() -> Self {
		Self {
			overwrites: BTreeMap::default(),
			condition_functions: BTreeMap::default(),
			proto: ProtoFilterTypes::default(),
			graphql: GraphQLFilterTypes::default(),
		}
	}
}

pub struct FilterTypesMapWrapper {
	pub graphql: GraphQlFilterType,
	pub proto: ProtoFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphQlFilterType {
	Text,
	String,
	Integer,
	Float,
	Boolean,
	Id,
	Enumeration(String),
	Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtoFilterType {
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

pub trait FilterTypeTrait: Sized {
	fn get_name() -> String;
	fn to_filter_type(column_type: &ColumnType) -> Option<Self>;
	fn to_value(&self, context: &'static BuilderContext) -> String;
	fn prepare<T>(
		&self,
		condition: Condition,
		filter: &ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync;
	fn get_input_filters<Ty>(context: &'static BuilderContext) -> Vec<Object<Ty>>
	where
		Ty: TypeRefTrait;

	fn from_filter_type(map: FilterTypesMapWrapper) -> Option<Self>;
}

impl FilterTypeTrait for GraphQlFilterType {
	fn get_name() -> String {
		"GraphQlFilterType".to_owned()
	}
	fn to_filter_type(column_type: &ColumnType) -> Option<Self> {
		match column_type {
			ColumnType::Char(_) => Some(GraphQlFilterType::Text),
			ColumnType::String(_) => Some(GraphQlFilterType::String),
			ColumnType::Text => Some(GraphQlFilterType::String),
			ColumnType::TinyInteger => Some(GraphQlFilterType::Integer),
			ColumnType::SmallInteger => Some(GraphQlFilterType::Integer),
			ColumnType::Integer => Some(GraphQlFilterType::Integer),
			ColumnType::BigInteger => Some(GraphQlFilterType::Integer),
			ColumnType::TinyUnsigned => Some(GraphQlFilterType::Integer),
			ColumnType::SmallUnsigned => Some(GraphQlFilterType::Integer),
			ColumnType::Unsigned => Some(GraphQlFilterType::Integer),
			ColumnType::BigUnsigned => Some(GraphQlFilterType::Integer),
			ColumnType::Float => Some(GraphQlFilterType::Float),
			ColumnType::Double => Some(GraphQlFilterType::Float),
			ColumnType::Decimal(_) => Some(GraphQlFilterType::Text),
			ColumnType::DateTime => Some(GraphQlFilterType::Text),
			ColumnType::Timestamp => Some(GraphQlFilterType::Text),
			ColumnType::TimestampWithTimeZone => Some(GraphQlFilterType::Text),
			ColumnType::Time => Some(GraphQlFilterType::Text),
			ColumnType::Date => Some(GraphQlFilterType::Text),
			ColumnType::Year => Some(GraphQlFilterType::Integer),
			ColumnType::Interval(_, _) => Some(GraphQlFilterType::Text),
			ColumnType::Binary(_) => Some(GraphQlFilterType::String),
			ColumnType::VarBinary(_) => Some(GraphQlFilterType::String),
			ColumnType::Bit(_) => Some(GraphQlFilterType::String),
			ColumnType::VarBit(_) => Some(GraphQlFilterType::String),
			ColumnType::Blob => None,
			ColumnType::Boolean => Some(GraphQlFilterType::Boolean),
			ColumnType::Money(_) => Some(GraphQlFilterType::Text),
			ColumnType::Json => None,
			ColumnType::JsonBinary => None,
			ColumnType::Uuid => Some(GraphQlFilterType::Text),
			ColumnType::Custom(name) => Some(GraphQlFilterType::Custom(name.to_string())),
			ColumnType::Enum {
				name,
				variants: _,
			} => Some(GraphQlFilterType::Enumeration(name.to_string())),
			ColumnType::Array(_) => None,
			ColumnType::Cidr => Some(GraphQlFilterType::Text),
			ColumnType::Inet => Some(GraphQlFilterType::Text),
			ColumnType::MacAddr => Some(GraphQlFilterType::Text),
			_ => None,
		}
	}

	fn to_value(&self, context: &'static BuilderContext) -> String {
		match &self {
			GraphQlFilterType::Text => {
				context.filter_types.graphql.text_filter_info.type_name.clone()
			}
			GraphQlFilterType::String => {
				context.filter_types.graphql.string_filter_info.type_name.clone()
			}
			GraphQlFilterType::Integer => {
				context.filter_types.graphql.integer_filter_info.type_name.clone()
			}
			GraphQlFilterType::Float => {
				context.filter_types.graphql.float_filter_info.type_name.clone()
			}
			GraphQlFilterType::Boolean => {
				context.filter_types.graphql.boolean_filter_info.type_name.clone()
			}
			GraphQlFilterType::Id => context.filter_types.graphql.id_filter_info.type_name.clone(),
			GraphQlFilterType::Enumeration(name) => {
				let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
					context,
				};

				active_enum_filter_input_builder.type_name_from_string(&name)
			}
			GraphQlFilterType::Custom(type_name) => type_name.to_owned(),
		}
	}

	fn prepare<T>(
		&self,
		mut condition: Condition,
		filter: &ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let filter_info = match self {
			GraphQlFilterType::Text => &context.filter_types.graphql.text_filter_info,
			GraphQlFilterType::String => &context.filter_types.graphql.string_filter_info,
			GraphQlFilterType::Integer => &context.filter_types.graphql.integer_filter_info,
			GraphQlFilterType::Float => &context.filter_types.graphql.float_filter_info,
			GraphQlFilterType::Boolean => &context.filter_types.graphql.boolean_filter_info,
			GraphQlFilterType::Id => &context.filter_types.graphql.id_filter_info,
			GraphQlFilterType::Enumeration(_) => {
				return prepare_enumeration_condition::<T>(filter, column, condition);
			}
			GraphQlFilterType::Custom(_) => {
				let entity_object_builder = EntityObjectBuilder {
					context,
				};

				let entity_name = entity_object_builder.type_name::<T>();
				let column_name = entity_object_builder.column_name::<T>(column);

				if let Some(filter_condition_fn) = context
					.filter_types
					.condition_functions
					.get(&format!("{entity_name}.{column_name}"))
				{
					return filter_condition_fn(condition, filter);
				} else {
					// FIXME: add log warning to console
					return Ok(condition);
				}
			}
		};

		for operation in filter_info.supported_operations.iter() {
			match operation {
				FilterOperation::Equals => {
					if let Some(value) = filter.get("eq") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.eq(value));
					}
				}
				FilterOperation::NotEquals => {
					if let Some(value) = filter.get("ne") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.ne(value));
					}
				}
				FilterOperation::GreaterThan => {
					if let Some(value) = filter.get("gt") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.gt(value));
					}
				}
				FilterOperation::GreaterThanEquals => {
					if let Some(value) = filter.get("gte") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.gte(value));
					}
				}
				FilterOperation::LessThan => {
					if let Some(value) = filter.get("lt") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.lt(value));
					}
				}
				FilterOperation::LessThanEquals => {
					if let Some(value) = filter.get("lte") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.lte(value));
					}
				}
				FilterOperation::IsIn => {
					if let Some(value) = filter.get("is_in") {
						let value = value
							.list()?
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
							.collect::<SeaResult<Vec<_>>>()?;
						condition = condition.add(column.is_in(value));
					}
				}
				FilterOperation::IsNotIn => {
					if let Some(value) = filter.get("is_not_in") {
						let value = value
							.list()?
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
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
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.contains(value.to_string()));
					}
				}
				FilterOperation::StartsWith => {
					if let Some(value) = filter.get("starts_with") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.starts_with(value.to_string()));
					}
				}
				FilterOperation::EndsWith => {
					if let Some(value) = filter.get("ends_with") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.ends_with(value.to_string()));
					}
				}
				FilterOperation::Like => {
					if let Some(value) = filter.get("like") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.like(value.to_string()));
					}
				}
				FilterOperation::NotLike => {
					if let Some(value) = filter.get("not_like") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.not_like(value.to_string()));
					}
				}
				FilterOperation::Between => {
					if let Some(value) = filter.get("between") {
						let value = value
							.list()?
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
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
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
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

	fn get_input_filters<Ty>(context: &'static BuilderContext) -> Vec<Object<Ty>>
	where
		Ty: TypeRefTrait,
	{
		vec![
			context.filter_types.graphql.text_filter_info.generate_filter_input(),
			context.filter_types.graphql.string_filter_info.generate_filter_input(),
			context.filter_types.graphql.integer_filter_info.generate_filter_input(),
			context.filter_types.graphql.float_filter_info.generate_filter_input(),
			context.filter_types.graphql.boolean_filter_info.generate_filter_input(),
			context.filter_types.graphql.id_filter_info.generate_filter_input(),
		]
	}

	fn from_filter_type(map: FilterTypesMapWrapper) -> Option<Self> {
		Some(map.graphql)
	}
}

impl FilterTypeTrait for ProtoFilterType {
	fn get_name() -> String {
		"ProtoFilterType".to_owned()
	}
	fn to_filter_type(column_type: &ColumnType) -> Option<Self> {
		match column_type {
			ColumnType::Char(_) => Some(ProtoFilterType::String),
			ColumnType::String(_) => Some(ProtoFilterType::String),
			ColumnType::Text => Some(ProtoFilterType::String),
			ColumnType::TinyInteger => Some(ProtoFilterType::Int32),
			ColumnType::SmallInteger => Some(ProtoFilterType::Int32),
			ColumnType::Integer => Some(ProtoFilterType::Int32),
			ColumnType::BigInteger => Some(ProtoFilterType::Int64),
			ColumnType::TinyUnsigned => Some(ProtoFilterType::UInt32),
			ColumnType::SmallUnsigned => Some(ProtoFilterType::UInt32),
			ColumnType::Unsigned => Some(ProtoFilterType::UInt32),
			ColumnType::BigUnsigned => Some(ProtoFilterType::UInt64),
			ColumnType::Float => Some(ProtoFilterType::Float),
			ColumnType::Double => Some(ProtoFilterType::Float),
			ColumnType::Decimal(_) => Some(ProtoFilterType::String),
			ColumnType::DateTime => Some(ProtoFilterType::String),
			ColumnType::Timestamp => Some(ProtoFilterType::String),
			ColumnType::TimestampWithTimeZone => Some(ProtoFilterType::String),
			ColumnType::Time => Some(ProtoFilterType::String),
			ColumnType::Date => Some(ProtoFilterType::String),
			ColumnType::Year => Some(ProtoFilterType::Int32),
			ColumnType::Interval(_, _) => Some(ProtoFilterType::String),
			ColumnType::Binary(_) => Some(ProtoFilterType::Binary),
			ColumnType::VarBinary(_) => Some(ProtoFilterType::Binary),
			ColumnType::Bit(_) => Some(ProtoFilterType::Binary),
			ColumnType::VarBit(_) => Some(ProtoFilterType::Binary),
			ColumnType::Blob => None,
			ColumnType::Boolean => Some(ProtoFilterType::Boolean),
			ColumnType::Money(_) => Some(ProtoFilterType::String),
			ColumnType::Json => None,
			ColumnType::JsonBinary => None,
			ColumnType::Uuid => Some(ProtoFilterType::String),
			ColumnType::Custom(name) => Some(ProtoFilterType::Custom(name.to_string())),
			ColumnType::Enum {
				name,
				variants: _,
			} => Some(ProtoFilterType::Enumeration(name.to_string())),
			ColumnType::Array(_) => None,
			ColumnType::Cidr => Some(ProtoFilterType::String),
			ColumnType::Inet => Some(ProtoFilterType::String),
			ColumnType::MacAddr => Some(ProtoFilterType::String),
			_ => None,
		}
	}

	fn to_value(&self, context: &'static BuilderContext) -> String {
		match &self {
			ProtoFilterType::Int32 => {
				context.filter_types.proto.int32_filter_info.type_name.clone()
			}
			ProtoFilterType::Int64 => {
				context.filter_types.proto.int64_filter_info.type_name.clone()
			}
			ProtoFilterType::UInt32 => {
				context.filter_types.proto.uint32_filter_info.type_name.clone()
			}
			ProtoFilterType::UInt64 => {
				context.filter_types.proto.uint64_filter_info.type_name.clone()
			}
			ProtoFilterType::SInt32 => {
				context.filter_types.proto.sint32_filter_info.type_name.clone()
			}
			ProtoFilterType::SInt64 => {
				context.filter_types.proto.sint64_filter_info.type_name.clone()
			}
			ProtoFilterType::Float => {
				context.filter_types.proto.float_filter_info.type_name.clone()
			}
			ProtoFilterType::Double => {
				context.filter_types.proto.double_filter_info.type_name.clone()
			}
			ProtoFilterType::Boolean => {
				context.filter_types.proto.boolean_filter_info.type_name.clone()
			}
			ProtoFilterType::String => {
				context.filter_types.proto.string_filter_info.type_name.clone()
			}
			ProtoFilterType::Binary => {
				context.filter_types.proto.binary_filter_info.type_name.clone()
			}
			ProtoFilterType::Enumeration(name) => {
				let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
					context,
				};

				active_enum_filter_input_builder.type_name_from_string(&name)
			}
			ProtoFilterType::Custom(type_name) => type_name.to_owned(),
		}
	}

	fn prepare<T>(
		&self,
		mut condition: Condition,
		filter: &ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let filter_info = match self {
			ProtoFilterType::Int32 => &context.filter_types.proto.int32_filter_info,
			ProtoFilterType::Int64 => &context.filter_types.proto.int64_filter_info,
			ProtoFilterType::UInt32 => &context.filter_types.proto.uint32_filter_info,
			ProtoFilterType::UInt64 => &context.filter_types.proto.uint64_filter_info,
			ProtoFilterType::SInt32 => &context.filter_types.proto.sint32_filter_info,
			ProtoFilterType::SInt64 => &context.filter_types.proto.sint64_filter_info,
			ProtoFilterType::Float => &context.filter_types.proto.float_filter_info,
			ProtoFilterType::Double => &context.filter_types.proto.double_filter_info,
			ProtoFilterType::Boolean => &context.filter_types.proto.boolean_filter_info,
			ProtoFilterType::String => &context.filter_types.proto.string_filter_info,
			ProtoFilterType::Binary => &context.filter_types.proto.binary_filter_info,
			ProtoFilterType::Enumeration(_) => {
				return prepare_enumeration_condition::<T>(filter, column, condition);
			}
			ProtoFilterType::Custom(_) => {
				let entity_object_builder = EntityObjectBuilder {
					context,
				};

				let entity_name = entity_object_builder.type_name::<T>();
				let column_name = entity_object_builder.column_name::<T>(column);

				if let Some(filter_condition_fn) = context
					.filter_types
					.condition_functions
					.get(&format!("{entity_name}.{column_name}"))
				{
					return filter_condition_fn(condition, filter);
				} else {
					// FIXME: add log warning to console
					return Ok(condition);
				}
			}
		};

		for operation in filter_info.supported_operations.iter() {
			match operation {
				FilterOperation::Equals => {
					if let Some(value) = filter.get("eq") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.eq(value));
					}
				}
				FilterOperation::NotEquals => {
					if let Some(value) = filter.get("ne") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.ne(value));
					}
				}
				FilterOperation::GreaterThan => {
					if let Some(value) = filter.get("gt") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.gt(value));
					}
				}
				FilterOperation::GreaterThanEquals => {
					if let Some(value) = filter.get("gte") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.gte(value));
					}
				}
				FilterOperation::LessThan => {
					if let Some(value) = filter.get("lt") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.lt(value));
					}
				}
				FilterOperation::LessThanEquals => {
					if let Some(value) = filter.get("lte") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.lte(value));
					}
				}
				FilterOperation::IsIn => {
					if let Some(value) = filter.get("is_in") {
						let value = value
							.list()?
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
							.collect::<SeaResult<Vec<_>>>()?;
						condition = condition.add(column.is_in(value));
					}
				}
				FilterOperation::IsNotIn => {
					if let Some(value) = filter.get("is_not_in") {
						let value = value
							.list()?
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
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
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.contains(value.to_string()));
					}
				}
				FilterOperation::StartsWith => {
					if let Some(value) = filter.get("starts_with") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.starts_with(value.to_string()));
					}
				}
				FilterOperation::EndsWith => {
					if let Some(value) = filter.get("ends_with") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.ends_with(value.to_string()));
					}
				}
				FilterOperation::Like => {
					if let Some(value) = filter.get("like") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.like(value.to_string()));
					}
				}
				FilterOperation::NotLike => {
					if let Some(value) = filter.get("not_like") {
						let value = types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
						condition = condition.add(column.not_like(value.to_string()));
					}
				}
				FilterOperation::Between => {
					if let Some(value) = filter.get("between") {
						let value = value
							.list()?
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
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
							.to_iter()
							.map(|v| types_map_helper.value_to_sea_orm_value::<T>(column, &v))
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

	fn get_input_filters<Ty>(context: &'static BuilderContext) -> Vec<Object<Ty>>
	where
		Ty: TypeRefTrait,
	{
		vec![
			context.filter_types.proto.int32_filter_info.generate_filter_input(),
			context.filter_types.proto.int64_filter_info.generate_filter_input(),
			context.filter_types.proto.uint32_filter_info.generate_filter_input(),
			context.filter_types.proto.uint64_filter_info.generate_filter_input(),
			context.filter_types.proto.sint32_filter_info.generate_filter_input(),
			context.filter_types.proto.sint64_filter_info.generate_filter_input(),
			context.filter_types.proto.float_filter_info.generate_filter_input(),
			context.filter_types.proto.double_filter_info.generate_filter_input(),
			context.filter_types.proto.boolean_filter_info.generate_filter_input(),
			context.filter_types.proto.string_filter_info.generate_filter_input(),
			context.filter_types.proto.binary_filter_info.generate_filter_input(),
		]
	}
	fn from_filter_type(map: FilterTypesMapWrapper) -> Option<Self> {
		Some(map.proto)
	}
}

/// The configuration for FilterType
pub struct FilterTypesMapHelper {
	pub context: &'static BuilderContext,
}

impl FilterTypesMapHelper {
	pub fn get_column_filter_type<T, F>(&self, column: &T::Column) -> Option<F>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		F: FilterTypeTrait,
	{
		//TODO: implment overwrites
		// let entity_object_builder = EntityObjectBuilder {
		// 	context: self.context,
		// };

		// let entity_name = entity_object_builder.type_name::<T>();
		// let column_name = entity_object_builder.column_name::<T>(column);

		// used to honor overwrites
		// if let Some(ty) =
		// 	self.context.filter_types.overwrites.get(&format!("{entity_name}.{column_name}"))
		// {
		// 	if let Some(ty) = ty {

		// 	}
		// }

		// default mappings
		self.to_filter_type(column.def().get_column_type())
	}

	/// used to get the GraphQL input value field for a SeaORM entity column
	pub fn get_column_filter_input_value<T, Ty, F>(&self, column: &T::Column) -> Option<Field<Ty>>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		Ty: TypeRefTrait,
		F: FilterTypeTrait,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let column_name = entity_object_builder.column_name::<T>(column);

		let filter_type = self.get_column_filter_type::<T, F>(column);

		match filter_type {
			Some(filter_type) => Some(self.to_value(&column_name, &filter_type, self.context)),
			None => None,
		}
	}

	/// used to parse a filter input object and update the query condition
	pub fn prepare_column_condition<T, F>(
		&self,
		condition: Condition,
		filter: &ObjectAccessor,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		F: FilterTypeTrait,
	{
		let types_map_helper = TypesMapHelper {
			context: self.context,
		};

		self.prepare::<T, F>(
			&self.get_column_filter_type::<T, F>(column),
			condition,
			filter,
			&types_map_helper,
			self.context,
			column,
		)
	}

	pub fn to_filter_type<F>(&self, column_type: &ColumnType) -> Option<F>
	where
		F: FilterTypeTrait,
	{
		F::to_filter_type(column_type)
	}

	pub fn prepare<T, F>(
		&self,
		filter_type: &Option<F>,
		condition: Condition,
		filter: &ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		F: FilterTypeTrait,
	{
		match filter_type {
			Some(filter_type) => {
				filter_type.prepare::<T>(condition, filter, types_map_helper, context, column)
			}
			None => Ok(condition),
		}
	}

	pub fn to_value<Ty, F>(
		&self,
		column_name: &String,
		filter_type: &F,
		context: &'static BuilderContext,
	) -> Field<Ty>
	where
		Ty: TypeRefTrait,
		F: FilterTypeTrait,
	{
		Field::input(column_name, Ty::named(filter_type.to_value(context)))
	}
}
