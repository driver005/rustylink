use async_graphql::{
	Positioned, QueryEnv, QueryPathNode, SchemaEnv, dynamic::Directive, parser::types::SelectionSet,
};

use crate::ContextBase;

use super::Field;

pub struct ContextType<'a, I> {
	/// The current path node being resolved.
	pub path_node: Option<QueryPathNode<'a>>,
	/// If `true` means the current field is for introspection.
	pub(crate) is_for_introspection: bool,
	#[doc(hidden)]
	pub schema_env: &'a SchemaEnv,
	#[doc(hidden)]
	pub query_env: &'a QueryEnv,
	#[doc(hidden)]
	pub item: I,
}

// /// Context for `SelectionSet`
// pub type ContextSelectionSet<'a> = ContextBase<'a, ContextType<'a, &'a Positioned<SelectionSet>>>;

// /// Context object for resolve field
// pub type Context<'a> = ContextBase<'a, ContextType<'a, &'a Positioned<Field>>>;

// /// Context object for execute directive.
// pub type ContextDirective<'a> = ContextBase<'a, ContextType<'a, &'a Positioned<Directive>>>;

// pub type WarpperContext = ContextType<'static, &'static Positioned<Field>>;
