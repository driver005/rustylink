use dynamic::prelude::*;
use sea_orm::{
	ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, TransactionTrait,
};

use crate::{
	BuilderContext, EntityInputBuilder, EntityObjectBuilder, EntityQueryFieldBuilder, GuardAction,
	prepare_active_model,
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
	pub fn to_field<T, A, Ty>(&self) -> Field<Ty>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
		Ty: TypeRefTrait,
	{
		let entity_input_builder = EntityInputBuilder {
			context: self.context,
		};
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};

		let context = self.context;

		let object_name: String = entity_object_builder.type_name::<T>();
		let guard = self.context.guards.entity_guards.get(&object_name);
		let field_guards = &self.context.guards.field_guards;

		Field::output(
			&self.type_name::<T>(),
			Ty::named_nn_list_nn(entity_object_builder.basic_type_name::<T>()),
			move |ctx| {
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

					let db = ctx.data::<DatabaseConnection>()?;
					let transaction = db.begin().await?;

					let entity_input_builder = EntityInputBuilder {
						context,
					};
					let entity_object_builder = EntityObjectBuilder {
						context,
					};

					let mut results: Vec<_> = Vec::new();
					let binding =
						match ctx.args.get(&context.entity_create_batch_mutation.data_field) {
							Some(value_accessor) => value_accessor,
							None => {
								return Err(SeaographyError::new(format!(
									"{} is a required argument but not provided.",
									context.entity_create_batch_mutation.data_field
								)));
							}
						}
						.list()?;

					for input in binding.to_iter().collect::<Vec<_>>() {
						let input_object = input.object()?;
						for (column, _) in input_object.to_iter() {
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
									Some(reason) => Err::<Option<_>, SeaographyError>(
										SeaographyError::new(reason),
									),
									None => Err::<Option<_>, SeaographyError>(
										SeaographyError::new("GraphQLField guard triggered."),
									),
								};
							}
						}

						let active_model = prepare_active_model::<T, A>(
							&entity_input_builder,
							&entity_object_builder,
							input_object,
						)?;

						let result = active_model.insert(&transaction).await?;
						results.push(result);
					}

					transaction.commit().await?;

					Ok(Some(FieldValue::list(results.into_iter().map(FieldValue::owned_any))))
				})
			},
		)
		.argument(Field::input(
			&context.entity_create_batch_mutation.data_field,
			Ty::named_nn_list_nn(entity_input_builder.insert_type_name::<T>()),
		))
	}
}
