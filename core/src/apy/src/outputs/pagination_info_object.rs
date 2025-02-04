use crate::BuilderContext;
use dynamic::prelude::*;

/// used to hold offset pagination info
#[derive(Clone, Debug)]
pub struct PaginationInfo {
	pub pages: u64,
	pub current: u64,
	pub offset: u64,
	pub total: u64,
}

/// The configuration structure for PaginationInfoObjectBuilder
pub struct PaginationInfoObjectConfig {
	/// type name
	pub type_name: String,
	/// name for 'pages' field
	pub pages: String,
	/// name for 'current' field
	pub current: String,
	/// name for 'offset' field
	pub offset: String,
	/// name for 'total' field
	pub total: String,
}

impl std::default::Default for PaginationInfoObjectConfig {
	fn default() -> Self {
		PaginationInfoObjectConfig {
			type_name: "PaginationInfo".into(),
			pages: "pages".into(),
			current: "current".into(),
			offset: "offset".into(),
			total: "total".into(),
		}
	}
}

/// This builder produces the PaginationInfo object
/// that contains page/offset pagination information
/// for a query
pub struct PaginationInfoObjectBuilder {
	pub context: &'static BuilderContext,
}

impl PaginationInfoObjectBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.pagination_info_object.type_name.clone()
	}

	/// used to get GraphQL message for PaginationInfo
	pub fn to_object(&self) -> Object {
		Object::new(&self.context.pagination_info_object.type_name, IO::Output)
			.field(Field::output(
				&self.context.pagination_info_object.pages,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
					ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let pagination_page_info =
							ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(pagination_page_info.pages),
							ProtoValue::from(pagination_page_info.pages),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.pagination_info_object.current,
				2u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
					ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let pagination_page_info =
							ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(pagination_page_info.current),
							ProtoValue::from(pagination_page_info.current),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.pagination_info_object.offset,
				3u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
					ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let pagination_page_info =
							ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(pagination_page_info.offset),
							ProtoValue::from(pagination_page_info.offset),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.pagination_info_object.total,
				4u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::INT),
					ProtoTypeRef::named_nn(ProtoTypeRef::UINT64),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let pagination_page_info =
							ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(pagination_page_info.total),
							ProtoValue::from(pagination_page_info.total),
						)))
					})
				},
			))
	}
}
