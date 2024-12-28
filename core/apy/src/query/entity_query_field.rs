use dynamic::prelude::{
	DataContext, GraphQLError, GraphQLField, GraphQLFieldFuture, GraphQLFieldValue,
	GraphQLInputValue, GraphQLTypeRef, GraphQLValueAccessor, ProtoError, ProtoField,
	ProtoFieldFuture, ProtoFieldValue, ProtoTypeRef, ProtoValueAccessor,
};
use heck::ToLowerCamelCase;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
	apply_order, apply_pagination, get_filter_conditions, BuilderContext, ConnectionObjectBuilder,
	EntityObjectBuilder, FilterInputBuilder, GuardAction, OrderInputBuilder,
	PaginationInputBuilder,
};

/// The configuration structure for EntityQueryFieldBuilder
pub struct EntityQueryFieldConfig {
	/// used to format entity field name
	pub type_name: crate::SimpleNamingFn,
	/// name for 'filters' field
	pub filters: String,
	/// name for 'orderBy' field
	pub order_by: String,
	/// name for 'pagination' field
	pub pagination: String,
}

impl std::default::Default for EntityQueryFieldConfig {
	fn default() -> Self {
		EntityQueryFieldConfig {
			type_name: Box::new(|object_name: &str| -> String {
				object_name.to_lower_camel_case()
			}),
			filters: "filters".into(),
			order_by: "orderBy".into(),
			pagination: "pagination".into(),
		}
	}
}

/// This builder produces a field for the Query object that queries a SeaORM entity
pub struct EntityQueryFieldBuilder {
	pub context: &'static BuilderContext,
}

impl EntityQueryFieldBuilder {
	/// used to get field name for a SeaORM entity
	pub fn type_name<T>(&self) -> String
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object = EntityObjectBuilder {
			context: self.context,
		};
		let object_name = entity_object.type_name::<T>();
		self.context.entity_query_field.type_name.as_ref()(&object_name)
	}

	/// used to get the Query object field for a SeaORM entity
	pub fn to_graphql_field<T>(&self) -> GraphQLField
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let connection_object_builder = ConnectionObjectBuilder {
			context: self.context,
		};
		let filter_input_builder = FilterInputBuilder {
			context: self.context,
		};
		let order_input_builder = OrderInputBuilder {
			context: self.context,
		};
		let pagination_input_builder = PaginationInputBuilder {
			context: self.context,
		};
		let entity_object = EntityObjectBuilder {
			context: self.context,
		};

		let object_name = entity_object.type_name::<T>();
		let type_name = connection_object_builder.type_name(&object_name);

		let guard = self.context.guards_graphql.entity_guards.get(&object_name);

		let context: &'static BuilderContext = self.context;
		GraphQLField::new(self.type_name::<T>(), GraphQLTypeRef::named_nn(type_name), move |ctx| {
			let context: &'static BuilderContext = context;
			GraphQLFieldFuture::new(async move {
				let guard_flag = if let Some(guard) = guard {
					(*guard)(&ctx)
				} else {
					GuardAction::Allow
				};

				if let GuardAction::Block(reason) = guard_flag {
					return match reason {
						Some(reason) => Err::<Option<_>, GraphQLError>(GraphQLError::new(reason)),
						None => Err::<Option<_>, GraphQLError>(GraphQLError::new(
							"Entity guard triggered.",
						)),
					};
				}

				let filters = ctx.args.get(&context.entity_query_field.filters);
				let filters = get_filter_conditions::<T, GraphQLValueAccessor>(context, filters);
				let order_by = ctx.args.get(&context.entity_query_field.order_by);
				let order_by = OrderInputBuilder {
					context,
				}
				.parse_object::<T, GraphQLValueAccessor>(order_by);
				let pagination = ctx.args.get(&context.entity_query_field.pagination);
				let pagination = PaginationInputBuilder {
					context,
				}
				.parse_object(pagination);

				let stmt = T::find();
				let stmt = stmt.filter(filters);
				let stmt = apply_order(stmt, order_by);

				let db = ctx.data::<DatabaseConnection>()?;

				let connection = apply_pagination::<T>(db, stmt, pagination).await?;

				Ok(Some(GraphQLFieldValue::owned_any(connection)))
			})
		})
		.argument(GraphQLInputValue::new(
			&self.context.entity_query_field.filters,
			GraphQLTypeRef::named(filter_input_builder.type_name(&object_name)),
		))
		.argument(GraphQLInputValue::new(
			&self.context.entity_query_field.order_by,
			GraphQLTypeRef::named(order_input_builder.type_name(&object_name)),
		))
		.argument(GraphQLInputValue::new(
			&self.context.entity_query_field.pagination,
			GraphQLTypeRef::named(pagination_input_builder.type_name()),
		))
	}

	/// used to get the Query message field for a SeaORM entity
	pub fn to_proto_field<T>(&self) -> ProtoField
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let connection_object_builder = ConnectionObjectBuilder {
			context: self.context,
		};
		let filter_input_builder = FilterInputBuilder {
			context: self.context,
		};
		let order_input_builder = OrderInputBuilder {
			context: self.context,
		};
		let pagination_input_builder = PaginationInputBuilder {
			context: self.context,
		};
		let entity_object = EntityObjectBuilder {
			context: self.context,
		};

		let object_name = entity_object.type_name::<T>();
		let type_name = connection_object_builder.type_name(&object_name);

		let guard = self.context.guards_proto.entity_guards.get(&object_name);

		let context: &'static BuilderContext = self.context;
		ProtoField::output(
			self.type_name::<T>(),
			1u32,
			ProtoTypeRef::named_nn(type_name),
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
							Some(reason) => Err::<Option<_>, ProtoError>(ProtoError::new(reason)),
							None => Err::<Option<_>, ProtoError>(ProtoError::new(
								"Entity guard triggered.",
							)),
						};
					}

					let filters = ctx.args.get(&context.entity_query_field.filters);
					let filters = get_filter_conditions::<T, ProtoValueAccessor>(context, filters);
					let order_by = ctx.args.get(&context.entity_query_field.order_by);
					let order_by = OrderInputBuilder {
						context,
					}
					.parse_object::<T, ProtoValueAccessor>(order_by);
					let pagination = ctx.args.get(&context.entity_query_field.pagination);
					let pagination = PaginationInputBuilder {
						context,
					}
					.parse_object(pagination);

					let stmt = T::find();
					let stmt = stmt.filter(filters);
					let stmt = apply_order(stmt, order_by);

					let db = ctx.data::<DatabaseConnection>()?;

					let connection = apply_pagination::<T>(db, stmt, pagination).await?;

					Ok(Some(ProtoFieldValue::owned_any(connection)))
				})
			},
		)
		.argument(ProtoField::input(
			&self.context.entity_query_field.filters,
			1u32,
			ProtoTypeRef::named(filter_input_builder.type_name(&object_name)),
		))
		.argument(ProtoField::input(
			&self.context.entity_query_field.order_by,
			2u32,
			ProtoTypeRef::named(order_input_builder.type_name(&object_name)),
		))
		.argument(ProtoField::input(
			&self.context.entity_query_field.pagination,
			3u32,
			ProtoTypeRef::named(pagination_input_builder.type_name()),
		))
	}
}
