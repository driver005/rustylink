// mod graphql;
mod proto;
mod thrift;

// pub use graphql::*;
pub use proto::*;
pub use thrift::*;

// Define MetaVisibleFn type
pub type MetaVisibleFn = fn() -> bool;

pub enum Registry {
	// GraphQL(GraphQLRegistry),
	Proto(ProtoRegistry),
	Thrift(ThriftRegistry),
}

impl Registry {
	pub fn build(&self) -> String {
		match self {
			// Registry::GraphQL(registry) => registry.export_sdl(SDLExportOptions::new()),
			Registry::Proto(registry) => registry.build(),
			Registry::Thrift(registry) => registry.build(),
		}
	}
}
