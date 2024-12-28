use async_graphql::dataloader::DataLoader;
use dynamic::prelude::{
	DataContext, GraphQLError, GraphQLField, GraphQLFieldFuture, GraphQLFieldValue,
	GraphQLInputValue, GraphQLTypeRef, GraphQLValueAccessor, ProtoError, ProtoField,
	ProtoFieldFuture, ProtoFieldValue, ProtoTypeRef, ProtoValueAccessor,
};
use heck::ToSnakeCase;
use sea_orm::{EntityTrait, Iden, ModelTrait, RelationDef};

use crate::{
	apply_memory_pagination, get_filter_conditions, BuilderContext, Connection,
	ConnectionObjectBuilder, EntityObjectBuilder, FilterInputBuilder, GuardAction,
	HashableGroupKey, KeyComplex, OneToManyLoader, OneToOneLoader, OrderInputBuilder,
	PaginationInputBuilder,
};

/// This builder produces a GraphQL field for an SeaORM entity relationship
/// that can be added to the entity object
pub struct EntityObjectRelationBuilder {
	pub context: &'static BuilderContext,
}

impl EntityObjectRelationBuilder {
	/// used to get a GraphQL field for an SeaORM entity relationship
	pub fn get_graphql_relation<T, R>(
		&self,
		name: &str,
		relation_definition: RelationDef,
	) -> GraphQLField
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
		R: EntityTrait,
		<R as sea_orm::EntityTrait>::Model: Sync,
		<<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
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
		let guard = self.context.guards_graphql.entity_guards.get(&object_name);

		let from_col = <T::Column as std::str::FromStr>::from_str(
			relation_definition.from_col.to_string().to_snake_case().as_str(),
		)
		.unwrap();

		let to_col = <R::Column as std::str::FromStr>::from_str(
			relation_definition.to_col.to_string().to_snake_case().as_str(),
		)
		.unwrap();

