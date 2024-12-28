use dynamic::prelude::{
	DataContext, GraphQLError, GraphQLField, GraphQLFieldFuture, GraphQLFieldValue,
	GraphQLInputValue, GraphQLObjectAccessor, GraphQLTypeRef, ObjectAccessor, ObjectAccessors,
	ProtoError, ProtoField, ProtoFieldFuture, ProtoFieldValue, ProtoObjectAccessor, ProtoTypeRef,
};
use sea_orm::{
	ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Iterable,
	PrimaryKeyToColumn, PrimaryKeyTrait,
};

use crate::{
	BuilderContext, EntityInputBuilder, EntityObjectBuilder, EntityQueryFieldBuilder, GuardAction,
	SeaResult,
};

/// The configuration structure of EntityCreateOneMutationBuilder
pub struct EntityCreateOneMutationConfig {
	/// suffix that is appended on create mutations
	pub mutation_suffix: String,
	/// name for `data` field
	pub data_field: String,
}

impl std::default::Default for EntityCreateOneMutationConfig {
	fn default() -> Self {
		EntityCreateOneMutationConfig {
			mutation_suffix: "CreateOne".into(),
			data_field: "data".into(),
		}
	}
}

/// This builder produces the create one mutation for an entity
pub struct EntityCreateOneMutationBuilder {
	pub context: &'static BuilderContext,
}

impl EntityCreateOneMutationBuilder {
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
			self.context.entity_create_one_mutation.mutation_suffix
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
			GraphQLTypeRef::named_nn(entity_object_builder.basic_type_name::<T>()),
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

					let entity_input_builder = EntityInputBuilder {
						context,
					};
					let entity_object_builder = EntityObjectBuilder {
						context,
					};
					let db = ctx.data::<DatabaseConnection>()?;
					let value_accessor =
						ctx.args.get(&context.entity_create_one_mutation.data_field).unwrap();
					let input_object = &value_accessor.object()?;

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

					let result = active_model.insert(db).await?;

					Ok(Some(GraphQLFieldValue::owned_any(result)))
				})
			},
		)
		.argument(GraphQLInputValue::new(
			&context.entity_create_one_mutation.data_field,
			GraphQLTypeRef::named_nn(entity_input_builder.insert_type_name::<T>()),
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
			ProtoTypeRef::named_nn(entity_object_builder.basic_type_name::<T>()),
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

					let entity_input_builder = EntityInputBuilder {
						context,
					};
					let entity_object_builder = EntityObjectBuilder {
						context,
					};
					let db = ctx.data::<DatabaseConnection>()?;
					let value_accessor =
						ctx.args.get(&context.entity_create_one_mutation.data_field).unwrap();
					let input_object = &value_accessor.object()?;

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

					let result = active_model.insert(db).await?;

					Ok(Some(ProtoFieldValue::owned_any(result)))
				})
			},
		)
		.argument(ProtoField::input(
			&context.entity_create_one_mutation.data_field,
			1u32,
			ProtoTypeRef::named_nn(entity_input_builder.insert_type_name::<T>()),
		))
	}
}

pub fn prepare_active_model<'a, T, A, O>(
	entity_input_builder: &'a EntityInputBuilder,
	entity_object_builder: &'a EntityObjectBuilder,
	input_object: &'a O,
) -> SeaResult<A>
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
	<T as EntityTrait>::Model: IntoActiveModel<A>,
	A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	O: ObjectAccessor<'a>,
{
	let mut data = match input_object.get_accessor() {
		ObjectAccessors::GraphQL(graphql) => {
			entity_input_builder.parse_object::<T, GraphQLObjectAccessor>(&graphql)?
		}
		ObjectAccessors::Proto(proto) => {
			entity_input_builder.parse_object::<T, ProtoObjectAccessor>(&proto)?
		}
	};

	let mut active_model = A::default();

	for column in T::Column::iter() {
		// used to skip auto created primary keys
		let auto_increment = match <T::PrimaryKey as PrimaryKeyToColumn>::from_column(column) {
			Some(_) => T::PrimaryKey::auto_increment(),
			None => false,
		};

		if auto_increment {
			continue;
		}

		match data.remove(&entity_object_builder.column_name::<T>(&column)) {
			Some(value) => {
				active_model.set(column, value);
			}
			None => continue,
		}
	}

	Ok(active_model)
}
