use dynamic::prelude::{
	GraphQLObjectAccessor, ListAccessorTrait, ListAccessors, ObjectAccessorTrait, ObjectAccessors,
	ProtoObjectAccessor, ValueAccessorTrait, ValueAccessors,
};
use sea_orm::{Condition, EntityTrait, Iterable};

use crate::{
	BuilderContext, EntityObjectBuilder, FilterTypesMapHelper, GraphQLFilterTypesMapHelper,
	ProtoFilterTypesMapHelper,
};

/// utility function used to create the query filter condition
/// for a SeaORM entity using query filter inputs
pub fn get_filter_conditions<'a, T>(
	context: &'static BuilderContext,
	filters: Option<ValueAccessors<'a>>,
) -> Condition
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
{
	if let Some(filters) = filters {
		let filters = filters.object().unwrap();

		match filters.get_accessor() {
			ObjectAccessors::GraphQL(graphql) => {
				recursive_prepare_condition::<T, GraphQLObjectAccessor>(context, &graphql)
			}
			ObjectAccessors::Proto(proto) => {
				recursive_prepare_condition::<T, ProtoObjectAccessor>(context, &proto)
			}
		}
	} else {
		Condition::all()
	}
}

/// used to prepare recursively the query filtering condition
pub fn recursive_prepare_condition<'a, T>(
	context: &'static BuilderContext,
	filters: &'a ObjectAccessors,
) -> Condition
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
{
	let entity_object_builder = EntityObjectBuilder {
		context,
	};

	let filter_types_map_helper = FilterTypesMapHelper {
		context,
	};

	let condition = T::Column::iter().fold(Condition::all(), |condition, column: T::Column| {
		let column_name = entity_object_builder.column_name::<T>(&column);

		let filter = filters.get(&column_name);

		if let Some(filter) = filter {
			let filter = filter.object().unwrap();

			filter_types_map_helper
				.prepare_column_condition::<T>(condition, &filter, &column)
				.unwrap()
		} else {
			condition
		}
	});

	let condition = if let Some(and) = filters.get("and") {
		let filters = and.list().unwrap();

		condition.add(filters.to_iter().fold(
			Condition::all(),
			|condition, filters: ValueAccessors| {
				let filters = filters.object().unwrap();
				condition.add(recursive_prepare_condition::<T>(context, &filters))
			},
		))
	} else {
		condition
	};

	let condition = if let Some(or) = filters.get("or") {
		let filters = or.list().unwrap();

		condition.add(filters.to_iter().fold(
			Condition::any(),
			|condition, filters: ValueAccessors| {
				let filters = filters.object().unwrap();
				condition.add(recursive_prepare_condition::<T>(context, &filters))
			},
		))
	} else {
		condition
	};

	condition
}
