use dynamic::prelude::{GraphQLResolverContext, ProtoResolverContext};
use std::collections::BTreeMap;

/// Entities and GraphQLField guards configuration.
/// The guards are used to control access to entities or fields.
pub struct GuardsConfig<T> {
	/// entity guards are executed before accessing an entity
	pub entity_guards: BTreeMap<String, T>,
	/// field guards are executed before accessing an entity field
	pub field_guards: BTreeMap<String, T>,
}

impl<T> Default for GuardsConfig<T> {
	fn default() -> Self {
		GuardsConfig {
			entity_guards: BTreeMap::new(), // Custom default behavior
			field_guards: BTreeMap::new(),  // Custom default behavior
		}
	}
}

/// guards are functions that receive the application context
pub type FnGuardGraphQL = Box<dyn Fn(&GraphQLResolverContext) -> GuardAction + Sync + Send>;
pub type FnGuardProto = Box<dyn Fn(&ProtoResolverContext) -> GuardAction + Sync + Send>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GuardAction {
	Block(Option<String>),
	Allow,
}
