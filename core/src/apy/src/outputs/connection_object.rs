use dynamic::prelude::{
	Field, FieldFuture, FieldValue, FieldValueTrait, GraphQLTypeRef, Object, ProtoTypeRef, TypeRef,
	TypeRefTrait,
};
use sea_orm::EntityTrait;

use crate::{
	BuilderContext, Edge, EdgeObjectBuilder, EntityObjectBuilder, PageInfo, PaginationInfo,
};

/// used to represent a GraphQL Connection node for any Type
#[derive(Clone, Debug)]
pub struct Connection<T>
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
{
	/// cursor pagination info
	pub page_info: PageInfo,

	/// pagination info
	pub pagination_info: Option<PaginationInfo>,

	/// vector of data vector
	pub edges: Vec<Edge<T>>,
}

/// The configuration structure for ConnectionObjectBuilder
pub struct ConnectionObjectConfig {
	/// used to format the type name of the object
	pub type_name: crate::SimpleNamingFn,
	/// name for 'pageInfo' field
	pub page_info: String,
	/// name for 'paginationInfo' field
	pub pagination_info: String,
	/// name for 'edges' field
	pub edges: String,
	/// name for 'nodes' field
	pub nodes: String,
}

impl std::default::Default for ConnectionObjectConfig {
	fn default() -> Self {
		ConnectionObjectConfig {
			type_name: Box::new(|object_name: &str| -> String {
				format!("{object_name}Connection")
			}),
			page_info: "pageInfo".into(),
			pagination_info: "paginationInfo".into(),
			edges: "edges".into(),
			nodes: "nodes".into(),
		}
	}
}

/// This builder produces the Connection object for a SeaORM entity
pub struct ConnectionObjectBuilder {
	pub context: &'static BuilderContext,
}

impl ConnectionObjectBuilder {
	/// used to get type name
	pub fn type_name(&self, object_name: &str) -> String {
		self.context.connection_object.type_name.as_ref()(object_name)
	}

	/// used to get the Connection object for a SeaORM entity
	pub fn to_object<T>(&self) -> Object
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let edge_object_builder = EdgeObjectBuilder {
			context: self.context,
		};
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let object_name = entity_object_builder.type_name::<T>();
		let name = self.type_name(&object_name);

		Object::new(name)
			.field(Field::output(
				&self.context.connection_object.page_info,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(&self.context.page_info_object.type_name),
					ProtoTypeRef::named_nn(&self.context.page_info_object.type_name),
				),
				|ctx| {
					FieldFuture::new(async move {
						let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
						Ok(Some(FieldValue::borrowed_any(&connection.page_info)))
					})
				},
			))
			.field(Field::output(
				&self.context.connection_object.pagination_info,
				2u32,
				TypeRef::new(
					GraphQLTypeRef::named(&self.context.pagination_info_object.type_name),
					ProtoTypeRef::named(&self.context.pagination_info_object.type_name),
				),
				|ctx| {
					FieldFuture::new(async move {
						let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
						if let Some(value) =
							connection.pagination_info.as_ref().map(|v| FieldValue::borrowed_any(v))
						{
							Ok(Some(value))
						} else {
							Ok(FieldValue::NONE)
						}
					})
				},
			))
			.field(Field::output(
				&self.context.connection_object.nodes,
				3u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn_list_nn(&object_name),
					ProtoTypeRef::named_nn_list_nn(&object_name),
				),
				|ctx| {
					FieldFuture::new(async move {
						let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
						Ok(Some(FieldValue::list(
							connection
								.edges
								.iter()
								.map(|edge: &Edge<T>| FieldValue::borrowed_any(&edge.node)),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.connection_object.edges,
				4u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn_list_nn(edge_object_builder.type_name(&object_name)),
					ProtoTypeRef::named_nn_list_nn(edge_object_builder.type_name(&object_name)),
				),
				|ctx| {
					FieldFuture::new(async move {
						let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
						Ok(Some(FieldValue::list(
							connection
								.edges
								.iter()
								.map(|edge: &Edge<T>| FieldValue::borrowed_any(edge)),
						)))
					})
				},
			))
	}
}
