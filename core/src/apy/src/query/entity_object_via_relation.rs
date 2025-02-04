use crate::{
	apply_memory_pagination, apply_order, apply_pagination, get_filter_conditions, BuilderContext,
	ConnectionObjectBuilder, EntityObjectBuilder, FilterInputBuilder, GuardAction,
	HashableGroupKey, KeyComplex, OneToManyLoader, OneToOneLoader, OrderInputBuilder,
	PaginationInputBuilder,
};
use async_graphql::dataloader::DataLoader;
use dynamic::prelude::*;
use heck::ToSnakeCase;
use sea_orm::{
	ColumnTrait, Condition, DatabaseConnection, EntityTrait, Iden, ModelTrait, QueryFilter, Related,
};

/// This builder produces a GraphQL field for an SeaORM entity related trait
/// that can be added to the entity object
pub struct EntityObjectViaRelationBuilder {
	pub context: &'static BuilderContext,
}

impl EntityObjectViaRelationBuilder {
	/// used to get a field for an SeaORM entity related trait
	pub fn get_relation<T, R>(&self, name: &str) -> Field
	where
		T: Related<R>,
		T: EntityTrait,
		R: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<R as sea_orm::EntityTrait>::Model: Sync,
		<<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
		<<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
	{
		let context: &'static BuilderContext = self.context;
		let to_relation_definition = <T as Related<R>>::to();
		let (via_relation_definition, is_via_relation) = match <T as Related<R>>::via() {
			Some(def) => (def, true),
			None => (<T as Related<R>>::to(), false),
		};

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
			via_relation_definition.from_col.to_string().to_snake_case().as_str(),
		)
		.unwrap();

		let to_col = <R::Column as std::str::FromStr>::from_str(
			to_relation_definition.to_col.to_string().to_snake_case().as_str(),
		)
		.unwrap();

		let field = match via_relation_definition.is_owner {
			false => Field::output(
				name,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named(&object_name),
					ProtoTypeRef::named(&object_name),
				),
				move |ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return match reason {
								Some(reason) => Err::<Option<_>, Error>(Error::new(reason)),
								None => {
									Err::<Option<_>, Error>(Error::new("Entity guard triggered."))
								}
							};
						}

						let loader = ctx.data_unchecked::<DataLoader<OneToOneLoader<R>>>();

						let parent: &T::Model = ctx
							.parent_value
							.try_downcast_ref::<T::Model>()
							.expect("Parent should exist");

						let stmt = if <T as Related<R>>::via().is_some() {
							<T as Related<R>>::find_related()
						} else {
							R::find()
						};

						let filters = ctx.args.get(&context.entity_query_field.filters);
						let filters = get_filter_conditions::<R>(context, filters);
						let order_by = ctx.args.get(&context.entity_query_field.order_by);
						let order_by = OrderInputBuilder {
							context,
						}
						.parse_object::<R>(order_by);
						let key = KeyComplex::<R> {
							key: vec![parent.get(from_col)],
							meta: HashableGroupKey::<R> {
								stmt,
								columns: vec![to_col],
								filters: Some(filters),
								order_by,
							},
						};

						let data = loader.load_one(key).await?;

						if let Some(data) = data {
							Ok(Some(FieldValue::owned_any(data)))
						} else {
							Ok(None)
						}
					})
				},
			),
			true => Field::output(
				name,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(connection_object_builder.type_name(&object_name)),
					ProtoTypeRef::named_nn(connection_object_builder.type_name(&object_name)),
				),
				move |ctx| {
					let context: &'static BuilderContext = context;
					FieldFuture::new(ctx.api_type.clone(), async move {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return match reason {
								Some(reason) => Err::<Option<_>, Error>(Error::new(reason)),
								None => {
									Err::<Option<_>, Error>(Error::new("Entity guard triggered."))
								}
							};
						}

						let stmt = if <T as Related<R>>::via().is_some() {
							<T as Related<R>>::find_related()
						} else {
							R::find()
						};

						let filters = ctx.args.get(&context.entity_query_field.filters);
						let filters = get_filter_conditions::<R>(context, filters);

						let order_by = ctx.args.get(&context.entity_query_field.order_by);
						let order_by = OrderInputBuilder {
							context,
						}
						.parse_object::<R>(order_by);

						let pagination = ctx.args.get(&context.entity_query_field.pagination);
						let pagination = PaginationInputBuilder {
							context,
						}
						.parse_object(pagination);

						let db = ctx.data::<DatabaseConnection>()?;

						let connection = if is_via_relation {
							// FIXME: optimize union queries
							// NOTE: each has unique query in order to apply pagination...
							let parent: &T::Model = ctx
								.parent_value
								.try_downcast_ref::<T::Model>()
								.expect("Parent should exist");

							// TODO optimize query
							let condition = Condition::all().add(from_col.eq(parent.get(from_col)));

							let stmt = stmt.filter(condition.add(filters));
							let stmt = apply_order(stmt, order_by);
							apply_pagination::<R>(db, stmt, pagination).await?
						} else {
							let loader = ctx.data_unchecked::<DataLoader<OneToManyLoader<R>>>();

							// FIXME: optimize union queries
							// NOTE: each has unique query in order to apply pagination...
							let parent: &T::Model = ctx
								.parent_value
								.try_downcast_ref::<T::Model>()
								.expect("Parent should exist");

							let key = KeyComplex::<R> {
								key: vec![parent.get(from_col)],
								meta: HashableGroupKey::<R> {
									stmt,
									columns: vec![to_col],
									filters: Some(filters),
									order_by,
								},
							};

							let values = loader.load_one(key).await?;

							apply_memory_pagination(values, pagination)
						};

						Ok(Some(FieldValue::owned_any(connection)))
					})
				},
			),
		};

		match via_relation_definition.is_owner {
			false => field,
			true => field
				.argument(Field::input(
					&context.entity_query_field.filters,
					1u32,
					TypeRef::new(
						GraphQLTypeRef::named(filter_input_builder.type_name(&object_name)),
						ProtoTypeRef::named(filter_input_builder.type_name(&object_name)),
					),
				))
				.argument(Field::input(
					&context.entity_query_field.order_by,
					2u32,
					TypeRef::new(
						GraphQLTypeRef::named(order_input_builder.type_name(&object_name)),
						ProtoTypeRef::named(order_input_builder.type_name(&object_name)),
					),
				))
				.argument(Field::input(
					&context.entity_query_field.pagination,
					3u32,
					TypeRef::new(
						GraphQLTypeRef::named(&context.pagination_input.type_name),
						ProtoTypeRef::named(&context.pagination_input.type_name),
					),
				)),
		}
	}
}
