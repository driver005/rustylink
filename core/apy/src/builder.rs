use crate::{
	ActiveEnumBuilder, ActiveEnumFilterInputBuilder, BuilderContext, ConnectionObjectBuilder,
	CursorInputBuilder, EdgeObjectBuilder, EntityCreateBatchMutationBuilder,
	EntityCreateOneMutationBuilder, EntityDeleteMutationBuilder, EntityInputBuilder,
	EntityObjectBuilder, EntityQueryFieldBuilder, EntityUpdateMutationBuilder, FilterInputBuilder,
	FilterTypesMapHelper, GraphQLFilterTypesMapHelper, OffsetInputBuilder, OneToManyLoader,
	OneToOneLoader, OrderByEnumBuilder, OrderInputBuilder, PageInfoObjectBuilder, PageInputBuilder,
	PaginationInfoObjectBuilder, PaginationInputBuilder, ProtoFilterTypesMapHelper,
};
use async_graphql::dataloader::DataLoader;
use dynamic::prelude::{
	GraphQLEnum, GraphQLField, GraphQLFieldFuture, GraphQLInputObject, GraphQLObject,
	GraphQLTypeRef, GraphQLValue, Proto, ProtoBuilder, ProtoEnum, ProtoField, ProtoMessage,
	ProtoService, Schema, SchemaBuilder,
};
use sea_orm::{ActiveEnum, ActiveModelTrait, EntityTrait, IntoActiveModel};

/// The Builder is used to create the Schema for GraphQL
///
/// You can populate it with the entities, enumerations of your choice
pub struct Builder {
	//GraphQL
	pub query: GraphQLObject,
	pub mutation: GraphQLObject,
	pub schema: SchemaBuilder,

	/// holds all output object types
	pub graphql_outputs: Vec<GraphQLObject>,

	/// holds all input object types
	pub graphql_inputs: Vec<GraphQLInputObject>,

	/// holds all enumeration types
	pub graphql_enumerations: Vec<GraphQLEnum>,

	/// holds all entities queries
	pub graphql_queries: Vec<GraphQLField>,

	/// holds all entities mutations
	pub graphql_mutations: Vec<GraphQLField>,

	//Proto
	pub proto: ProtoBuilder,
	pub service: ProtoService,
	pub message: ProtoMessage,

	/// holds all entities mutations
	pub proto_services: Vec<ProtoField>,

	/// holds all message types
	pub proto_messages: Vec<ProtoMessage>,

	/// holds all enumeration types
	pub proto_enumerations: Vec<ProtoEnum>,

	/// holds a copy to the database connection
	pub connection: sea_orm::DatabaseConnection,

	/// configuration for builder
	pub context: &'static BuilderContext,
}

impl Builder {
	/// Used to create a new Builder from the given configuration context
	pub fn new(context: &'static BuilderContext, connection: sea_orm::DatabaseConnection) -> Self {
		let query: GraphQLObject = GraphQLObject::new("Query");
		let mutation = GraphQLObject::new("Mutation").field(GraphQLField::new(
			"_ping",
			GraphQLTypeRef::named(GraphQLTypeRef::STRING),
			|_| GraphQLFieldFuture::new(async move { Ok(Some(GraphQLValue::from("pong"))) }),
		));
		let schema = Schema::build(query.type_name(), Some(mutation.type_name()), None);

		let message = ProtoMessage::new("Message");
		let service = ProtoService::new("Service");
		let proto = Proto::build(service.type_name());

		Self {
			query,
			mutation,
			schema,
			graphql_outputs: Vec::new(),
			graphql_inputs: Vec::new(),
			graphql_enumerations: Vec::new(),
			graphql_queries: Vec::new(),
			graphql_mutations: Vec::new(),
			message,
			service,
			proto,
			proto_enumerations: Vec::new(),
			proto_messages: Vec::new(),
			proto_services: Vec::new(),
			connection,
			context,
		}
	}

