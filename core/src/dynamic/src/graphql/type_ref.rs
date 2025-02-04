pub use async_graphql::dynamic::TypeRef;

use crate::TypeRefTrait;

impl TypeRefTrait for TypeRef {
	fn named(type_name: impl Into<String>) -> Self {
		TypeRef::named(type_name)
	}

	fn named_nn(type_name: impl Into<String>) -> Self {
		TypeRef::named_nn(type_name)
	}

	fn named_list(type_name: impl Into<String>) -> Self {
		TypeRef::named_list(type_name)
	}

	fn named_nn_list(type_name: impl Into<String>) -> Self {
		TypeRef::named_nn_list(type_name)
	}

	fn named_list_nn(type_name: impl Into<String>) -> Self {
		TypeRef::named_list_nn(type_name)
	}

	fn named_nn_list_nn(type_name: impl Into<String>) -> Self {
		TypeRef::named_nn_list_nn(type_name)
	}

	fn type_name(&self) -> &str {
		self.type_name()
	}
}