		let field = match relation_definition.is_owner {
			false => GraphQLField::new(name, GraphQLTypeRef::named(&object_name), move |ctx| {
				GraphQLFieldFuture::new(async move {
					let guard_flag = if let Some(guard) = guard {
						(*guard)(&ctx)
					} else {
						GuardAction::Allow
					};

					if let GuardAction::Block(reason) = guard_flag {
						return match reason {
							Some(reason) => {
								Err::<Option<_>, GraphQLError>(GraphQLError::new(reason))
							}
							None => Err::<Option<_>, GraphQLError>(GraphQLError::new(
								"Entity guard triggered.",
							)),
						};
					}

					let parent: &T::Model = ctx
						.parent_value
						.try_downcast_ref::<T::Model>()
						.expect("Parent should exist");

					let loader = ctx.data_unchecked::<DataLoader<OneToOneLoader<R>>>();

					let stmt = R::find();
					let filters = ctx.args.get(&context.entity_query_field.filters);
					let filters =
						get_filter_conditions::<R, GraphQLValueAccessor>(context, filters);
					let order_by = ctx.args.get(&context.entity_query_field.order_by);
					let order_by = OrderInputBuilder {
						context,
					}
					.parse_object::<R, GraphQLValueAccessor>(order_by);
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
						Ok(Some(GraphQLFieldValue::owned_any(data)))
					} else {
						Ok(None)
					}
				})
			}),
			true => GraphQLField::new(
				name,
				GraphQLTypeRef::named_nn(connection_object_builder.type_name(&object_name)),
				move |ctx| {
					let context: &'static BuilderContext = context;
					GraphQLFieldFuture::new(async move {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return match reason {
								Some(reason) => {
									Err::<Option<_>, GraphQLError>(GraphQLError::new(reason))
								}
								None => Err::<Option<_>, GraphQLError>(GraphQLError::new(
									"Entity guard triggered.",
								)),
							};
						}

						let parent: &T::Model = ctx
							.parent_value
							.try_downcast_ref::<T::Model>()
							.expect("Parent should exist");

						let loader = ctx.data_unchecked::<DataLoader<OneToManyLoader<R>>>();

						let stmt = R::find();
						let filters = ctx.args.get(&context.entity_query_field.filters);
						let filters =
							get_filter_conditions::<R, GraphQLValueAccessor>(context, filters);
						let order_by = ctx.args.get(&context.entity_query_field.order_by);
						let order_by = OrderInputBuilder {
							context,
						}
						.parse_object::<R, GraphQLValueAccessor>(order_by);
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

						let pagination = ctx.args.get(&context.entity_query_field.pagination);
						let pagination = PaginationInputBuilder {
							context,
						}
						.parse_object(pagination);

						let connection: Connection<R> = apply_memory_pagination(values, pagination);

						Ok(Some(GraphQLFieldValue::owned_any(connection)))
					})
				},
			),
		};

		match relation_definition.is_owner {
			false => field,
			true => field
				.argument(GraphQLInputValue::new(
					&context.entity_query_field.filters,
					GraphQLTypeRef::named(filter_input_builder.type_name(&object_name)),
				))
				.argument(GraphQLInputValue::new(
					&context.entity_query_field.order_by,
					GraphQLTypeRef::named(order_input_builder.type_name(&object_name)),
				))
				.argument(GraphQLInputValue::new(
					&context.entity_query_field.pagination,
					GraphQLTypeRef::named(&context.pagination_input.type_name),
				)),
		}
	}

	/// used to get a Proto field for an SeaORM entity relationship
	pub fn get_proto_relation<T, R>(
		&self,
		name: &str,
		relation_definition: RelationDef,
	) -> ProtoField
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
		R: EntityTrait,
		<R as sea_orm::EntityTrait>::Model: Sync,
		<<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
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
		let guard = self.context.guards_proto.entity_guards.get(&object_name);

		let from_col = <T::Column as std::str::FromStr>::from_str(
			relation_definition.from_col.to_string().to_snake_case().as_str(),
		)
		.unwrap();

		let to_col = <R::Column as std::str::FromStr>::from_str(
			relation_definition.to_col.to_string().to_snake_case().as_str(),
		)
		.unwrap();

		let field = match relation_definition.is_owner {
			false => {
				ProtoField::output(name, 1u32, ProtoTypeRef::named(&object_name), move |ctx| {
					ProtoFieldFuture::new(async move {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return match reason {
								Some(reason) => {
									Err::<Option<_>, ProtoError>(ProtoError::new(reason))
								}
								None => Err::<Option<_>, ProtoError>(ProtoError::new(
									"Entity guard triggered.",
								)),
							};
						}

						let parent: &T::Model = ctx
							.parent_value
							.try_downcast_ref::<T::Model>()
							.expect("Parent should exist");

						let loader = ctx.data_unchecked::<DataLoader<OneToOneLoader<R>>>();

						let stmt = R::find();
						let filters = ctx.args.get(&context.entity_query_field.filters);
						let filters =
							get_filter_conditions::<R, ProtoValueAccessor>(context, filters);
						let order_by = ctx.args.get(&context.entity_query_field.order_by);
						let order_by = OrderInputBuilder {
							context,
						}
						.parse_object::<R, ProtoValueAccessor>(order_by);
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
							Ok(Some(ProtoFieldValue::owned_any(data)))
						} else {
							Ok(None)
						}
					})
				})
			}
			true => ProtoField::output(
				name,
				1u32,
				ProtoTypeRef::named_nn(connection_object_builder.type_name(&object_name)),
				move |ctx| {
					let context: &'static BuilderContext = context;
					ProtoFieldFuture::new(async move {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return match reason {
								Some(reason) => {
									Err::<Option<_>, ProtoError>(ProtoError::new(reason))
								}
								None => Err::<Option<_>, ProtoError>(ProtoError::new(
									"Entity guard triggered.",
								)),
							};
						}

						let parent: &T::Model = ctx
							.parent_value
							.try_downcast_ref::<T::Model>()
							.expect("Parent should exist");

						let loader = ctx.data_unchecked::<DataLoader<OneToManyLoader<R>>>();

						let stmt = R::find();
						let filters = ctx.args.get(&context.entity_query_field.filters);
						let filters =
							get_filter_conditions::<R, ProtoValueAccessor>(context, filters);
						let order_by = ctx.args.get(&context.entity_query_field.order_by);
						let order_by = OrderInputBuilder {
							context,
						}
						.parse_object::<R, ProtoValueAccessor>(order_by);
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

						let pagination = ctx.args.get(&context.entity_query_field.pagination);
						let pagination = PaginationInputBuilder {
							context,
						}
						.parse_object(pagination);

						let connection: Connection<R> = apply_memory_pagination(values, pagination);

						Ok(Some(ProtoFieldValue::owned_any(connection)))
					})
				},
			),
		};

		match relation_definition.is_owner {
			false => field,
			true => field
				.argument(ProtoField::input(
					&context.entity_query_field.filters,
					1u32,
					ProtoTypeRef::named(filter_input_builder.type_name(&object_name)),
				))
				.argument(ProtoField::input(
					&context.entity_query_field.order_by,
					2u32,
					ProtoTypeRef::named(order_input_builder.type_name(&object_name)),
				))
				.argument(ProtoField::input(
					&context.entity_query_field.pagination,
					3u32,
					ProtoTypeRef::named(&context.pagination_input.type_name),
				)),
		}
	}
}
