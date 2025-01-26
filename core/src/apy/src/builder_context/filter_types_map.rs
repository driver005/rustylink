use crate::{
	prepare_enumeration_condition, ActiveEnumFilterInputBuilder, BuilderContext,
	EntityObjectBuilder, SeaResult, TypesMapHelper,
};
use dynamic::{
	prelude::{
		Field, GraphQLTypeRef, Object, ObjectAccessorTrait, ObjectAccessors, ProtoTypeRef, TypeRef,
		TypeRefTrait, ValueAccessorTrait,
	},
	ListAccessorTrait,
};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait};
use std::{
	collections::{BTreeMap, BTreeSet},
	ops::Add,
};

pub type FnFilterCondition =
	Box<dyn Fn(Condition, &ObjectAccessors) -> SeaResult<Condition> + Send + Sync>;

#[derive(Clone, Debug)]
pub struct FilterInfo {
	pub type_name: String,
	pub base_type: String,
	pub supported_operations: BTreeSet<FilterOperation>,
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
	pub overwrites: BTreeMap<String, Option<FilterType>>,
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

#[derive(Debug, Clone)]
pub struct FilterType {
	graphql: Option<GraphQlFilterType>,
	proto: Option<ProtoFilterType>,
}

impl FilterType {
	pub fn new(graphql: GraphQlFilterType, proto: ProtoFilterType) -> Self {
		Self {
			graphql: Some(graphql),
			proto: Some(proto),
		}
	}

	pub fn graphql(graphql: GraphQlFilterType) -> Self {
		Self {
			graphql: Some(graphql),
			proto: None,
		}
	}

	pub fn proto(proto: ProtoFilterType) -> Self {
		Self {
			graphql: None,
			proto: Some(proto),
		}
	}
}

/// The configuration for FilterType
pub struct FilterTypesMapHelper {
	pub context: &'static BuilderContext,
}

impl FilterTypesMapHelper {
	pub fn get_column_filter_type<T>(&self, column: &T::Column) -> Option<FilterType>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};

		let entity_name = entity_object_builder.type_name::<T>();
		let column_name = entity_object_builder.column_name::<T>(column);

		// used to honor overwrites
		if let Some(ty) =
			self.context.filter_types.overwrites.get(&format!("{entity_name}.{column_name}"))
		{
			return ty.clone();
		}

