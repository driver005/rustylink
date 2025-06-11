use super::{
	Arguments, ExecutionResult, Executor, FieldError, GraphQLType, GraphQLValue, GraphQLValueAsync,
	IntrospectionMode, MetaType, NodeInfo, NodeType, Registry, Scalar, TYPE_REGISTRY, Type,
	ValidationMode,
};
use crate::{BoxResolverFn, ContextBase, Data, FieldFuture, ResolverContext, SchemaError, Value};
use actix_web::HttpResponse;
use futures::{FutureExt, future::BoxFuture};
use juniper::{
	EmptySubscription, RootNode,
	http::{GraphQLRequest, GraphQLResponse},
};
use std::{any::Any, collections::BTreeMap, fmt::Debug, sync::Arc};

pub struct NodeWrapper {}

impl NodeWrapper {
	pub fn new() -> Self {
		Self {}
	}
}

#[async_trait::async_trait]
impl GraphQLType<Value> for NodeWrapper {
	fn name(info: &Self::TypeInfo) -> Option<&str> {
		Some(info.type_name())
	}

	fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, Value>) -> MetaType<'r, Value>
	where
		Value: 'r,
	{
		if info.type_name().contains("Empty") {
			return registry.build_object_type::<Self>(info, &vec![]).into_meta();
		}
		match TYPE_REGISTRY.get(info.type_name()) {
			Some(obj) => obj.meta(registry),
			None => {
				panic!(
					"`{}` with name `{}` not found",
					info.node_type.type_name(),
					info.type_name()
				);
			}
		}
	}
}

impl GraphQLValue<Value> for NodeWrapper {
	type Context = ContextBase;
	type TypeInfo = NodeInfo;
	fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
		Some(info.type_name())
	}

	fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
		info.type_name().to_string()
	}
}

impl GraphQLValueAsync<Value> for NodeWrapper
where
	Self::TypeInfo: Sync,
	Self::Context: Sync,
{
	fn resolve_field_async<'a>(
		&'a self,
		info: &'a Self::TypeInfo,
		field_name: &'a str,
		arguments: &'a Arguments<Value>,
		executor: &'a Executor<Self::Context, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		async move {
			match TYPE_REGISTRY.get(info.type_name()) {
				Some(obj) => obj.resolve(field_name, arguments, executor).await,
				None => Err(FieldError::from(format!(
					"`{}` with name `{}` not found",
					info.node_type.type_name(),
					info.type_name()
				))),
			}
		}
		.boxed()
	}
}

pub type Root = RootNode<'static, NodeWrapper, NodeWrapper, EmptySubscription<ContextBase>, Value>;

/// Dynamic schema builder
pub struct SchemaBuilder {
	query_type: NodeInfo,
	mutation_type: Option<NodeInfo>,
	subscription_type: Option<NodeInfo>,
	pub(crate) data: Data,
	validation_mode: ValidationMode,
	recursive_depth: usize,
	max_directives: Option<usize>,
	complexity: Option<usize>,
	depth: Option<usize>,
	enable_suggestions: bool,
	introspection_mode: IntrospectionMode,
	enable_federation: bool,
	entity_resolver: Option<BoxResolverFn>,
}

impl SchemaBuilder {
	/// Register a GraphQL type
	#[must_use]
	pub fn register(self, ty: impl Into<Type>) -> Self {
		let ty = ty.into();
		TYPE_REGISTRY.add(ty.name().to_owned(), ty);
		self
	}

	/// Add a global data that can be accessed in the `Schema`. You access it
	/// with `Context::data`.
	#[must_use]
	pub fn data<D: Any + Send + Sync>(mut self, data: D) -> Self {
		self.data.insert(data);
		self
	}

	/// Set the maximum complexity a query can have. By default, there is no
	/// limit.
	#[must_use]
	pub fn limit_complexity(mut self, complexity: usize) -> Self {
		self.complexity = Some(complexity);
		self
	}

	/// Set the maximum depth a query can have. By default, there is no limit.
	#[must_use]
	pub fn limit_depth(mut self, depth: usize) -> Self {
		self.depth = Some(depth);
		self
	}

	/// Set the maximum recursive depth a query can have. (default: 32)
	///
	/// If the value is too large, stack overflow may occur, usually `32` is
	/// enough.
	#[must_use]
	pub fn limit_recursive_depth(mut self, depth: usize) -> Self {
		self.recursive_depth = depth;
		self
	}

	/// Set the maximum number of directives on a single field. (default: no
	/// limit)
	pub fn limit_directives(mut self, max_directives: usize) -> Self {
		self.max_directives = Some(max_directives);
		self
	}

	/// Set the validation mode, default is `ValidationMode::Strict`.
	#[must_use]
	pub fn validation_mode(mut self, validation_mode: ValidationMode) -> Self {
		self.validation_mode = validation_mode;
		self
	}

	/// Disable field suggestions.
	#[must_use]
	pub fn disable_suggestions(mut self) -> Self {
		self.enable_suggestions = false;
		self
	}

	/// Disable introspection queries.
	#[must_use]
	pub fn disable_introspection(mut self) -> Self {
		self.introspection_mode = IntrospectionMode::Disabled;
		self
	}

	/// Only process introspection queries, everything else is processed as an
	/// error.
	#[must_use]
	pub fn introspection_only(mut self) -> Self {
		self.introspection_mode = IntrospectionMode::IntrospectionOnly;
		self
	}

	/// Enable federation, which is automatically enabled if the Query has least
	/// one entity definition.
	#[must_use]
	pub fn enable_federation(mut self) -> Self {
		self.enable_federation = true;
		self
	}

