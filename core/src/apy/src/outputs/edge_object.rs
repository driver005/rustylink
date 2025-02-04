use crate::{BuilderContext, EntityObjectBuilder};
use dynamic::prelude::*;
use sea_orm::EntityTrait;
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

	/// used to get the Node message for a SeaORM entity
	pub fn to_object<T>(&self) -> Object
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let object_name = entity_object_builder.type_name::<T>();
		let name = self.type_name(&object_name);

		Object::new(name, IO::Output)
			.field(Field::output(
				&self.context.edge_object.cursor,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::STRING),
					ProtoTypeRef::named_nn(ProtoTypeRef::STRING),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(edge.cursor.as_str()),
							ProtoValue::from(edge.cursor.as_str()),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.edge_object.node,
				2u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(&object_name),
					ProtoTypeRef::named_nn(&object_name),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
						Ok(Some(FieldValue::borrowed_any(&edge.node)))
					})
				},
			))
	}
}
