use crate::{BuilderContext, EntityObjectBuilder, FilterTypeTrait, FilterTypesMapHelper};
use dynamic::prelude::{
	ListAccessorTrait, ObjectAccessor, ObjectAccessorTrait, ValueAccessor, ValueAccessorTrait,
};
use sea_orm::{Condition, EntityTrait, Iterable};

/// utility function used to create the query filter condition
/// for a SeaORM entity using query filter inputs
pub fn get_filter_conditions<'a, T, F>(
	context: &'static BuilderContext,
	filters: Option<ValueAccessor<'a>>,
) -> Condition
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
	F: FilterTypeTrait,
{
	if let Some(filters) = filters {
		let filters = filters.object().unwrap();

		recursive_prepare_condition::<T, F>(context, &filters)
	} else {
		Condition::all()
	}
}

/// used to prepare recursively the query filtering condition
pub fn recursive_prepare_condition<'a, T, F>(
	context: &'static BuilderContext,
	filters: &'a ObjectAccessor,
) -> Condition
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
	F: FilterTypeTrait,
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
				.prepare_column_condition::<T, F>(condition, &filter, &column)
				.unwrap()
		} else {
			condition
		}
	});

	let condition = if let Some(and) = filters.get("and") {
		let filters = and.list().unwrap();

		condition.add(filters.to_iter().fold(
			Condition::all(),
			|condition, filters: ValueAccessor| {
				let filters = filters.object().unwrap();
				condition.add(recursive_prepare_condition::<T, F>(context, &filters))
			},
		))
	} else {
		condition
	};

	let condition = if let Some(or) = filters.get("or") {
		let filters = or.list().unwrap();

		condition.add(filters.to_iter().fold(
			Condition::any(),
			|condition, filters: ValueAccessor| {
				let filters = filters.object().unwrap();
				condition.add(recursive_prepare_condition::<T, F>(context, &filters))
			},
		))
	} else {
		condition
	};

	condition
}