		// default mappings
		self.to_filter_type(column.def().get_column_type())
	}

	/// used to get the GraphQL input value field for a SeaORM entity column
	pub fn get_column_filter_input_value<T>(&self, column: &T::Column, tag: u32) -> Option<Field>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let column_name = entity_object_builder.column_name::<T>(column);

		let filter_type = self.get_column_filter_type::<T>(column);

		match filter_type {
			Some(filter_type) => Some(self.to_value(&column_name, tag, &filter_type, self.context)),
			None => None,
		}
	}

	/// used to parse a filter input object and update the query condition
	pub fn prepare_column_condition<T>(
		&self,
		condition: Condition,
		filter: &ObjectAccessors,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let types_map_helper = TypesMapHelper {
			context: self.context,
		};

		self.prepare::<T>(
			&self.get_column_filter_type::<T>(column),
			condition,
			filter,
			&types_map_helper,
			self.context,
			column,
		)
	}

	pub fn to_filter_type(&self, column_type: &ColumnType) -> Option<FilterType> {
		match column_type {
			ColumnType::Char(_) => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::String(_) => {
				Some(FilterType::new(GraphQlFilterType::String, ProtoFilterType::String))
			}
			ColumnType::Text => {
				Some(FilterType::new(GraphQlFilterType::String, ProtoFilterType::String))
			}
			ColumnType::TinyInteger => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::Int32))
			}
			ColumnType::SmallInteger => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::Int32))
			}
			ColumnType::Integer => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::Int32))
			}
			ColumnType::BigInteger => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::Int64))
			}
			ColumnType::TinyUnsigned => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::UInt32))
			}
			ColumnType::SmallUnsigned => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::UInt32))
			}
			ColumnType::Unsigned => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::UInt32))
			}
			ColumnType::BigUnsigned => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::UInt64))
			}
			ColumnType::Float => {
				Some(FilterType::new(GraphQlFilterType::Float, ProtoFilterType::Float))
			}
			ColumnType::Double => {
				Some(FilterType::new(GraphQlFilterType::Float, ProtoFilterType::Float))
			}
			ColumnType::Decimal(_) => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::DateTime => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Timestamp => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::TimestampWithTimeZone => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Time => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Date => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Year => {
				Some(FilterType::new(GraphQlFilterType::Integer, ProtoFilterType::Int32))
			}
			ColumnType::Interval(_, _) => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Binary(_) => Some(FilterType::proto(ProtoFilterType::Binary)),
			ColumnType::VarBinary(_) => Some(FilterType::proto(ProtoFilterType::Binary)),
			ColumnType::Bit(_) => Some(FilterType::proto(ProtoFilterType::Binary)),
			ColumnType::VarBit(_) => Some(FilterType::proto(ProtoFilterType::Binary)),
			ColumnType::Blob => None,
			ColumnType::Boolean => {
				Some(FilterType::new(GraphQlFilterType::Boolean, ProtoFilterType::Boolean))
			}
			ColumnType::Money(_) => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Json => None,
			ColumnType::JsonBinary => None,
			ColumnType::Uuid => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Custom(name) => Some(FilterType::new(
				GraphQlFilterType::Custom(name.to_string()),
				ProtoFilterType::Custom(name.to_string()),
			)),
			ColumnType::Enum {
				name,
				variants: _,
			} => Some(FilterType::new(
				GraphQlFilterType::Enumeration(name.to_string()),
				ProtoFilterType::Enumeration(name.to_string()),
			)),
			ColumnType::Array(_) => None,
			ColumnType::Cidr => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::Inet => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			ColumnType::MacAddr => {
				Some(FilterType::new(GraphQlFilterType::Text, ProtoFilterType::String))
			}
			_ => None,
		}
	}

	pub fn prepare<T>(
		&self,
		filter_type: &Option<FilterType>,
		mut condition: Condition,
		filter: &ObjectAccessors,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let filter_info_graphql = match filter_type {
			Some(filter_types) => match &filter_types.graphql {
				Some(filter_type) => {
					let filter_type_result = match filter_type {
						GraphQlFilterType::Text => {
							&self.context.filter_types.graphql.text_filter_info
						}
						GraphQlFilterType::String => {
							&self.context.filter_types.graphql.string_filter_info
						}
						GraphQlFilterType::Integer => {
							&self.context.filter_types.graphql.integer_filter_info
						}
						GraphQlFilterType::Float => {
							&self.context.filter_types.graphql.float_filter_info
						}
						GraphQlFilterType::Boolean => {
							&self.context.filter_types.graphql.boolean_filter_info
						}
						GraphQlFilterType::Id => &self.context.filter_types.graphql.id_filter_info,
						GraphQlFilterType::Enumeration(_) => {
							return prepare_enumeration_condition::<T>(filter, column, condition)
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
					Some(filter_type_result)
				}
				None => None,
			},
			None => None,
		};

		let filter_info_proto = match filter_type {
			Some(filter_types) => match &filter_types.proto {
				Some(filter_type) => {
					let filter_type_result = match filter_type {
						ProtoFilterType::Int32 => {
							&self.context.filter_types.proto.int32_filter_info
						}
						ProtoFilterType::Int64 => {
							&self.context.filter_types.proto.int64_filter_info
						}
						ProtoFilterType::UInt32 => {
							&self.context.filter_types.proto.uint32_filter_info
						}
						ProtoFilterType::UInt64 => {
							&self.context.filter_types.proto.uint64_filter_info
						}
						ProtoFilterType::SInt32 => {
							&self.context.filter_types.proto.sint32_filter_info
						}
						ProtoFilterType::SInt64 => {
							&self.context.filter_types.proto.sint64_filter_info
						}
						ProtoFilterType::Float => {
							&self.context.filter_types.proto.float_filter_info
						}
						ProtoFilterType::Double => {
							&self.context.filter_types.proto.double_filter_info
						}
						ProtoFilterType::Boolean => {
							&self.context.filter_types.proto.boolean_filter_info
						}
						ProtoFilterType::String => {
							&self.context.filter_types.proto.string_filter_info
						}
						ProtoFilterType::Binary => {
							&self.context.filter_types.proto.binary_filter_info
						}
						ProtoFilterType::Enumeration(_) => {
							return prepare_enumeration_condition::<T>(filter, column, condition)
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
					Some(filter_type_result)
				}
				None => None,
			},
			None => None,
		};

		if filter_info_graphql.is_none() && filter_info_proto.is_none() {
			return Ok(condition);
		}

		if let Some(filter_info) = filter_info_graphql {
			for operation in filter_info.supported_operations.iter() {
				match operation {
					FilterOperation::Equals => {
						if let Some(value) = filter.get("eq") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.eq(value));
						}
					}
					FilterOperation::NotEquals => {
						if let Some(value) = filter.get("ne") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.ne(value));
						}
					}
					FilterOperation::GreaterThan => {
						if let Some(value) = filter.get("gt") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.gt(value));
						}
					}
					FilterOperation::GreaterThanEquals => {
						if let Some(value) = filter.get("gte") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.gte(value));
						}
					}
					FilterOperation::LessThan => {
						if let Some(value) = filter.get("lt") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.lt(value));
						}
					}
					FilterOperation::LessThanEquals => {
						if let Some(value) = filter.get("lte") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
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
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.contains(value.to_string()));
						}
					}
					FilterOperation::StartsWith => {
						if let Some(value) = filter.get("starts_with") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.starts_with(value.to_string()));
						}
					}
					FilterOperation::EndsWith => {
						if let Some(value) = filter.get("ends_with") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.ends_with(value.to_string()));
						}
					}
					FilterOperation::Like => {
						if let Some(value) = filter.get("like") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.like(value.to_string()));
						}
					}
					FilterOperation::NotLike => {
						if let Some(value) = filter.get("not_like") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
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
		}

		if let Some(filter_info) = filter_info_proto {
			for operation in filter_info.supported_operations.iter() {
				match operation {
					FilterOperation::Equals => {
						if let Some(value) = filter.get("eq") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.eq(value));
						}
					}
					FilterOperation::NotEquals => {
						if let Some(value) = filter.get("ne") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.ne(value));
						}
					}
					FilterOperation::GreaterThan => {
						if let Some(value) = filter.get("gt") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.gt(value));
						}
					}
					FilterOperation::GreaterThanEquals => {
						if let Some(value) = filter.get("gte") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.gte(value));
						}
					}
					FilterOperation::LessThan => {
						if let Some(value) = filter.get("lt") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.lt(value));
						}
					}
					FilterOperation::LessThanEquals => {
						if let Some(value) = filter.get("lte") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
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
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.contains(value.to_string()));
						}
					}
					FilterOperation::StartsWith => {
						if let Some(value) = filter.get("starts_with") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.starts_with(value.to_string()));
						}
					}
					FilterOperation::EndsWith => {
						if let Some(value) = filter.get("ends_with") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.ends_with(value.to_string()));
						}
					}
					FilterOperation::Like => {
						if let Some(value) = filter.get("like") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
							condition = condition.add(column.like(value.to_string()));
						}
					}
					FilterOperation::NotLike => {
						if let Some(value) = filter.get("not_like") {
							let value =
								types_map_helper.value_to_sea_orm_value::<T>(column, &value)?;
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
		}

		Ok(condition)
	}

	/// used to get all basic input filter objects
	pub fn get_input_filters(&self) -> Vec<Object> {
		vec![
			self.generate_filter_input(&self.context.filter_types.proto.int32_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.int64_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.uint32_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.uint64_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.sint32_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.sint64_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.float_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.double_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.boolean_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.string_filter_info),
			self.generate_filter_input(&self.context.filter_types.proto.binary_filter_info),
		]
	}

	/// used to convert a filter input info struct into input object
	pub fn generate_filter_input(&self, filter_info: &FilterInfo) -> Object {
		filter_info.supported_operations.iter().enumerate().fold(
			Object::new(filter_info.type_name.to_string()),
			|object, (index, cur)| {
				let tag = index.add(1) as u32;
				let field = match cur {
					FilterOperation::Equals => Field::input(
						"eq",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::NotEquals => Field::input(
						"ne",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::GreaterThan => Field::input(
						"gt",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::GreaterThanEquals => Field::input(
						"gte",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::LessThan => Field::input(
						"lt",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::LessThanEquals => Field::input(
						"lte",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::IsIn => Field::input(
						"is_in",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
							ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
						),
					),
					FilterOperation::IsNotIn => Field::input(
						"is_not_in",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
							ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
						),
					),
					FilterOperation::IsNull => Field::input(
						"is_null",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::IsNotNull => Field::input(
						"is_not_null",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::Contains => Field::input(
						"contains",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::StartsWith => Field::input(
						"starts_with",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::EndsWith => Field::input(
						"ends_with",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::Like => Field::input(
						"like",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::NotLike => Field::input(
						"not_like",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named(filter_info.base_type.clone()),
							ProtoTypeRef::named(filter_info.base_type.clone()),
						),
					),
					FilterOperation::Between => Field::input(
						"between",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
							ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
						),
					),
					FilterOperation::NotBetween => Field::input(
						"not_between",
						tag,
						TypeRef::new(
							GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
							ProtoTypeRef::named_nn_list(filter_info.base_type.clone()),
						),
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
	) -> Field {
		let mut proto = None;
		let mut graphql = None;

		if let Some(filter_type) = &filter_type.graphql {
			graphql = match filter_type {
				GraphQlFilterType::Text => {
					Some(self.context.filter_types.graphql.text_filter_info.type_name.clone())
				}
				GraphQlFilterType::String => {
					Some(self.context.filter_types.graphql.text_filter_info.type_name.clone())
				}
				GraphQlFilterType::Integer => {
					Some(self.context.filter_types.graphql.text_filter_info.type_name.clone())
				}
				GraphQlFilterType::Float => {
					Some(self.context.filter_types.graphql.text_filter_info.type_name.clone())
				}
				GraphQlFilterType::Boolean => {
					Some(self.context.filter_types.graphql.text_filter_info.type_name.clone())
				}
				GraphQlFilterType::Id => {
					Some(self.context.filter_types.graphql.text_filter_info.type_name.clone())
				}
				GraphQlFilterType::Enumeration(name) => Some(name.clone()),
				GraphQlFilterType::Custom(type_name) => {
					let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
						context,
					};
					Some(active_enum_filter_input_builder.type_name_from_string(&type_name))
				}
			};
		};

		if let Some(filter_type) = &filter_type.proto {
			proto = match filter_type {
				ProtoFilterType::Int32 => {
					Some(self.context.filter_types.proto.int32_filter_info.type_name.clone())
				}
				ProtoFilterType::Int64 => {
					Some(self.context.filter_types.proto.int64_filter_info.type_name.clone())
				}
				ProtoFilterType::UInt32 => {
					Some(self.context.filter_types.proto.uint32_filter_info.type_name.clone())
				}
				ProtoFilterType::UInt64 => {
					Some(self.context.filter_types.proto.uint64_filter_info.type_name.clone())
				}
				ProtoFilterType::SInt32 => {
					Some(self.context.filter_types.proto.sint32_filter_info.type_name.clone())
				}
				ProtoFilterType::SInt64 => {
					Some(self.context.filter_types.proto.sint64_filter_info.type_name.clone())
				}
				ProtoFilterType::Float => {
					Some(self.context.filter_types.proto.float_filter_info.type_name.clone())
				}
				ProtoFilterType::Double => {
					Some(self.context.filter_types.proto.double_filter_info.type_name.clone())
				}
				ProtoFilterType::Boolean => {
					Some(self.context.filter_types.proto.boolean_filter_info.type_name.clone())
				}
				ProtoFilterType::String => {
					Some(self.context.filter_types.proto.string_filter_info.type_name.clone())
				}
				ProtoFilterType::Binary => {
					Some(self.context.filter_types.proto.binary_filter_info.type_name.clone())
				}
				ProtoFilterType::Enumeration(name) => Some(name.clone()),
				ProtoFilterType::Custom(type_name) => {
					let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
						context,
					};
					Some(active_enum_filter_input_builder.type_name_from_string(&type_name))
				}
			};
		};

		let type_ref_proto = if let Some(type_name) = proto {
			Some(ProtoTypeRef::named(type_name))
		} else {
			None
		};

		let type_ref_graphql = if let Some(type_name) = graphql {
			Some(GraphQLTypeRef::named(type_name))
		} else {
			None
		};

		Field::input(
			column_name,
			tag,
			TypeRef {
				proto: type_ref_proto,
				graphql: type_ref_graphql,
			},
		)
	}
}
