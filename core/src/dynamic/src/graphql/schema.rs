use std::{any::Any, borrow::Cow, collections::BTreeMap, fmt::Debug, sync::Arc, sync::Mutex};

use crate::{
	BoxResolverFn, ContextBase, Data, FieldFuture, FieldValue, ObjectAccessor, ResolverContext,
	SchemaError, SeaResult, SeaographyError, Value,
};
use actix_web::HttpResponse;
use async_graphql::{
	InputType, Positioned, Request, ServerResult, Variables,
	parser::{
		parse_query,
		types::{Directive, ExecutableDocument, Field, Selection, SelectionSet},
	},
};
use futures::TryFutureExt;
use juniper::{
	EmptyMutation, EmptySubscription, GraphQLType, GraphQLValue, RootNode, http::GraphQLRequest,
};

use super::{
	Info, IntrospectionMode, Object, QueryWrapper, Registry, Scalar, ServerError, Subscription,
	Type, TypeRef, Union, ValidationMode, add_type, get_type,
};

pub type Root = RootNode<
	'static,
	QueryWrapper,
	EmptyMutation<ContextBase>,
	EmptySubscription<ContextBase>,
	Value,
>;

/// Dynamic schema builder
pub struct SchemaBuilder {
	query_type: Info,
	mutation_type: Option<String>,
	subscription_type: Option<String>,
	types: BTreeMap<String, Type>,
	pub data: Data,
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
		add_type(ty.name().to_owned(), ty);
		// self.types.insert(ty.name().to_owned(), ty);
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
			add_type(ty.to_string(), Scalar::new(ty).into());
		}

		let inner = SchemaInner {
			root_node: Root::new_with_info(
				QueryWrapper::new(),
				EmptyMutation::new(),
				EmptySubscription::new(),
				self.query_type,
				(),
				(),
			),
			// env: SchemaEnv(Arc::new(SchemaEnvInner {
			// 	registry,
			// 	data: Default::default(),
			// 	custom_directives: Default::default(),
			// })),
			// registry,
			data: Arc::new(self.data),
			recursive_depth: self.recursive_depth,
			max_directives: self.max_directives,
			complexity: self.complexity,
			depth: self.depth,
			validation_mode: self.validation_mode,
			entity_resolver: self.entity_resolver,
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
	// pub(crate) env: SchemaEnv,
	// pub(crate) registry: Registry,
	pub(crate) data: Arc<Data>,
	recursive_depth: usize,
	max_directives: Option<usize>,
	complexity: Option<usize>,
	depth: Option<usize>,
	validation_mode: ValidationMode,
	pub(crate) entity_resolver: Option<BoxResolverFn>,
}

impl Schema {
	/// Create a schema builder
	pub fn build(query: &str, mutation: Option<&str>, subscription: Option<&str>) -> SchemaBuilder {
		SchemaBuilder {
			query_type: Info::new(query.to_string()),
			mutation_type: mutation.map(ToString::to_string),
			subscription_type: subscription.map(ToString::to_string),
			types: Default::default(),
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

	// fn create_extensions(&self, session_data: Arc<async_graphql::Data>) -> Extensions {
	// 	Extensions::new(
	// 		// self.0.extensions.iter().map(|f| f.create()),
	// 		vec![].into_iter(),
	// 		self.0.env.clone(),
	// 		session_data,
	// 	)
	// }

	// fn query_root(&self) -> SeaResult<&Object> {
	// 	self.0.types.get(&self.0.registry.query_type).and_then(Type::as_object).ok_or_else(|| {
	// 		SeaographyError::AsyncGraphQLError(ServerError::new("Query root not found", None))
	// 	})
	// }

	// fn mutation_root(&self) -> SeaResult<&Object> {
	// 	self.0
	// 		.env
	// 		.registry
	// 		.mutation_type
	// 		.as_ref()
	// 		.and_then(|mutation_name| self.0.types.get(mutation_name))
	// 		.and_then(Type::as_object)
	// 		.ok_or_else(|| {
	// 			SeaographyError::AsyncGraphQLError(ServerError::new(
	// 				"Mutation root not found",
	// 				None,
	// 			))
	// 		})
	// }

	// fn subscription_root(&self) -> SeaResult<&Subscription> {
	// 	self.0
	// 		.env
	// 		.registry
	// 		.subscription_type
	// 		.as_ref()
	// 		.and_then(|subscription_name| self.0.types.get(subscription_name))
	// 		.and_then(Type::as_subscription)
	// 		.ok_or_else(|| {
	// 			SeaographyError::AsyncGraphQLError(ServerError::new(
	// 				"Subscription root not found",
	// 				None,
	// 			))
	// 		})
	// }

	// /// Returns SDL(Schema Definition Language) of this schema.
	// pub fn sdl(&self) -> String {
	// 	self.0.env.registry.export_sdl(Default::default())
	// }

	// /// Returns SDL(Schema Definition Language) of this schema with options.
	// pub fn sdl_with_options(&self, options: SDLExportOptions) -> String {
	// 	self.0.env.registry.export_sdl(options)
	// }

	// async fn execute_once(
	// 	&self,
	// 	env: QueryEnv,
	// 	root_value: &FieldValue<'static>,
	// 	// execute_data: Option<Data>,
	// ) -> Response {
	// 	// execute
	// 	let ctx = env.create_context(
	// 		&self.0.env,
	// 		None,
	// 		&env.operation.node.selection_set,
	// 		Default::default(),
	// 	);

	// 	let res = match &env.operation.node.ty {
	// 		OperationType::Query => {
	// 			async move { self.query_root() }
	// 				.and_then(|query_root| {
	// 					resolve_container(self, query_root, &ctx, root_value, false)
	// 				})
	// 				.await
	// 		}
	// 		OperationType::Mutation => {
	// 			async move { self.mutation_root() }
	// 				.and_then(|query_root| {
	// 					resolve_container(self, query_root, &ctx, root_value, true)
	// 				})
	// 				.await
	// 		}
	// 		OperationType::Subscription => {
	// 			Err(ServerError::new("Subscriptions are not supported on this transport.", None))
	// 		}
	// 	};

	// 	let mut resp = match res {
	// 		Ok(value) => Response::new(value.unwrap_or_default()),
	// 		Err(err) => Response::from_errors(vec![err]),
	// 	}
	// 	.http_headers(std::mem::take(&mut *env.http_headers.lock().unwrap()));

	// 	resp.errors.extend(std::mem::take(&mut *env.errors.lock().unwrap()));
	// 	resp
	// }
	// pub(crate) fn to_object_accessor(
	// 	&self,
	// 	buf: &mut Vec<u8>,
	// 	field: &Positioned<Field>,
	// ) -> SeaResult<ObjectAccessor> {
	// 	Ok(ObjectAccessor(Cow::Owned(
	// 		field
	// 			.node
	// 			.arguments
	// 			.iter()
	// 			.map(|(name, value)| {
	// 				ctx_field
	// 					.resolve_input_value(value.clone())
	// 					.map(|value| (name.node.clone(), value))
	// 			})
	// 			.collect::<ServerResult<IndexMap<Name, Value>>>()?,
	// 	)))
	// }

	/// Execute a GraphQL query.
	pub async fn execute(&self, request: GraphQLRequest<Value>) -> HttpResponse {
		let mut ctx = ContextBase::new(crate::ApiType::GraphQL);

		ctx.execute_data = Some(self.0.data.clone());

		let res = request.execute(&self.0.root_node, &ctx).await;

		HttpResponse::Ok().json(res)
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
