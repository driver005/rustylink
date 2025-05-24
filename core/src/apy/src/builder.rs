use crate::{
	ActiveEnumBuilder, ActiveEnumFilterInputBuilder, BuilderContext, ConnectionObjectBuilder,
	CursorInputBuilder, EdgeObjectBuilder, EntityCreateBatchMutationBuilder,
	EntityCreateOneMutationBuilder, EntityDeleteMutationBuilder, EntityInputBuilder,
	EntityObjectBuilder, EntityQueryFieldBuilder, EntityUpdateMutationBuilder, FilterInputBuilder,
	FilterTypeTrait, OffsetInputBuilder, OneToManyLoader, OneToOneLoader, OrderByEnumBuilder,
	OrderInputBuilder, PageInfoObjectBuilder, PageInputBuilder, PaginationInfoObjectBuilder,
	PaginationInputBuilder,
};
use async_graphql::dataloader::DataLoader;
use dynamic::prelude::*;
use sea_orm::{ActiveEnum, ActiveModelTrait, EntityTrait, IntoActiveModel};

/// The Builder is used to create the Schema for GraphQL
///
/// You can populate it with the entities, enumerations of your choice
pub struct Builder<Ty, E, F>
where
	Ty: TypeRefTrait,
	E: EnumTrait,
	F: FilterTypeTrait,
{
	//GraphQL
	pub query: Object<Ty>,
	pub mutation: Object<Ty>,
	pub builder: DynamicBuilder<Ty, E>,

	/// holds all output object types
	pub outputs: Vec<Object<Ty>>,

	/// holds all input object types
	pub inputs: Vec<Object<Ty>>,

	/// holds all enumeration types
	pub enumerations: Vec<E>,

	/// holds all entities queries
	pub queries: Vec<Field<Ty>>,

	/// holds all entities mutations
	pub mutations: Vec<Field<Ty>>,

	/// holds a copy to the database connection
	pub connection: sea_orm::DatabaseConnection,

	/// configuration for builder
	pub context: &'static BuilderContext,

	_marker: std::marker::PhantomData<F>,
}

