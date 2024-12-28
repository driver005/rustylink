use crate::{BuilderContext, EntityObjectBuilder, SeaResult, TypesMapHelper};
use dynamic::prelude::{
	GraphQLInputObject, GraphQLInputValue, GraphQLObjectAccessor, ProtoField, ProtoMessage,
	ProtoObjectAccessor,
};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait};
use std::collections::{BTreeMap, BTreeSet};

mod graphql;
mod proto;

pub use graphql::{
	FilterType as GraphQLFilterType, FilterTypesMapConfig as GraphQLFilterTypesMapConfig,
	FilterTypesMapHelper as GraphQLFilterTypesMapHelper,
	FnFilterCondition as GraphQLFnFilterCondition,
};
pub use proto::{
	FilterType as ProtoFilterType, FilterTypesMapConfig as ProtoFilterTypesMapConfig,
	FilterTypesMapHelper as ProtoFilterTypesMapHelper, FnFilterCondition as ProtoFnFilterCondition,
};

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

/// The FilterTypesMapHelper
/// * provides basic input filter types
/// * provides entity filter object type mappings
/// * helper functions that assist filtering on queries
/// * helper function that generate input filter types
pub trait FilterTypesMapHelper<'a> {
	type FilterType;
	type Field;
	type InputFilter;
	type ObjectAccessor;
	// type Value;
	// type Error;

	fn get_column_filter_type<T>(&self, column: &T::Column) -> Option<Self::FilterType>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		Self::FilterType: Clone,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context(),
		};

		let entity_name = entity_object_builder.type_name::<T>();
		let column_name = entity_object_builder.column_name::<T>(column);

		// used to honor overwrites
		if let Some(ty) = self.get_overwrites().get(&format!("{entity_name}.{column_name}")) {
			return ty.clone();
		}

		// default mappings
		self.to_filter_type(column.def().get_column_type())
	}

	/// used to get the GraphQL input value field for a SeaORM entity column
	fn get_column_filter_input_value<T>(&self, column: &T::Column, tag: u32) -> Option<Self::Field>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		Self::FilterType: Clone,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context(),
		};
		let column_name = entity_object_builder.column_name::<T>(column);

		let filter_type = self.get_column_filter_type::<T>(column);

		match filter_type {
			Some(filter_type) => {
				Some(self.to_value(&column_name, &filter_type, tag, self.context()))
			}
			None => None,
		}
	}

	/// used to parse a filter input object and update the query condition
	fn prepare_column_condition<T>(
		&self,
		condition: Condition,
		filter: &Self::ObjectAccessor,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		Self::FilterType: Clone,
	{
		let types_map_helper = TypesMapHelper {
			context: self.context(),
		};

		self.prepare::<T>(
			&self.get_column_filter_type::<T>(column),
			condition,
			filter,
			&types_map_helper,
			self.context(),
			column,
		)
	}

	fn to_filter_type(&self, column_type: &ColumnType) -> Option<Self::FilterType>;
	fn prepare<T>(
		&self,
		filter_type: &Option<Self::FilterType>,
		condition: Condition,
		filter: &Self::ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync;
	fn get_input_filters(&self) -> Vec<Self::InputFilter>;
	fn to_value(
		&self,
		column_name: &String,
		filter_type: &Self::FilterType,
		tag: u32,
		context: &'static BuilderContext,
	) -> Self::Field;
	fn context(&self) -> &'static BuilderContext;
	fn get_overwrites(&self) -> &BTreeMap<String, Option<Self::FilterType>>;
}

impl<'a> FilterTypesMapHelper<'a> for GraphQLFilterTypesMapHelper {
	type FilterType = GraphQLFilterType;
	type Field = GraphQLInputValue;
	type InputFilter = GraphQLInputObject;
	type ObjectAccessor = GraphQLObjectAccessor<'a>;

	fn to_filter_type(&self, column_type: &ColumnType) -> Option<Self::FilterType> {
		self.to_filter_type(column_type)
	}

	fn prepare<T>(
		&self,
		filter_type: &Option<Self::FilterType>,
		condition: Condition,
		filter: &Self::ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		self.prepare::<T>(filter_type, condition, filter, types_map_helper, context, column)
	}

	fn get_input_filters(&self) -> Vec<Self::InputFilter> {
		self.get_input_filters()
	}

	fn to_value(
		&self,
		column_name: &String,
		filter_type: &Self::FilterType,
		_: u32,
		context: &'static BuilderContext,
	) -> Self::Field {
		self.to_value(column_name, filter_type, context)
	}

	fn context(&self) -> &'static BuilderContext {
		self.context
	}

	fn get_overwrites(&self) -> &BTreeMap<String, Option<Self::FilterType>> {
		&self.context.filter_types_graphql.overwrites
	}
}

impl<'a> FilterTypesMapHelper<'a> for ProtoFilterTypesMapHelper {
	type FilterType = ProtoFilterType;
	type Field = ProtoField;
	type InputFilter = ProtoMessage;
	type ObjectAccessor = ProtoObjectAccessor<'a>;

	fn to_filter_type(&self, column_type: &ColumnType) -> Option<Self::FilterType> {
		self.to_filter_type(column_type)
	}

	fn prepare<T>(
		&self,
		filter_type: &Option<Self::FilterType>,
		condition: Condition,
		filter: &Self::ObjectAccessor,
		types_map_helper: &TypesMapHelper,
		context: &'static BuilderContext,
		column: &T::Column,
	) -> SeaResult<Condition>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		self.prepare::<T>(filter_type, condition, filter, types_map_helper, context, column)
	}

	fn get_input_filters(&self) -> Vec<Self::InputFilter> {
		self.get_input_filters()
	}

	fn to_value(
		&self,
		column_name: &String,
		filter_type: &Self::FilterType,
		tag: u32,
		context: &'static BuilderContext,
	) -> Self::Field {
		self.to_value(column_name, tag, filter_type, context)
	}

	fn context(&self) -> &'static BuilderContext {
		self.context
	}

	fn get_overwrites(&self) -> &BTreeMap<String, Option<Self::FilterType>> {
		&self.context.filter_types_proto.overwrites
	}
}
