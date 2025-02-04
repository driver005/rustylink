use crate::{
	get_filter_conditions, BuilderContext, EntityObjectBuilder, EntityQueryFieldBuilder,
	FilterInputBuilder,
};
use dynamic::prelude::*;
use sea_orm::{
	ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel, QueryFilter,
};

/// The configuration structure of EntityDeleteMutationBuilder
pub struct EntityDeleteMutationConfig {
	/// suffix that is appended on delete mutations
	pub mutation_suffix: String,

	/// name for `filter` field
	pub filter_field: String,
}

impl std::default::Default for EntityDeleteMutationConfig {
	fn default() -> Self {
		Self {
			mutation_suffix: "Delete".into(),
			filter_field: "filter".into(),
		}
	}
}

/// This builder produces the delete mutation for an entity
pub struct EntityDeleteMutationBuilder {
	pub context: &'static BuilderContext,
}

impl EntityDeleteMutationBuilder {
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
			self.context.entity_delete_mutation.mutation_suffix
		)
	}

	/// used to get the delete mutation field for a SeaORM entity
	pub fn to_field<T, A>(&self) -> Field
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		let entity_filter_input_builder = FilterInputBuilder {
			context: self.context,
		};
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let object_name: String = entity_object_builder.type_name::<T>();

		let context = self.context;

		Field::output(
			self.type_name::<T>(),
			1u32,
			TypeRef::new(
				GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
				ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
			),
			move |ctx| {
				FieldFuture::new(ctx.api_type.clone(), async move {
					let db = ctx.data::<DatabaseConnection>()?;

					let filters = ctx.args.get(&context.entity_delete_mutation.filter_field);
					let filter_condition = get_filter_conditions::<T>(context, filters);

					let res: DeleteResult =
						T::delete_many().filter(filter_condition).exec(db).await?;

					Ok(Some(Value::new(
						GraphQLValue::from(res.rows_affected),
						ProtoValue::from(res.rows_affected),
					)))
				})
			},
		)
		.argument(Field::input(
			&context.entity_delete_mutation.filter_field,
			1u32,
			TypeRef::new(
				GraphQLTypeRef::named(entity_filter_input_builder.type_name(&object_name)),
				ProtoTypeRef::named(entity_filter_input_builder.type_name(&object_name)),
			),
		))
	}
}
