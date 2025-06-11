use crate::{
	BuilderContext, Connection, ConnectionObjectBuilder, EntityObjectBuilder, FilterInputBuilder,
	FilterTypeTrait, GuardAction, HashableGroupKey, KeyComplex, OneToManyLoader, OneToOneLoader,
	OrderInputBuilder, PaginationInputBuilder, apply_memory_pagination, get_filter_conditions,
};
use dataloader::BatchFn;
use dynamic::prelude::*;
use heck::ToSnakeCase;
use sea_orm::{EntityTrait, Iden, ModelTrait, RelationDef};

/// This builder produces a GraphQL field for an SeaORM entity relationship
/// that can be added to the entity object
pub struct EntityObjectRelationBuilder {
	pub context: &'static BuilderContext,
}

impl EntityObjectRelationBuilder {
	/// used to get a field for an SeaORM entity relationship
	pub fn get_relation<T, R, Ty, F>(
		&self,
		name: &str,
		relation_definition: RelationDef,
	) -> SeaResult<Field<Ty>>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
		R: EntityTrait,
		<R as sea_orm::EntityTrait>::Model: Sync,
		<<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
		Ty: TypeRefTrait,
		F: FilterTypeTrait,
	{
		let context: &'static BuilderContext = self.context;
		let entity_object_builder = EntityObjectBuilder {
			context,
		};
		let connection_object_builder = ConnectionObjectBuilder {
			context,
		};
		let filter_input_builder = FilterInputBuilder {
			context,
		};
		let order_input_builder = OrderInputBuilder {
			context,
		};

		let object_name: String = entity_object_builder.type_name::<R>();
		let guard = self.context.guards.entity_guards.get(&object_name);

		let from_col = <T::Column as std::str::FromStr>::from_str(
			relation_definition.from_col.to_string().to_snake_case().as_str(),
		)
		.map_err(|err| SeaographyError::new(format!("{:#?}", err)))?;

		let to_col = <R::Column as std::str::FromStr>::from_str(
			relation_definition.to_col.to_string().to_snake_case().as_str(),
		)
		.map_err(|err| SeaographyError::new(format!("{:#?}", err)))?;

		let field = match relation_definition.is_owner {
			false => Field::output(name, Ty::named(&object_name), move |ctx| {
				FieldFuture::new(async move {
					let guard_flag = if let Some(guard) = guard {
						(*guard)(&ctx)
					} else {
						GuardAction::Allow
					};

					if let GuardAction::Block(reason) = guard_flag {
						return match reason {
							Some(reason) => {
								Err::<Option<_>, SeaographyError>(SeaographyError::new(reason))
							}
							None => Err::<Option<_>, SeaographyError>(SeaographyError::new(
								"Entity guard triggered.",
							)),
						};
					}

					let loader = ctx.data_unchecked::<OneToOneLoader<R>>();

					let parent: &T::Model = ctx
						.parent_value
						.try_downcast_ref::<T::Model>()
						.expect("Parent should exist");

					let stmt = R::find();
					let filters = ctx.args.get(&context.entity_query_field.filters);
					let filters = get_filter_conditions::<R, F>(context, filters)?;
					let order_by = ctx.args.get(&context.entity_query_field.order_by);
					let order_by = OrderInputBuilder {
						context,
					}
					.parse_object::<R>(order_by)?;
					let key = KeyComplex::<R> {
						key: vec![parent.get(from_col)],
						meta: HashableGroupKey::<R> {
							stmt,
							columns: vec![to_col],
							filters: Some(filters),
							order_by,
						},
					};

					let keys = vec![key];

					let mut values = loader.clone().load(&keys).await;

					if let Some(data) = values.remove(&keys[0]) {
						Ok(Some(FieldValue::owned_any(data)))
					} else {
						Ok(None)
					}
				})
			}),
			true => Field::output(
				name,
				Ty::named_nn(connection_object_builder.type_name(&object_name)),
				move |ctx| {
					let context: &'static BuilderContext = context;
					FieldFuture::new(async move {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return match reason {
								Some(reason) => {
									Err::<Option<_>, SeaographyError>(SeaographyError::new(reason))
								}
								None => Err::<Option<_>, SeaographyError>(SeaographyError::new(
									"Entity guard triggered.",
								)),
							};
						}

						let loader = ctx.data_unchecked::<OneToManyLoader<R>>();

						let parent: &T::Model = ctx
							.parent_value
							.try_downcast_ref::<T::Model>()
							.expect("Parent should exist");

						let stmt = R::find();
						let filters = ctx.args.get(&context.entity_query_field.filters);
						let filters = get_filter_conditions::<R, F>(context, filters)?;
						let order_by = ctx.args.get(&context.entity_query_field.order_by);
						let order_by = OrderInputBuilder {
							context,
						}
						.parse_object::<R>(order_by)?;
						let key = KeyComplex::<R> {
							key: vec![parent.get(from_col)],
							meta: HashableGroupKey::<R> {
								stmt,
								columns: vec![to_col],
								filters: Some(filters),
								order_by,
							},
						};

						let keys = vec![key];

						let mut values = loader.clone().load(&keys).await;

						let pagination = ctx.args.get(&context.entity_query_field.pagination);
						let pagination = PaginationInputBuilder {
							context,
						}
						.parse_object(pagination)?;

						let connection: Connection<R> =
							apply_memory_pagination(values.remove(&keys[0]), pagination)?;

						Ok(Some(FieldValue::owned_any(connection)))
					})
				},
			),
		};

		Ok(match relation_definition.is_owner {
			false => field,
			true => field
				.argument(Field::input(
					&context.entity_query_field.filters,
					Ty::named(filter_input_builder.type_name(&object_name)),
				))
				.argument(Field::input(
					&context.entity_query_field.order_by,
					Ty::named(order_input_builder.type_name(&object_name)),
				))
				.argument(Field::input(
					&context.entity_query_field.pagination,
					Ty::named(&context.pagination_input.type_name),
				)),
		})
	}
}
