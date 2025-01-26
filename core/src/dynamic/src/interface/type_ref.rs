use crate::prelude::{GraphQLTypeRef, ProtoTypeRef};

#[derive(Debug)]
pub struct TypeRef {
	pub graphql: Option<GraphQLTypeRef>,
	pub proto: Option<ProtoTypeRef>,
}

impl TypeRef {
	pub fn new(graphql: GraphQLTypeRef, proto: ProtoTypeRef) -> Self {
		Self {
			graphql: Some(graphql),
			proto: Some(proto),
		}
	}
}