	/// used to register a new entity to the Builder context
	pub fn register_entity<T>(
		&mut self,
		graphql_relations: Vec<GraphQLField>,
		proto_relations: Vec<ProtoField>,
	) where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		self.register_graphql_entity::<T>(graphql_relations);
		self.register_proto_entity::<T>(proto_relations);
	}

	pub fn register_graphql_entity<T>(&mut self, relations: Vec<GraphQLField>)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let entity_object = relations
			.into_iter()
			.fold(entity_object_builder.to_object::<T>(), |entity_object, field| {
				entity_object.field(field)
			});

		let edge_object_builder = EdgeObjectBuilder {
			context: self.context,
		};
		let edge = edge_object_builder.to_object::<T>();

		let connection_object_builder = ConnectionObjectBuilder {
			context: self.context,
		};
		let connection = connection_object_builder.to_object::<T>();

		self.graphql_outputs.extend(vec![entity_object, edge, connection]);

		let filter_input_builder = FilterInputBuilder {
			context: self.context,
		};
		let filter = filter_input_builder.to_object::<T>();

		let order_input_builder = OrderInputBuilder {
			context: self.context,
		};
		let order = order_input_builder.to_object::<T>();
		self.graphql_inputs.extend(vec![filter, order]);

		let entity_query_field_builder = EntityQueryFieldBuilder {
			context: self.context,
		};
		let query = entity_query_field_builder.to_graphql_field::<T>();
		self.graphql_queries.push(query);
	}

	pub fn register_proto_entity<T>(&mut self, relations: Vec<ProtoField>)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let entity_object = relations
			.into_iter()
			.fold(entity_object_builder.to_message::<T>(), |entity_object, field| {
				entity_object.field(field)
			});

		let edge_object_builder = EdgeObjectBuilder {
			context: self.context,
		};
		let edge = edge_object_builder.to_message::<T>();

		let connection_object_builder = ConnectionObjectBuilder {
			context: self.context,
		};
		let connection = connection_object_builder.to_message::<T>();

		self.proto_messages.extend(vec![entity_object, edge, connection]);

		let filter_input_builder = FilterInputBuilder {
			context: self.context,
		};
		let filter = filter_input_builder.to_message::<T>();

		let order_input_builder = OrderInputBuilder {
			context: self.context,
		};
		let order = order_input_builder.to_message::<T>();
		self.proto_messages.extend(vec![filter, order]);

		let entity_query_field_builder = EntityQueryFieldBuilder {
			context: self.context,
		};
		let query = entity_query_field_builder.to_proto_field::<T>();
		self.proto_services.push(query);
	}

	pub fn register_entity_methods<T, A>(&mut self)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		self.register_entity_mutations::<T, A>();
		self.register_entity_service::<T, A>();
	}

	pub fn register_entity_mutations<T, A>(&mut self)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let basic_entity_object = entity_object_builder.basic_to_object::<T>();
		self.graphql_outputs.push(basic_entity_object);

		let entity_input_builder = EntityInputBuilder {
			context: self.context,
		};

		let entity_insert_input_object = entity_input_builder.insert_input_object::<T>();
		let entity_update_input_object = entity_input_builder.update_input_object::<T>();
		self.graphql_inputs.extend(vec![entity_insert_input_object, entity_update_input_object]);

		// create one mutation
		let entity_create_one_mutation_builder = EntityCreateOneMutationBuilder {
			context: self.context,
		};
		let create_one_mutation = entity_create_one_mutation_builder.to_graphql_field::<T, A>();
		self.graphql_mutations.push(create_one_mutation);

		// create batch mutation
		let entity_create_batch_mutation_builder: EntityCreateBatchMutationBuilder =
			EntityCreateBatchMutationBuilder {
				context: self.context,
			};
		let create_batch_mutation = entity_create_batch_mutation_builder.to_graphql_field::<T, A>();
		self.graphql_mutations.push(create_batch_mutation);

		// update mutation
		let entity_update_mutation_builder = EntityUpdateMutationBuilder {
			context: self.context,
		};
		let update_mutation = entity_update_mutation_builder.to_graphql_field::<T, A>();
		self.graphql_mutations.push(update_mutation);

		let entity_delete_mutation_builder = EntityDeleteMutationBuilder {
			context: self.context,
		};
		let delete_mutation = entity_delete_mutation_builder.to_graphql_field::<T, A>();
		self.graphql_mutations.push(delete_mutation);
	}

	pub fn register_entity_service<T, A>(&mut self)
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		<T as EntityTrait>::Model: IntoActiveModel<A>,
		A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let basic_entity_object = entity_object_builder.basic_to_message::<T>();
		self.proto_messages.push(basic_entity_object);

		let entity_input_builder = EntityInputBuilder {
			context: self.context,
		};

		let entity_insert_input_object = entity_input_builder.insert_message::<T>();
		let entity_update_input_object = entity_input_builder.update_message::<T>();
		self.proto_messages.extend(vec![entity_insert_input_object, entity_update_input_object]);

		// create one mutation
		let entity_create_one_mutation_builder = EntityCreateOneMutationBuilder {
			context: self.context,
		};
		let create_one_mutation = entity_create_one_mutation_builder.to_proto_field::<T, A>();
		self.proto_services.push(create_one_mutation);

		// create batch mutation
		let entity_create_batch_mutation_builder: EntityCreateBatchMutationBuilder =
			EntityCreateBatchMutationBuilder {
				context: self.context,
			};
		let create_batch_mutation = entity_create_batch_mutation_builder.to_proto_field::<T, A>();
		self.proto_services.push(create_batch_mutation);

		// update mutation
		let entity_update_mutation_builder = EntityUpdateMutationBuilder {
			context: self.context,
		};
		let update_mutation = entity_update_mutation_builder.to_proto_field::<T, A>();
		self.proto_services.push(update_mutation);

		let entity_delete_mutation_builder = EntityDeleteMutationBuilder {
			context: self.context,
		};
		let delete_mutation = entity_delete_mutation_builder.to_proto_field::<T, A>();
		self.proto_services.push(delete_mutation);
	}

	pub fn register_entity_dataloader_one_to_one<T, R, S>(mut self, _entity: T, spawner: S) -> Self
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		S: Fn(async_graphql::futures_util::future::BoxFuture<'static, ()>) -> R
			+ Send
			+ Sync
			+ 'static,
	{
		self.schema = self
			.schema
			.data(DataLoader::new(OneToOneLoader::<T>::new(self.connection.clone()), spawner));
		self
	}

	pub fn register_entity_dataloader_one_to_many<T, R, S>(mut self, _entity: T, spawner: S) -> Self
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		S: Fn(async_graphql::futures_util::future::BoxFuture<'static, ()>) -> R
			+ Send
			+ Sync
			+ 'static,
	{
		self.schema = self
			.schema
			.data(DataLoader::new(OneToManyLoader::<T>::new(self.connection.clone()), spawner));
		self
	}

	/// used to register a new enumeration to the builder context
	pub fn register_enumeration<A>(&mut self)
	where
		A: ActiveEnum,
	{
		self.register_graphql_enumeration::<A>();
		self.register_proto_enumeration::<A>();
	}

	pub fn register_graphql_enumeration<A>(&mut self)
	where
		A: ActiveEnum,
	{
		let active_enum_builder = ActiveEnumBuilder {
			context: self.context,
		};
		let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
			context: self.context,
		};
		let filter_types_map_helper = GraphQLFilterTypesMapHelper {
			context: self.context,
		};

		let enumeration = active_enum_builder.enumeration_graphql::<A>();
		self.graphql_enumerations.push(enumeration);

		let filter_info = active_enum_filter_input_builder.filter_info::<A>();
		self.graphql_inputs.push(filter_types_map_helper.generate_filter_input(&filter_info));
	}

	pub fn register_proto_enumeration<A>(&mut self)
	where
		A: ActiveEnum,
	{
		let active_enum_builder = ActiveEnumBuilder {
			context: self.context,
		};
		let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
			context: self.context,
		};
		let filter_types_map_helper = ProtoFilterTypesMapHelper {
			context: self.context,
		};

		let enumeration = active_enum_builder.enumeration_proto::<A>();
		self.proto_enumerations.push(enumeration);

		let filter_info = active_enum_filter_input_builder.filter_info::<A>();
		self.proto_messages.push(filter_types_map_helper.generate_filter_input(&filter_info));
	}

	/// used to consume the builder context and generate a ready to be completed GraphQL schema
	pub fn schema_builder(self) -> SchemaBuilder {
		let query = self.query;
		let mutation = self.mutation;
		let schema = self.schema;

		// register queries
		let query = self.graphql_queries.into_iter().fold(query, |query, field| query.field(field));

		// register mutations
		let mutation = self
			.graphql_mutations
			.into_iter()
			.fold(mutation, |mutation, field| mutation.field(field));

		// register entities to schema
		let schema =
			self.graphql_outputs.into_iter().fold(schema, |schema, entity| schema.register(entity));

		// register input types to schema
		let schema =
			self.graphql_inputs.into_iter().fold(schema, |schema, edge| schema.register(edge));

		// register enumerations
		let schema = self
			.graphql_enumerations
			.into_iter()
			.fold(schema, |schema, enumeration| schema.register(enumeration));

		// register input filters
		let filter_types_map_helper = GraphQLFilterTypesMapHelper {
			context: self.context,
		};
		let schema = filter_types_map_helper
			.get_input_filters()
			.into_iter()
			.fold(schema, |schema, cur| schema.register(cur));

		schema
			.register(
				OrderByEnumBuilder {
					context: self.context,
				}
				.enumeration_graphql(),
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

	/// used to consume the builder context and generate a ready to be completed Proto definition
	pub fn proto_builder(self) -> ProtoBuilder {
		let service = self.service;
		let schema = self.proto;

		// register services
		let service =
			self.proto_services.into_iter().fold(service, |service, field| service.field(field));

		// register messages to schema
		let schema =
			self.proto_messages.into_iter().fold(schema, |schema, entity| schema.register(entity));

		// register enumerations
		let schema = self
			.proto_enumerations
			.into_iter()
			.fold(schema, |schema, enumeration| schema.register(enumeration));

		// register input filters
		let filter_types_map_helper = ProtoFilterTypesMapHelper {
			context: self.context,
		};
		let schema = filter_types_map_helper
			.get_input_filters()
			.into_iter()
			.fold(schema, |schema, cur| schema.register(cur));

		schema
			.register(
				OrderByEnumBuilder {
					context: self.context,
				}
				.enumeration_proto(),
			)
			.register(
				CursorInputBuilder {
					context: self.context,
				}
				.message(),
			)
			.register(
				CursorInputBuilder {
					context: self.context,
				}
				.message(),
			)
			.register(
				PageInputBuilder {
					context: self.context,
				}
				.message(),
			)
			.register(
				OffsetInputBuilder {
					context: self.context,
				}
				.message(),
			)
			.register(
				PaginationInputBuilder {
					context: self.context,
				}
				.message(),
			)
			.register(
				PageInfoObjectBuilder {
					context: self.context,
				}
				.to_message(),
			)
			.register(
				PaginationInfoObjectBuilder {
					context: self.context,
				}
				.to_message(),
			)
			.register(service)
	}
}

pub trait RelationBuilder {
	fn get_graphql_relation(
		&self,
		context: &'static crate::BuilderContext,
	) -> dynamic::prelude::GraphQLField;
	fn get_proto_relation(
		&self,
		context: &'static crate::BuilderContext,
	) -> dynamic::prelude::ProtoField;
}

#[macro_export]
macro_rules! register_entity {
	($builder:expr, $module_path:ident) => {
		$builder.register_entity::<$module_path::Entity>(
			<$module_path::RelatedEntity as sea_orm::Iterable>::iter()
				.map(|rel| apy::RelationBuilder::get_graphql_relation(&rel, $builder.context))
				.collect(),
			<$module_path::RelatedEntity as sea_orm::Iterable>::iter()
				.map(|rel| apy::RelationBuilder::get_proto_relation(&rel, $builder.context))
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
    ($builder:expr, [$($module_paths:ident),+ $(,)?]) => {
        $(apy::register_entity!($builder, $module_paths);)*
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
