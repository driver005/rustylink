use crate::BuilderContext;
use dynamic::prelude::*;
use std::sync::Arc;

/// used to hold information about offset pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OffsetInput {
	pub offset: u64,
	pub limit: u64,
}

/// The configuration structure for OffsetInputBuilder
pub struct OffsetInputConfig {
	/// name of the object
	pub type_name: String,
	/// name for 'offset' field
	pub offset: String,
	/// name for 'limit' field
	pub limit: String,
}

impl OffsetInputConfig {
	pub fn type_name(&self) -> Arc<String> {
		Arc::new(self.type_name.clone())
	}
}

impl std::default::Default for OffsetInputConfig {
	fn default() -> Self {
		Self {
			type_name: "OffsetInput".into(),
			offset: "offset".into(),
			limit: "limit".into(),
		}
	}
}

/// This builder produces the offset pagination options input object
pub struct OffsetInputBuilder {
	pub context: &'static BuilderContext,
}

impl OffsetInputBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.offset_input.type_name.clone()
	}

	/// used to get offset pagination options message
	pub fn input_object<Ty>(&self) -> Object<Ty>
	where
		Ty: TypeRefTrait,
	{
		Object::new(&self.context.offset_input.type_name, IO::Input)
			.field(Field::input(&self.context.offset_input.limit, Ty::named_nn(Ty::UINT64)))
			.field(Field::input(&self.context.offset_input.offset, Ty::named_nn(Ty::UINT64)))
	}

	/// used to parse query input to offset pagination options struct
	pub fn parse_object<'a>(&self, object: &'a ObjectAccessor) -> SeaResult<OffsetInput> {
		let offset =
			object.get(&self.context.offset_input.offset).map_or_else(|| Ok(0), |v| v.uint64())?;

		let limit = match object.get(&self.context.offset_input.limit) {
			Some(value_accessor) => value_accessor,
			None => {
				return Err(SeaographyError::new(format!(
					"{} is a required argument but not provided.",
					self.context.offset_input.limit
				)));
			}
		}
		.uint64()?;

		Ok(OffsetInput {
			offset,
			limit,
		})
	}
}
