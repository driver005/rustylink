use crate::prelude::{GraphQLTypeRef, ProtoTypeRef};

#[derive(Debug)]
pub struct TypeRef {
	pub graphql: GraphQLTypeRef,
	pub proto: ProtoTypeRef,
}

impl TypeRef {
	pub fn new(graphql: GraphQLTypeRef, proto: ProtoTypeRef) -> Self {
		Self {
			graphql,
			proto,
		}
	}

	pub(crate) fn to_graphql(self) -> GraphQLTypeRef {
		self.graphql
	}

	pub(crate) fn to_proto(self) -> ProtoTypeRef {
		self.proto
	}
}
