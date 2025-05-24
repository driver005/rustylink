use crate::{BuilderContext, EntityObjectBuilder, FilterTypeTrait, FilterTypesMapHelper};
use dynamic::prelude::*;
use sea_orm::{EntityTrait, Iterable};
use std::ops::Add;

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

	/// used to produce the filter input message of a SeaORM entity
	pub fn to_object<T, Ty, F>(&self) -> Object<Ty>
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
		Ty: TypeRefTrait,
		F: FilterTypeTrait,
	{
		let filter_types_map_helper = FilterTypesMapHelper {
			context: self.context,
		};

		let entity_object_builder = EntityObjectBuilder {
			context: self.context,
		};
		let entity_name = entity_object_builder.type_name::<T>();
		let filter_name = self.type_name(&entity_name);

		let object = T::Column::iter().enumerate().fold(
			Object::new(&filter_name, IO::Input),
			|object, (index, column)| match filter_types_map_helper
				.get_column_filter_input_value::<T, Ty, F>(&column, index.add(1) as u32)
			{
				Some(field) => object.field(field),
				None => object,
			},
		);

		let length = object.field_len();

		object
			.field(Field::input("and", length.add(2) as u32, Ty::named_nn_list(&filter_name)))
			.field(Field::input("or", length.add(3) as u32, Ty::named_nn_list(&filter_name)))
	}
}
