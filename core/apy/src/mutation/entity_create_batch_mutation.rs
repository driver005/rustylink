use dynamic::prelude::{
	DataContext, GraphQLError, GraphQLField, GraphQLFieldFuture, GraphQLFieldValue,
	GraphQLInputValue, GraphQLObjectAccessor, GraphQLTypeRef, ProtoError, ProtoField,
	ProtoFieldFuture, ProtoFieldValue, ProtoObjectAccessor, ProtoTypeRef,
};
use sea_orm::{
	ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, TransactionTrait,
};

use crate::{
	prepare_active_model, BuilderContext, EntityInputBuilder, EntityObjectBuilder,
	EntityQueryFieldBuilder, GuardAction,
};

/// The configuration structure of EntityCreateBatchMutationBuilder
pub struct EntityCreateBatchMutationConfig {
	/// suffix that is appended on create mutations
	pub mutation_suffix: String,
	/// name for `data` field
	pub data_field: String,
}

impl std::default::Default for EntityCreateBatchMutationConfig {
	fn default() -> Self {
		EntityCreateBatchMutationConfig {
			mutation_suffix: "CreateBatch".into(),
			data_field: "data".into(),
		}
	}
}

/// This builder produces the create batch mutation for an entity
pub struct EntityCreateBatchMutationBuilder {
	pub context: &'static BuilderContext,
}

impl EntityCreateBatchMutationBuilder {
	/// used to get mutation name for a SeaORM entity
	pub fn type_name<T>(&self) -> String
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_query_field_builder = EntityQueryFieldBuilder {
			context: self.context,
		};
		format!(
			"{}{}",
			entity_query_field_builder.type_name::<T>(),
			self.context.entity_create_batch_mutation.mutation_suffix
		)
	}

	/// used to get the create mutation field for a SeaORM entity
	pub fn to_graphql_field<T, A>(&self) -> GraphQLField
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		let entity_input_builder = EntityInputBuilder {
			context: self.context,
		};
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};

		let context = self.context;

		let object_name: String = entity_object_builder.type_name::<T>();
		let guard = self.context.guards_graphql.entity_guards.get(&object_name);
		let field_guards = &self.context.guards_graphql.field_guards;

		GraphQLField::new(
			self.type_name::<T>(),
			GraphQLTypeRef::named_nn_list_nn(entity_object_builder.basic_type_name::<T>()),
			move |ctx| {
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

					let db = ctx.data::<DatabaseConnection>()?;
					let transaction = db.begin().await?;

					let entity_input_builder = EntityInputBuilder {
						context,
					};
					let entity_object_builder = EntityObjectBuilder {
						context,
					};

					let mut results: Vec<_> = Vec::new();
					for input in ctx
						.args
						.get(&context.entity_create_batch_mutation.data_field)
						.unwrap()
						.list()?
						.iter()
					{
						let input_object = &input.object()?;
						for (column, _) in input_object.iter() {
							let field_guard = field_guards.get(&format!(
								"{}.{}",
								entity_object_builder.type_name::<T>(),
								column
							));
							let field_guard_flag = if let Some(field_guard) = field_guard {
								(*field_guard)(&ctx)
							} else {
								GuardAction::Allow
							};
							if let GuardAction::Block(reason) = field_guard_flag {
								return match reason {
									Some(reason) => {
										Err::<Option<_>, GraphQLError>(GraphQLError::new(reason))
									}
									None => Err::<Option<_>, GraphQLError>(GraphQLError::new(
										"GraphQLField guard triggered.",
									)),
								};
							}
						}

						let active_model = prepare_active_model::<T, A, GraphQLObjectAccessor>(
							&entity_input_builder,
							&entity_object_builder,
							input_object,
						)?;
						let result = active_model.insert(&transaction).await?;
						results.push(result);
					}

					transaction.commit().await?;

					Ok(Some(GraphQLFieldValue::list(
						results.into_iter().map(GraphQLFieldValue::owned_any),
					)))
				})
			},
		)
		.argument(GraphQLInputValue::new(
			&context.entity_create_batch_mutation.data_field,
			GraphQLTypeRef::named_nn_list_nn(entity_input_builder.insert_type_name::<T>()),
		))
	}

	/// used to get the create mutation field for a SeaORM entity
	pub fn to_proto_field<T, A>(&self) -> ProtoField
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		let entity_input_builder = EntityInputBuilder {
			context: self.context,
		};
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};

		let context = self.context;

		let object_name: String = entity_object_builder.type_name::<T>();
		let guard = self.context.guards_proto.entity_guards.get(&object_name);
		let field_guards = &self.context.guards_proto.field_guards;

		ProtoField::output(
			self.type_name::<T>(),
			1u32,
			ProtoTypeRef::named_nn_list_nn(entity_object_builder.basic_type_name::<T>()),
			move |ctx| {
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

					let db = ctx.data::<DatabaseConnection>()?;
					let transaction = db.begin().await?;

					let entity_input_builder = EntityInputBuilder {
						context,
					};
					let entity_object_builder = EntityObjectBuilder {
						context,
					};

					let mut results: Vec<_> = Vec::new();
					for input in ctx
						.args
						.get(&context.entity_create_batch_mutation.data_field)
						.unwrap()
						.list()?
						.iter()
					{
						let input_object = &input.object()?;
						for (column, _) in input_object.iter() {
							let field_guard = field_guards.get(&format!(
								"{}.{}",
								entity_object_builder.type_name::<T>(),
								column
							));
							let field_guard_flag = if let Some(field_guard) = field_guard {
								(*field_guard)(&ctx)
							} else {
								GuardAction::Allow
							};
							if let GuardAction::Block(reason) = field_guard_flag {
								return match reason {
									Some(reason) => {
										Err::<Option<_>, ProtoError>(ProtoError::new(reason))
									}
									None => Err::<Option<_>, ProtoError>(ProtoError::new(
										"GraphQLField guard triggered.",
									)),
								};
							}
						}

						let active_model = prepare_active_model::<T, A, ProtoObjectAccessor>(
							&entity_input_builder,
							&entity_object_builder,
							input_object,
						)?;
						let result = active_model.insert(&transaction).await?;
						results.push(result);
					}

					transaction.commit().await?;

					Ok(Some(ProtoFieldValue::list(
						results.into_iter().map(ProtoFieldValue::owned_any),
					)))
				})
			},
		)
		.argument(ProtoField::input(
			&context.entity_create_batch_mutation.data_field,
			1u32,
			ProtoTypeRef::named_nn_list_nn(entity_input_builder.insert_type_name::<T>()),
		))
	}
}