impl<Ty, E, F> Builder<Ty, E, F>
where
	Ty: TypeRefTrait,
	E: EnumTrait,
	F: FilterTypeTrait,
{
	/// Used to create a new Builder from the given configuration context
	pub fn new(context: &'static BuilderContext, connection: sea_orm::DatabaseConnection) -> Self {
		let query = Object::new("Query", IO::Output);
		let mutation = Object::new("Mutation", IO::Output).field(Field::output(
			"_ping",
			1u32,
			Ty::named(Ty::STRING),
			|_| FieldFuture::new(async move { Ok(Some(Value::from("pong"))) }),
		));
		let schema_builder = Schema::build(query.type_name(), Some(mutation.type_name()), None);

		let proto_builder = Proto::build(mutation.type_name());

		let builder = DynamicBuilder::new(schema_builder, proto_builder);

		Self {
			query,
			mutation,
			builder,
			outputs: Vec::new(),
			inputs: Vec::new(),
			enumerations: Vec::new(),
			queries: Vec::new(),
			mutations: Vec::new(),
			connection,
			context,
			_marker: std::marker::PhantomData,
		}
	}

	pub fn register_entity<T>(&mut self, relations: Vec<Field<Ty>>)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let entity_object = relations
			.into_iter()
			.fold(entity_object_builder.to_object::<T, Ty>(), |entity_object, field| {
				entity_object.field(field)
			});

		let edge_object_builder = EdgeObjectBuilder {
			context: self.context,
		};
		let edge = edge_object_builder.to_object::<T, Ty>();

		let connection_object_builder = ConnectionObjectBuilder {
			context: self.context,
		};
		let connection = connection_object_builder.to_object::<T, Ty>();

		self.outputs.extend(vec![entity_object, edge, connection]);

		let filter_input_builder = FilterInputBuilder {
			context: self.context,
		};
		let filter = filter_input_builder.to_object::<T, Ty, F>();

		let order_input_builder = OrderInputBuilder {
			context: self.context,
		};
		let order = order_input_builder.to_object::<T, Ty>();
		self.inputs.extend(vec![filter, order]);

		let entity_query_field_builder = EntityQueryFieldBuilder {
			context: self.context,
		};
		let query = entity_query_field_builder.to_field::<T, Ty, F>();
		self.queries.push(query);
	}

	pub fn register_entity_methods<T, A>(&mut self)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let basic_entity_object = entity_object_builder.basic_to_object::<T, Ty>();
		self.outputs.push(basic_entity_object);

		let entity_input_builder = EntityInputBuilder {
			context: self.context,
		};

		let entity_insert_input_object = entity_input_builder.insert_input_object::<T, Ty>();
		let entity_update_input_object = entity_input_builder.update_input_object::<T, Ty>();
		self.inputs.extend(vec![entity_insert_input_object, entity_update_input_object]);

		// create one mutation
		let entity_create_one_mutation_builder = EntityCreateOneMutationBuilder {
			context: self.context,
		};
		let create_one_mutation = entity_create_one_mutation_builder.to_field::<T, A, Ty>();
		self.mutations.push(create_one_mutation);

		// create batch mutation
		let entity_create_batch_mutation_builder: EntityCreateBatchMutationBuilder =
			EntityCreateBatchMutationBuilder {
				context: self.context,
			};
		let create_batch_mutation = entity_create_batch_mutation_builder.to_field::<T, A, Ty>();
		self.mutations.push(create_batch_mutation);

		// update mutation
		let entity_update_mutation_builder = EntityUpdateMutationBuilder {
			context: self.context,
		};
		let update_mutation = entity_update_mutation_builder.to_field::<T, A, Ty, F>();
		self.mutations.push(update_mutation);

		let entity_delete_mutation_builder = EntityDeleteMutationBuilder {
			context: self.context,
		};
		let delete_mutation = entity_delete_mutation_builder.to_field::<T, A, Ty, F>();
		self.mutations.push(delete_mutation);
	}

	pub fn register_entity_dataloader_one_to_one<T, R, S>(mut self, _entity: T, spawner: S) -> Self
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		S: Fn(async_graphql::futures_util::future::BoxFuture<'static, ()>) -> R
			+ Send
			+ Sync
			+ Clone
			+ 'static,
	{
		self.builder = self.builder.data(OneToOneLoader::<T>::new(self.connection.clone()));
		self
	}

	pub fn register_entity_dataloader_one_to_many<T, R, S>(mut self, _entity: T, spawner: S) -> Self
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		S: Fn(async_graphql::futures_util::future::BoxFuture<'static, ()>) -> R
			+ Send
			+ Sync
			+ Clone
			+ 'static,
	{
		self.builder = self.builder.data(OneToManyLoader::<T>::new(self.connection.clone()));
		self
	}

	/// used to register a new enumeration to the builder context
	pub fn register_enumeration<A>(&mut self)
	where
		A: ActiveEnum,
	{
		let active_enum_builder = ActiveEnumBuilder {
			context: self.context,
		};
		let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
			context: self.context,
		};
		// let filter_types_map_helper = FilterTypesMapHelper {
		// 	context: self.context,
		// };

		let enumeration = active_enum_builder.enumeration::<A, E>();
		self.enumerations.push(enumeration);

		let filter_info = active_enum_filter_input_builder.filter_info::<A>();
		self.inputs.push(filter_info.generate_filter_input());
	}

	/// used to consume the builder context and generate a ready to be completed GraphQL builder
	pub fn builder(self) -> DynamicBuilder<Ty, E> {
		let query = self.query;
		let mutation = self.mutation;
		let builder = self.builder;

		// register queries
		let query = self.queries.into_iter().fold(query, |query, field| query.field(field));

		// register mutations
		let mutation =
			self.mutations.into_iter().fold(mutation, |mutation, field| mutation.field(field));

		// register entities to builder
		let builder =
			self.outputs.into_iter().fold(builder, |builder, entity| builder.register(entity));

		// register input types to builder
		let builder = self.inputs.into_iter().fold(builder, |builder, edge| builder.register(edge));

		// register enumerations
		let builder = self
			.enumerations
			.into_iter()
			.fold(builder, |builder, enumeration| builder.register(enumeration));

		// register input filters

		let builder = F::get_input_filters(self.context)
			.into_iter()
			.fold(builder, |builder, cur| builder.register(cur));

		builder
			.register(
				OrderByEnumBuilder {
					context: self.context,
				}
				.enumeration::<E>(),
			)
			.register(
				CursorInputBuilder {
					context: self.context,
				}
				.input_object(),
			)
			.register(
				CursorInputBuilder {
					context: self.context,
				}
				.input_object(),
			)
			.register(
				PageInputBuilder {
					context: self.context,
				}
				.input_object(),
			)
			.register(
				OffsetInputBuilder {
					context: self.context,
				}
				.input_object(),
			)
			.register(
				PaginationInputBuilder {
					context: self.context,
				}
				.input_object(),
			)
			.register(
				PageInfoObjectBuilder {
					context: self.context,
				}
				.to_object(),
			)
			.register(
				PaginationInfoObjectBuilder {
					context: self.context,
				}
				.to_object(),
			)
			.register(query)
			.register(mutation)
	}
}

pub trait RelationBuilder {
	fn get_relation<T, F>(
		&self,
		context: &'static crate::BuilderContext,
	) -> dynamic::prelude::Field<T>
	where
		T: TypeRefTrait,
		F: FilterTypeTrait;
}

#[macro_export]
macro_rules! register_entity {
	($builder:expr, $module_path:ident, $type_ref:ty, $filter_type:ty) => {
		$builder.register_entity::<$module_path::Entity>(
			<$module_path::RelatedEntity as sea_orm::Iterable>::iter()
				.map(|rel| {
					apy::RelationBuilder::get_relation::<$type_ref, $filter_type>(
						&rel,
						$builder.context,
					)
				})
				.collect(),
		);
		$builder =
			$builder.register_entity_dataloader_one_to_one($module_path::Entity, tokio::spawn);
		$builder =
			$builder.register_entity_dataloader_one_to_many($module_path::Entity, tokio::spawn);
		$builder.register_entity_methods::<$module_path::Entity, $module_path::ActiveModel>();
	};
}

#[macro_export]
macro_rules! register_entities {
    ($builder:expr, [$($module_paths:ident),+ $(,)?], $type_ref:ty, $filter_type:ty) => {
        $(apy::register_entity!($builder, $module_paths, $type_ref, $filter_type);)*
    };
}

#[macro_export]
macro_rules! register_entity_without_relation {
	($builder:expr, $module_path:ident) => {
		$builder.register_entity::<$module_path::Entity>(vec![]);
	};
}

#[macro_export]
macro_rules! register_entities_without_relation {
    ($builder:expr, [$($module_paths:ident),+ $(,)?]) => {
        $(apy::register_entity_without_relation!($builder, $module_paths);)*
    };
}
