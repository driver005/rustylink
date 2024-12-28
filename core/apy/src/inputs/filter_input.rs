use std::ops::Add;

use crate::builder_context::filter_types_map::FilterTypesMapHelper;
use crate::{
	BuilderContext, EntityObjectBuilder, GraphQLFilterTypesMapHelper, ProtoFilterTypesMapHelper,
};
use dynamic::prelude::{
	GraphQLInputObject, GraphQLInputValue, GraphQLTypeRef, ProtoField, ProtoMessage, ProtoTypeRef,
};
use sea_orm::{EntityTrait, Iterable};

/// The configuration structure for FilterInputBuilder
pub struct FilterInputConfig {
	/// the filter input type name formatter function
	pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for FilterInputConfig {
	fn default() -> Self {
		FilterInputConfig {
			type_name: Box::new(|object_name: &str| -> String {
				format!("{object_name}FilterInput")
			}),
		}
	}
}

/// This builder is used to produce the filter input object of a SeaORM entity
pub struct FilterInputBuilder {
	pub context: &'static BuilderContext,
}

impl FilterInputBuilder {
	/// used to get the filter input object name
	/// object_name is the name of the SeaORM Entity GraphQL object
	pub fn type_name(&self, object_name: &str) -> String {
		self.context.filter_input.type_name.as_ref()(object_name)
	}

	/// used to produce the filter input object of a SeaORM entity
	pub fn to_object<T>(&self) -> GraphQLInputObject
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let filter_types_map_helper = GraphQLFilterTypesMapHelper {
			context: self.context,
		};

		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let entity_name = entity_object_builder.type_name::<T>();
		let filter_name = self.type_name(&entity_name);

		let object =
			T::Column::iter().fold(GraphQLInputObject::new(&filter_name), |object, column| {
				match filter_types_map_helper.get_column_filter_input_value::<T>(&column, 0) {
					Some(field) => object.field(field),
					None => object,
				}
			});

		object
			.field(GraphQLInputValue::new("and", GraphQLTypeRef::named_nn_list(&filter_name)))
			.field(GraphQLInputValue::new("or", GraphQLTypeRef::named_nn_list(&filter_name)))
	}

	/// used to produce the filter input message of a SeaORM entity
	pub fn to_message<T>(&self) -> ProtoMessage
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let filter_types_map_helper = ProtoFilterTypesMapHelper {
			context: self.context,
		};

		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let entity_name = entity_object_builder.type_name::<T>();
		let filter_name = self.type_name(&entity_name);

		let object = T::Column::iter().enumerate().fold(
			ProtoMessage::new(&filter_name),
			|object, (index, column)| match filter_types_map_helper
				.get_column_filter_input_value::<T>(&column, index.add(1) as u32)
			{
				Some(field) => object.field(field),
				None => object,
			},
		);

		let length = object.field_len();

		object
			.field(ProtoField::input(
				"and",
				length.add(2) as u32,
				ProtoTypeRef::named_nn_list(&filter_name),
			))
			.field(ProtoField::input(
				"or",
				length.add(3) as u32,
				ProtoTypeRef::named_nn_list(&filter_name),
			))
	}
}
