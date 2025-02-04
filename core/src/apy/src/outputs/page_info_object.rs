use crate::BuilderContext;
use dynamic::prelude::*;

/// used to hold pages pagination info
#[derive(Clone, Debug)]
pub struct PageInfo {
	pub has_previous_page: bool,
	pub has_next_page: bool,
	pub start_cursor: Option<String>,
	pub end_cursor: Option<String>,
}

/// The configuration structure for PageInfoObjectBuilder
pub struct PageInfoObjectConfig {
	/// type name
	pub type_name: String,
	/// name for 'hasPreviousPage' field
	pub has_previous_page: String,
	/// name for 'hasNextPage' field
	pub has_next_page: String,
	/// name for 'startCursor' field
	pub start_cursor: String,
	/// name for 'endCursor' field
	pub end_cursor: String,
}

impl std::default::Default for PageInfoObjectConfig {
	fn default() -> Self {
		PageInfoObjectConfig {
			type_name: "PageInfo".into(),
			has_previous_page: "hasPreviousPage".into(),
			has_next_page: "hasNextPage".into(),
			start_cursor: "startCursor".into(),
			end_cursor: "endCursor".into(),
		}
	}
}

/// This builder produces the PageInfo object
/// that contains cursor pagination information
/// for a query
pub struct PageInfoObjectBuilder {
	pub context: &'static BuilderContext,
}

impl PageInfoObjectBuilder {
	/// used to get type name
	pub fn type_name(&self) -> String {
		self.context.page_info_object.type_name.clone()
	}

	/// used to get GraphQL message for PageInfo
	pub fn to_object(&self) -> Object {
		Object::new(&self.context.page_info_object.type_name, IO::Output)
			.field(Field::output(
				&self.context.page_info_object.has_previous_page,
				1u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::BOOLEAN),
					ProtoTypeRef::named_nn(ProtoTypeRef::BOOL),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(cursor_page_info.has_previous_page),
							ProtoValue::from(cursor_page_info.has_previous_page),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.page_info_object.has_next_page,
				2u32,
				TypeRef::new(
					GraphQLTypeRef::named_nn(GraphQLTypeRef::BOOLEAN),
					ProtoTypeRef::named_nn(ProtoTypeRef::BOOL),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
						Ok(Some(Value::new(
							GraphQLValue::from(cursor_page_info.has_next_page),
							ProtoValue::from(cursor_page_info.has_next_page),
						)))
					})
				},
			))
			.field(Field::output(
				&self.context.page_info_object.start_cursor,
				3u32,
				TypeRef::new(
					GraphQLTypeRef::named(GraphQLTypeRef::STRING),
					ProtoTypeRef::named(ProtoTypeRef::STRING),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
						let value = cursor_page_info
							.start_cursor
							.as_ref()
							.map(|v| {
								Value::new(
									GraphQLValue::from(v.as_str()),
									ProtoValue::from(v.as_str()),
								)
							})
							.or_else(|| Some(Value::null()));
						Ok(value)
					})
				},
			))
			.field(Field::output(
				&self.context.page_info_object.end_cursor,
				4u32,
				TypeRef::new(
					GraphQLTypeRef::named(GraphQLTypeRef::STRING),
					ProtoTypeRef::named(ProtoTypeRef::STRING),
				),
				|ctx| {
					FieldFuture::new(ctx.api_type.clone(), async move {
						let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
						let value = cursor_page_info
							.end_cursor
							.as_ref()
							.map(|v| {
								Value::new(
									GraphQLValue::from(v.as_str()),
									ProtoValue::from(v.as_str()),
								)
							})
							.or_else(|| Some(Value::null()));
						Ok(value)
					})
				},
			))
	}
}
