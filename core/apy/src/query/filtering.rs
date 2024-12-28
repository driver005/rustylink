use dynamic::prelude::{
	GraphQLObjectAccessor, ListAccessor, ListAccessors, ObjectAccessor, ObjectAccessors,
	ProtoObjectAccessor, ValueAccessor,
};
use sea_orm::{Condition, EntityTrait, Iterable};

use crate::{
	BuilderContext, EntityObjectBuilder, FilterTypesMapHelper, GraphQLFilterTypesMapHelper,
	ProtoFilterTypesMapHelper,
};

/// utility function used to create the query filter condition
/// for a SeaORM entity using query filter inputs
pub fn get_filter_conditions<'a, T, V>(
	context: &'static BuilderContext,
	filters: Option<V>,
) -> Condition
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
	V: ValueAccessor<'a>,
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
pub fn recursive_prepare_condition<'a, T, O>(
	context: &'static BuilderContext,
	filters: &'a O,
) -> Condition
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
	O: ObjectAccessor<'a>,
{
	let entity_object_builder = EntityObjectBuilder {
		context,
	};

	let to_codition = |data: &O::ValueAccessor| -> Condition {
		let filters = data.list().unwrap();

		match filters.get_accessor() {
			ListAccessors::GraphQL(graphql) => {
				graphql.to_iter().fold(Condition::any(), |condition, filters| {
					let filters = filters.object().unwrap();

					match filters.get_accessor() {
						ObjectAccessors::GraphQL(graphql) => {
							condition.add(recursive_prepare_condition::<T, GraphQLObjectAccessor>(
								context, &graphql,
							))
						}
						ObjectAccessors::Proto(proto) => condition.add(
							recursive_prepare_condition::<T, ProtoObjectAccessor>(context, &proto),
						),
					}
				})
			}
			ListAccessors::Proto(proto) => {
				proto.to_iter().fold(Condition::any(), |condition, filters| {
					let filters = filters.object().unwrap();

					match filters.get_accessor() {
						ObjectAccessors::GraphQL(graphql) => {
							condition.add(recursive_prepare_condition::<T, GraphQLObjectAccessor>(
								context, &graphql,
							))
						}
						ObjectAccessors::Proto(proto) => condition.add(
							recursive_prepare_condition::<T, ProtoObjectAccessor>(context, &proto),
						),
					}
				})
			}
		}
	};

	let condition = T::Column::iter().fold(Condition::all(), |condition, column: T::Column| {
		let column_name = entity_object_builder.column_name::<T>(&column);

		let filter = filters.get(&column_name);

		if let Some(filter) = filter {
			let filter = filter.object().unwrap();

			match filter.get_accessor() {
				ObjectAccessors::GraphQL(graphql) => GraphQLFilterTypesMapHelper {
					context,
				}
				.prepare_column_condition::<T>(condition, &graphql, &column)
				.unwrap(),
				ObjectAccessors::Proto(proto) => ProtoFilterTypesMapHelper {
					context,
				}
				.prepare_column_condition::<T>(condition, &proto, &column)
				.unwrap(),
			}
		} else {
			condition
		}
	});

	let condition = if let Some(and) = filters.get("and") {
		condition.add(to_codition(&and))
	} else {
		condition
	};

	let condition = if let Some(or) = filters.get("or") {
		condition.add(to_codition(&or))
	} else {
		condition
	};

	condition
}
