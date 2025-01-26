use crate::prelude::{GraphQLValue, ProtoValue};

#[derive(Debug, Clone)]
pub struct Value {
	graphql: GraphQLValue,
	proto: ProtoValue,
}

impl Value {
	pub fn new(graphql: GraphQLValue, proto: ProtoValue) -> Self {
		Self {
			graphql,
			proto,
		}
	}

	pub const fn null() -> Self {
		Self {
			graphql: GraphQLValue::Null,
			proto: ProtoValue::Null,
		}
	}

	pub const fn graphql(graphql: GraphQLValue) -> Self {
		Self {
			graphql,
			proto: ProtoValue::Null,
		}
	}

	pub const fn proto(proto: ProtoValue) -> Self {
		Self {
			graphql: GraphQLValue::Null,
			proto,
		}
	}
}
