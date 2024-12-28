use super::{FilterInfo, FilterOperation};
use crate::{
	prepare_enumeration_condition, ActiveEnumFilterInputBuilder, BuilderContext,
	EntityObjectBuilder, SeaResult, TypesMapHelper,
};
use dynamic::prelude::{
	GraphQLInputObject, GraphQLInputValue, GraphQLObjectAccessor, GraphQLTypeRef,
	GraphQLValueAccessor,
};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait};
use std::collections::{BTreeMap, BTreeSet};

pub type FnFilterCondition =
	Box<dyn Fn(Condition, &GraphQLObjectAccessor) -> SeaResult<Condition> + Send + Sync>;

/// The configuration for FilterTypesMapHelper
pub struct FilterTypesMapConfig {
	/// used to map entity_name.column_name to a custom filter type
	pub overwrites: BTreeMap<String, Option<FilterType>>,
	/// used to map entity_name.column_name to a custom condition function
	pub condition_functions: BTreeMap<String, FnFilterCondition>,

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

impl std::default::Default for FilterTypesMapConfig {
	fn default() -> Self {
		Self {
			overwrites: BTreeMap::default(),
			condition_functions: BTreeMap::default(),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterType {
	Text,
	String,
	Integer,
	Float,
	Boolean,
	Id,
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
			ColumnType::Char(_) => Some(FilterType::Text),
			ColumnType::String(_) => Some(FilterType::String),
			ColumnType::Text => Some(FilterType::String),
			ColumnType::TinyInteger => Some(FilterType::Integer),
			ColumnType::SmallInteger => Some(FilterType::Integer),
			ColumnType::Integer => Some(FilterType::Integer),
			ColumnType::BigInteger => Some(FilterType::Integer),
			ColumnType::TinyUnsigned => Some(FilterType::Integer),
			ColumnType::SmallUnsigned => Some(FilterType::Integer),
			ColumnType::Unsigned => Some(FilterType::Integer),
			ColumnType::BigUnsigned => Some(FilterType::Integer),
			ColumnType::Float => Some(FilterType::Float),
			ColumnType::Double => Some(FilterType::Float),
			ColumnType::Decimal(_) => Some(FilterType::Text),
			ColumnType::DateTime => Some(FilterType::Text),
			ColumnType::Timestamp => Some(FilterType::Text),
			ColumnType::TimestampWithTimeZone => Some(FilterType::Text),
			ColumnType::Time => Some(FilterType::Text),
			ColumnType::Date => Some(FilterType::Text),
			ColumnType::Year => Some(FilterType::Integer),
			ColumnType::Interval(_, _) => Some(FilterType::Text),
			ColumnType::Binary(_) => None,
			ColumnType::VarBinary(_) => None,
			ColumnType::Bit(_) => None,
			ColumnType::VarBit(_) => None,
			ColumnType::Blob => None,
			ColumnType::Boolean => Some(FilterType::Boolean),
			ColumnType::Money(_) => Some(FilterType::Text),
			ColumnType::Json => None,
			ColumnType::JsonBinary => None,
			ColumnType::Uuid => Some(FilterType::Text),
			ColumnType::Custom(name) => Some(FilterType::Custom(name.to_string())),
			ColumnType::Enum {
				name,
				variants: _,
			} => Some(FilterType::Enumeration(name.to_string())),
			ColumnType::Array(_) => None,
			ColumnType::Cidr => Some(FilterType::Text),
			ColumnType::Inet => Some(FilterType::Text),
			ColumnType::MacAddr => Some(FilterType::Text),
			_ => None,
		}
	}

	pub fn prepare<T>(
		&self,
		filter_type: &Option<FilterType>,
		mut condition: Condition,
		filter: &GraphQLObjectAccessor,
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
				FilterType::Text => &self.context.filter_types_graphql.text_filter_info,
				FilterType::String => &self.context.filter_types_graphql.string_filter_info,
				FilterType::Integer => &self.context.filter_types_graphql.integer_filter_info,
				FilterType::Float => &self.context.filter_types_graphql.float_filter_info,
				FilterType::Boolean => &self.context.filter_types_graphql.boolean_filter_info,
				FilterType::Id => &self.context.filter_types_graphql.id_filter_info,
				FilterType::Enumeration(_) => {
					return prepare_enumeration_condition::<T, GraphQLObjectAccessor>(
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
						.filter_types_graphql
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
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.eq(value));
					}
				}
				FilterOperation::NotEquals => {
					if let Some(value) = filter.get("ne") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.ne(value));
					}
				}
				FilterOperation::GreaterThan => {
					if let Some(value) = filter.get("gt") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.gt(value));
					}
				}
				FilterOperation::GreaterThanEquals => {
					if let Some(value) = filter.get("gte") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.gte(value));
					}
				}
				FilterOperation::LessThan => {
					if let Some(value) = filter.get("lt") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.lt(value));
					}
				}
				FilterOperation::LessThanEquals => {
					if let Some(value) = filter.get("lte") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
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
									.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &v)
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
									.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &v)
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
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.contains(value.to_string()));
					}
				}
				FilterOperation::StartsWith => {
					if let Some(value) = filter.get("starts_with") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.starts_with(value.to_string()));
					}
				}
				FilterOperation::EndsWith => {
					if let Some(value) = filter.get("ends_with") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.ends_with(value.to_string()));
					}
				}
				FilterOperation::Like => {
					if let Some(value) = filter.get("like") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
						condition = condition.add(column.like(value.to_string()));
					}
				}
				FilterOperation::NotLike => {
					if let Some(value) = filter.get("not_like") {
						let value = types_map_helper
							.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &value)?;
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
									.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &v)
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
									.value_to_sea_orm_value::<T, GraphQLValueAccessor>(column, &v)
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
	pub fn get_input_filters(&self) -> Vec<GraphQLInputObject> {
		vec![
			self.generate_filter_input(&self.context.filter_types_graphql.text_filter_info),
			self.generate_filter_input(&self.context.filter_types_graphql.string_filter_info),
			self.generate_filter_input(&self.context.filter_types_graphql.integer_filter_info),
			self.generate_filter_input(&self.context.filter_types_graphql.float_filter_info),
			self.generate_filter_input(&self.context.filter_types_graphql.boolean_filter_info),
			self.generate_filter_input(&self.context.filter_types_graphql.id_filter_info),
		]
	}

	/// used to convert a filter input info struct into input object
	pub fn generate_filter_input(&self, filter_info: &FilterInfo) -> GraphQLInputObject {
		filter_info.supported_operations.iter().fold(
			GraphQLInputObject::new(filter_info.type_name.to_string()),
			|object, cur| {
				let field = match cur {
					FilterOperation::Equals => GraphQLInputValue::new(
						"eq",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::NotEquals => GraphQLInputValue::new(
						"ne",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::GreaterThan => GraphQLInputValue::new(
						"gt",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::GreaterThanEquals => GraphQLInputValue::new(
						"gte",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::LessThan => GraphQLInputValue::new(
						"lt",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::LessThanEquals => GraphQLInputValue::new(
						"lte",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::IsIn => GraphQLInputValue::new(
						"is_in",
						GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
					FilterOperation::IsNotIn => GraphQLInputValue::new(
						"is_not_in",
						GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
					FilterOperation::IsNull => GraphQLInputValue::new(
						"is_null",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::IsNotNull => GraphQLInputValue::new(
						"is_not_null",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::Contains => GraphQLInputValue::new(
						"contains",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::StartsWith => GraphQLInputValue::new(
						"starts_with",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::EndsWith => GraphQLInputValue::new(
						"ends_with",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::Like => GraphQLInputValue::new(
						"like",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::NotLike => GraphQLInputValue::new(
						"not_like",
						GraphQLTypeRef::named(filter_info.base_type.clone()),
					),
					FilterOperation::Between => GraphQLInputValue::new(
						"between",
						GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
					FilterOperation::NotBetween => GraphQLInputValue::new(
						"not_between",
						GraphQLTypeRef::named_nn_list(filter_info.base_type.clone()),
					),
				};
				object.field(field)
			},
		)
	}

	pub fn to_value(
		&self,
		column_name: &String,
		filter_type: &FilterType,
		context: &'static BuilderContext,
	) -> GraphQLInputValue {
		match filter_type {
			FilterType::Text => {
				let info = &self.context.filter_types_graphql.text_filter_info;
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(info.type_name.clone()))
			}
			FilterType::String => {
				let info = &self.context.filter_types_graphql.string_filter_info;
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(info.type_name.clone()))
			}
			FilterType::Integer => {
				let info = &self.context.filter_types_graphql.integer_filter_info;
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(info.type_name.clone()))
			}
			FilterType::Float => {
				let info = &self.context.filter_types_graphql.float_filter_info;
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(info.type_name.clone()))
			}
			FilterType::Boolean => {
				let info = &self.context.filter_types_graphql.boolean_filter_info;
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(info.type_name.clone()))
			}
			FilterType::Id => {
				let info = &self.context.filter_types_graphql.id_filter_info;
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(info.type_name.clone()))
			}
			FilterType::Enumeration(name) => {
				let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
					context,
				};

				GraphQLInputValue::new(
					column_name,
					GraphQLTypeRef::named(
						active_enum_filter_input_builder.type_name_from_string(&name),
					),
				)
			}
			FilterType::Custom(type_name) => {
				GraphQLInputValue::new(column_name, GraphQLTypeRef::named(type_name))
			}
		}
	}
}
