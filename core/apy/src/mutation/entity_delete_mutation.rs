use dynamic::prelude::{
	DataContext, GraphQLField, GraphQLFieldFuture, GraphQLInputValue, GraphQLTypeRef, GraphQLValue,
	GraphQLValueAccessor, ProtoField, ProtoFieldFuture, ProtoTypeRef, ProtoValue,
	ProtoValueAccessor,
};
use sea_orm::{
	ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::{
	get_filter_conditions, BuilderContext, EntityObjectBuilder, EntityQueryFieldBuilder,
	FilterInputBuilder,
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
	pub fn to_graphql_field<T, A>(&self) -> GraphQLField
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

		GraphQLField::new(
			self.type_name::<T>(),
			GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
			move |ctx| {
				GraphQLFieldFuture::new(async move {
					let db = ctx.data::<DatabaseConnection>()?;

					let filters = ctx.args.get(&context.entity_delete_mutation.filter_field);
					let filter_condition =
						get_filter_conditions::<T, GraphQLValueAccessor>(context, filters);

					let res: DeleteResult =
						T::delete_many().filter(filter_condition).exec(db).await?;

					Ok(Some(GraphQLValue::from(res.rows_affected)))
				})
			},
		)
		.argument(GraphQLInputValue::new(
			&context.entity_delete_mutation.filter_field,
			GraphQLTypeRef::named(entity_filter_input_builder.type_name(&object_name)),
		))
	}

	/// used to get the delete mutation field for a SeaORM entity
	pub fn to_proto_field<T, A>(&self) -> ProtoField
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

		ProtoField::output(
			self.type_name::<T>(),
			1u32,
			ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
			move |ctx| {
				ProtoFieldFuture::new(async move {
					let db = ctx.data::<DatabaseConnection>()?;

					let filters = ctx.args.get(&context.entity_delete_mutation.filter_field);
					let filter_condition =
						get_filter_conditions::<T, ProtoValueAccessor>(context, filters);

					let res: DeleteResult =
						T::delete_many().filter(filter_condition).exec(db).await?;

					Ok(Some(ProtoValue::from(res.rows_affected)))
				})
			},
		)
		.argument(ProtoField::input(
			&context.entity_delete_mutation.filter_field,
			1u32,
			ProtoTypeRef::named(entity_filter_input_builder.type_name(&object_name)),
		))
	}
}
