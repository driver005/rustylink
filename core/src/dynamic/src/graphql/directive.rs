use super::MetaDirectiveInvocation;
use crate::Value;
use std::collections::BTreeMap;

/// A GraphQL directive
#[derive(Debug, Clone)]
pub struct Directive {
	pub name: String,
	pub args: BTreeMap<Value, Value>,
}

impl Directive {
	/// Create a directive usage
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			args: BTreeMap::default(),
		}
	}

	/// Add an argument to the directive
	#[inline]
	pub fn argument(mut self, name: Value, value: Value) -> Self {
		self.args.insert(name, value);
		self
	}
}

// impl From<Directive> for MetaDirectiveInvocation {
// 	fn from(directive: Directive) -> Self {
// 		Self {
// 			name: directive.name,
// 			args: directive.args,
// 		}
// 	}
// }

// pub fn to_meta_directive_invocation(directives: Vec<Directive>) -> Vec<MetaDirectiveInvocation> {
// 	directives.into_iter().map(MetaDirectiveInvocation::from).collect()
// }
