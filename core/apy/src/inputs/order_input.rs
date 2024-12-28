use std::ops::Add;

use crate::{BuilderContext, EntityObjectBuilder};
use dynamic::prelude::{
	GraphQLInputObject, GraphQLInputValue, GraphQLTypeRef, GraphQLValueAccessor, ObjectAccessor,
	ObjectAccessors, ProtoField, ProtoMessage, ProtoTypeRef, ProtoValueAccessor, ValueAccessor,
};
use sea_orm::{EntityTrait, Iterable};

/// The configuration structure for OrderInputBuilder
pub struct OrderInputConfig {
	/// used to format OrderInput object name
	pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for OrderInputConfig {
	fn default() -> Self {
		OrderInputConfig {
			type_name: Box::new(|object_name: &str| -> String {
				format!("{object_name}OrderInput")
			}),
		}
	}
}

/// This builder produces the OrderInput object of a SeaORM entity
pub struct OrderInputBuilder {
	pub context: &'static BuilderContext,
}

impl OrderInputBuilder {
	/// used to get type name
	pub fn type_name(&self, object_name: &str) -> String {
		self.context.order_input.type_name.as_ref()(object_name)
	}

	/// used to get the OrderInput object of a SeaORM entity
	pub fn to_object<T>(&self) -> GraphQLInputObject
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};

		let object_name = entity_object_builder.type_name::<T>();
		let name = self.type_name(&object_name);

		T::Column::iter().fold(GraphQLInputObject::new(name), |object, column| {
			object.field(GraphQLInputValue::new(
				entity_object_builder.column_name::<T>(&column),
				GraphQLTypeRef::named(&self.context.order_by_enum.type_name),
			))
		})
	}

	/// used to get the OrderInput message of a SeaORM entity
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

		T::Column::iter().enumerate().fold(ProtoMessage::new(name), |object, (index, column)| {
			object.field(ProtoField::input(
				entity_object_builder.column_name::<T>(&column),
				index.add(1) as u32,
				ProtoTypeRef::named(&self.context.order_by_enum.type_name),
			))
		})
	}

	pub fn parse_object<'a, T, V>(
		&self,
		value: Option<V>,
	) -> Vec<(T::Column, sea_orm::sea_query::Order)>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		V: ValueAccessor<'a>,
	{
		match value {
			Some(value) => {
				let mut data = Vec::new();

				let entity_object = EntityObjectBuilder {
					context: self.context,
				};

				let order_by = value.object().unwrap();

				for col in T::Column::iter() {
					let column_name = entity_object.column_name::<T>(&col);

					match order_by.get_accessor() {
						ObjectAccessors::GraphQL(graphql) => self.order::<T, GraphQLValueAccessor>(
							&mut data,
							col,
							&graphql,
							&column_name,
						),
						ObjectAccessors::Proto(proto) => self.order::<T, ProtoValueAccessor>(
							&mut data,
							col,
							&proto,
							&column_name,
						),
					};
				}

				data
			}
			None => Vec::new(),
		}
	}
	fn order<'a, T, V>(
		&self,
		data: &mut Vec<(T::Column, sea_orm::sea_query::Order)>,
		col: T::Column,
		order_by: &'a V::ObjectAccessor,
		column_name: &str,
	) where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		V: ValueAccessor<'a> + 'a,
	{
		if let Some(order) = order_by.get(&column_name) {
			let order = order.enum_name().unwrap();

			let asc_variant = &self.context.order_by_enum.asc_variant;
			let desc_variant = &self.context.order_by_enum.desc_variant;

			if order.eq(asc_variant) {
				data.push((col, sea_orm::Order::Asc));
			} else if order.eq(desc_variant) {
				data.push((col, sea_orm::Order::Desc));
			} else {
				panic!("Cannot map enumeration")
			}
		}
	}
}