	/// Set the entity resolver for federation
	pub fn entity_resolver<F>(self, resolver_fn: F) -> Self
	where
		F: for<'b> Fn(ResolverContext<'b>) -> FieldFuture<'b> + Send + Sync + 'static,
	{
		Self {
			entity_resolver: Some(Box::new(resolver_fn)),
			..self
		}
	}

	/// Consumes this builder and returns a schema.
	pub fn finish(self) -> Result<Schema, SchemaError> {
		// create system scalars
		for ty in ["Int", "Float", "Boolean", "String", "ID"] {
			TYPE_REGISTRY.add(ty.to_string(), Scalar::new(ty).into());
		}

		let inner = SchemaInner {
			root_node: Root::new_with_info(
				NodeWrapper::new(),
				NodeWrapper::new(),
				EmptySubscription::new(),
				self.query_type,
				match self.mutation_type {
					Some(ty) => ty,
					None => NodeInfo::new("EmptyMutation".to_string(), NodeType::Mutation),
				},
				(),
			),
			data: Arc::new(self.data),
		};
		Ok(Schema(Arc::new(inner)))
	}
}

/// Dynamic GraphQL schema.
///
/// Cloning a schema is cheap, so it can be easily shared.
#[derive(Clone)]
pub struct Schema(pub(crate) Arc<SchemaInner>);

impl Debug for Schema {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Schema").finish()
	}
}

pub struct SchemaInner {
	root_node: Root,
	pub(crate) data: Arc<Data>,
}

impl Schema {
	/// Create a schema builder
	pub fn build(query: &str, mutation: Option<&str>, subscription: Option<&str>) -> SchemaBuilder {
		#[cfg(test)]
		TYPE_REGISTRY.clear();

		SchemaBuilder {
			query_type: NodeInfo::new(query.to_string(), NodeType::Query),
			mutation_type: mutation
				.map(|mutation| NodeInfo::new(mutation.to_string(), NodeType::Mutation)),
			subscription_type: subscription.map(|subscription| {
				NodeInfo::new(subscription.to_string(), NodeType::Subscription)
			}),
			data: Default::default(),
			validation_mode: ValidationMode::Strict,
			recursive_depth: 32,
			max_directives: None,
			complexity: None,
			depth: None,
			enable_suggestions: true,
			introspection_mode: IntrospectionMode::Enabled,
			entity_resolver: None,
			enable_federation: false,
		}
	}

	/// Execute a GraphQL query.
	pub async fn execute(&self, request: GraphQLRequest<Value>) -> HttpResponse {
		let mut ctx = ContextBase::new(crate::ApiType::GraphQL);

		ctx.execute_data = Some(self.0.data.clone());

		let res = request.execute(&self.0.root_node, &ctx).await;

		HttpResponse::Ok().json(res)
	}

	#[cfg(test)]
	pub(crate) async fn executer(&self, request: GraphQLRequest<Value>) -> GraphQLResponse<Value> {
		let mut ctx = ContextBase::new(crate::ApiType::GraphQL);

		ctx.execute_data = Some(self.0.data.clone());

		request.execute(&self.0.root_node, &ctx).await
	}

	// /// Execute a GraphQL subscription with session data.
	// pub fn execute_stream_with_session_data(
	// 	&self,
	// 	request: impl Into<DynamicRequest>,
	// 	session_data: Arc<Data>,
	// ) -> impl Stream<Item = Response> + Send + Unpin + 'static {
	// 	let schema = self.clone();
	// 	let request = request.into();
	// 	let extensions = self.create_extensions(session_data.clone());

	// 	let stream = {
	// 		let extensions = extensions.clone();

	// 		async_stream::stream! {
	// 			let subscription = match schema.subscription_root() {
	// 				Ok(subscription) => subscription,
	// 				Err(err) => {
	// 					yield Response::from_errors(vec![err]);
	// 					return;
	// 				}
	// 			};

	// 			let (env, _) = match prepare_request(
	// 				extensions,
	// 				request.inner,
	// 				session_data,
	// 				&schema.0.env.registry,
	// 				schema.0.validation_mode,
	// 				schema.0.recursive_depth,
	// 				schema.0.max_directives,
	// 				schema.0.complexity,
	// 				schema.0.depth,
	// 			)
	// 			.await {
	// 				Ok(res) => res,
	// 				Err(errors) => {
	// 					yield Response::from_errors(errors);
	// 					return;
	// 				}
	// 			};

	// 			if env.operation.node.ty != OperationType::Subscription {
	// 				yield schema.execute_once(env, &request.root_value, None).await;
	// 				return;
	// 			}

	// 			let ctx = env.create_context(
	// 				&schema.0.env,
	// 				None,
	// 				&env.operation.node.selection_set,
	// 				None,
	// 			);
	// 			let mut streams = Vec::new();
	// 			subscription.collect_streams(&schema, &ctx, &mut streams, &request.root_value);

	// 			let mut stream = futures_util::stream::select_all(streams);
	// 			while let Some(resp) = stream.next().await {
	// 				yield resp;
	// 			}
	// 		}
	// 	};
	// 	extensions.subscribe(stream.boxed())
	// }

	// /// Execute a GraphQL subscription.
	// pub fn execute_stream(
	// 	&self,
	// 	request: impl Into<DynamicRequest>,
	// ) -> impl Stream<Item = Response> + Send + Unpin {
	// 	self.execute_stream_with_session_data(request, Default::default())
	// }

	// /// Returns the registry of this schema.
	// pub fn registry(&self) -> &Registry {
	// 	&self.0.registry
	// }
}
