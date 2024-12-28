use dynamic::prelude::{
	GraphQLField, GraphQLFieldFuture, GraphQLFieldValue, GraphQLObject, GraphQLTypeRef,
	GraphQLValue, ProtoField, ProtoFieldFuture, ProtoFieldValue, ProtoMessage, ProtoTypeRef,
	ProtoValue,
};
use sea_orm::EntityTrait;

use crate::{BuilderContext, EntityObjectBuilder};
/// used to represent a data Edge for GraphQL pagination
#[derive(Clone, Debug)]
pub struct Edge<T>
where
	T: EntityTrait,
	<T as EntityTrait>::Model: Sync,
{
	/// cursor string
	pub cursor: String,

	/// data
	pub node: T::Model,
}

/// The configuration structure for EdgeObjectBuilder
pub struct EdgeObjectConfig {
	/// used to format the type name of the object
	pub type_name: crate::SimpleNamingFn,
	/// name for 'cursor' field
	pub cursor: String,
	/// name for 'node' field
	pub node: String,
}

impl std::default::Default for EdgeObjectConfig {
	fn default() -> EdgeObjectConfig {
		EdgeObjectConfig {
			type_name: Box::new(|object_name: &str| -> String { format!("{object_name}Edge") }),
			cursor: "cursor".into(),
			node: "node".into(),
		}
	}
}

/// This builder produces the Node object for a SeaORM entity
pub struct EdgeObjectBuilder {
	pub context: &'static BuilderContext,
}

impl EdgeObjectBuilder {
	/// used to get type name
	pub fn type_name(&self, object_name: &str) -> String {
		self.context.edge_object.type_name.as_ref()(object_name)
	}

	/// used to get the Node object for a SeaORM entity
	pub fn to_object<T>(&self) -> GraphQLObject
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let object_name = entity_object_builder.type_name::<T>();
		let name = self.type_name(&object_name);

		GraphQLObject::new(name)
			.field(GraphQLField::new(
				&self.context.edge_object.cursor,
				GraphQLTypeRef::named_nn(GraphQLTypeRef::STRING),
				|ctx| {
					GraphQLFieldFuture::new(async move {
						let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
						Ok(Some(GraphQLValue::from(edge.cursor.as_str())))
					})
				},
			))
			.field(GraphQLField::new(
				&self.context.edge_object.node,
				GraphQLTypeRef::named_nn(object_name),
				|ctx| {
					GraphQLFieldFuture::new(async move {
						let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
						Ok(Some(GraphQLFieldValue::borrowed_any(&edge.node)))
					})
				},
			))
	}

	/// used to get the Node message for a SeaORM entity
	pub fn to_message<T>(&self) -> ProtoMessage
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let object_name = entity_object_builder.type_name::<T>();
		let name = self.type_name(&object_name);

		ProtoMessage::new(name)
			.field(ProtoField::output(
				&self.context.edge_object.cursor,
				1u32,
				ProtoTypeRef::named_nn(ProtoTypeRef::STRING),
				|ctx| {
					ProtoFieldFuture::new(async move {
						let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
						Ok(Some(ProtoValue::from(edge.cursor.as_str())))
					})
				},
			))
			.field(ProtoField::output(
				&self.context.edge_object.node,
				2u32,
				ProtoTypeRef::named_nn(object_name),
				|ctx| {
					ProtoFieldFuture::new(async move {
						let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
						Ok(Some(ProtoFieldValue::borrowed_any(&edge.node)))
					})
				},
			))
	}
}
