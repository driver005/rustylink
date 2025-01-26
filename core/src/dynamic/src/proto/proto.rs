use super::{
	add_type, get_type, BoxResolverFn, Error, Field, FieldFuture, ObjectAccessor, ResolverContext,
	Result, Scalar, Type, Value, TYPES,
};
use crate::{context::Data, Registry, SchemaError};
use async_graphql::Name;
use binary::proto::Decoder;
use bytes::Bytes;
use indexmap::IndexMap;
use std::{any::Any, borrow::Cow, fmt::Debug, sync::Arc};

/// Dynamic schema builder
pub struct ProtoBuilder {
	data: Data,
	proto_type: String,
	entity_resolver: Option<BoxResolverFn>,
}

impl ProtoBuilder {
	/// Register a GraphQL type
	#[must_use]
	pub fn register(self, ty: impl Into<Type>) -> Self {
		let ty = ty.into();
		add_type(ty.name().to_owned(), ty);
		self
	}

	/// Add a global data that can be accessed in the `Proto`. You access it
	/// with `Context::data`.
	#[must_use]
	pub fn data<D: Any + Send + Sync>(mut self, data: D) -> Self {
		self.data.insert(data);
		self
	}

	/// Set the entity resolver for federation
	pub fn entity_resolver<F>(self, resolver_fn: F) -> Self
	where
		F: for<'a> Fn(ResolverContext<'a>) -> FieldFuture<'a> + Send + Sync + 'static,
	{
		Self {
			entity_resolver: Some(Box::new(resolver_fn)),
			..self
		}
	}

	pub fn finish(self) -> Result<Proto, SchemaError> {
		let mut registry = Registry {
			types: Default::default(),
			proto_type: self.proto_type,
			ignore_name_conflicts: Default::default(),
		};

		// create system scalars
		for ty in [
			"double", "float", "int32", "int64", "uint32", "uint64", "sint32", "sint64", "fixed32",
			"fixed64", "sfixed32", "sfixed64", "bool", "string", "bytes",
		] {
			add_type(ty.to_string(), Type::Scalar(Scalar::new(ty)));
		}

		for ty in TYPES.read().unwrap().values() {
			ty.register(&mut registry)?;
		}

		let inner = ProtoInner {
			registry,
			data: self.data,
		};
		// inner.check()?;
		Ok(Proto(Arc::new(inner)))
	}
}

/// Dynamic GraphQL schema.
///
/// Cloning a schema is cheap, so it can be easily shared.
#[derive(Clone)]
pub struct Proto(pub(crate) Arc<ProtoInner>);

impl Debug for Proto {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Proto").finish()
	}
}

pub struct ProtoInner {
	pub(crate) registry: Registry,
	pub(crate) data: Data,
}

impl Proto {
	/// Create a schema builder
	pub fn build(proto: &str) -> ProtoBuilder {
		ProtoBuilder {
			proto_type: proto.to_string(),
			data: Default::default(),
			entity_resolver: None,
		}
	}

	/// Returns SDL(Proto Definition Language) of this schema.
	pub fn sdl(&self) -> String {
		self.registry().build_proto()
	}

	pub(crate) fn decode(
		&self,
		buf: &mut Vec<u8>,
		parent: &Field,
	) -> Result<IndexMap<Name, Value>> {
		let mut decoder = Decoder::default();
		let mut dst = vec![];
		let mut arguments = IndexMap::new();
		println!("name: {}", parent.type_name());
		decoder.decode(buf, &mut dst)?;

		for (tag, _, byt) in dst.drain(..) {
			let field = match parent.argument_by_tag(tag) {
				Some(field) => field,
				None => parent,
			};
			field.decode(&mut decoder, byt, tag, &mut arguments)?
		}

		Ok(arguments)
	}

	pub(crate) fn to_object_accessor(
		&self,
		buf: &mut Vec<u8>,
		field: &Field,
	) -> Result<ObjectAccessor> {
		Ok(ObjectAccessor(Cow::Owned(self.decode(buf, field)?)))
	}

	pub async fn execute_once<'a>(
		&self,
		// ctx: &'a Context<'a>,
		mut buf: Vec<u8>,
		name: &str,
	) -> Result<Bytes> {
		let mut ctx = crate::ContextBase::new();

		ctx.execute_data = Some(&self.0.data);

		async move {
			match get_type(&self.registry().proto_type) {
				Some(inner) => match inner.as_service() {
					Some(service) => match service.get_field(name) {
						Some(field) => {
							let arguments = self.to_object_accessor(&mut buf, field)?;

							Ok(Bytes::from(field.execute(&ctx, &arguments).await?))
						}
						None => Err(Error::new(format!("Method not found: {}", name))),
					},
					None => Err(Error::new(format!("Type '{}' not of type Service", name))),
				},
				None => Err(Error::new(format!("Service not found: {}", name))),
			}
		}
		.await
	}
	/// Execute a GraphQL query.
	// pub async fn execute(&self, request: impl Into<DynamicRequest>) -> Response {
	// 	let request = request.into();
	// 	let extensions = self.create_extensions(Default::default());
	// 	let request_fut = {
	// 		let extensions = extensions.clone();
	// 		async move {
	// 			match prepare_request(
	// 				extensions,
	// 				request.inner,
	// 				Default::default(),
	// 				&self.0.env.registry,
	// 				self.0.validation_mode,
	// 				self.0.recursive_depth,
	// 				self.0.complexity,
	// 				self.0.depth,
	// 			)
	// 			.await
	// 			{
	// 				Ok((env, cache_control)) => {
	// 					let f = {
	// 						|execute_data| {
	// 							let env = env.clone();
	// 							async move {
	// 								self.execute_once(env, &request.root_value, execute_data)
	// 									.await
	// 									.cache_control(cache_control)
	// 							}
	// 						}
	// 					};
	// 					env.extensions.execute(env.operation_name.as_deref(), f).await
	// 				}
	// 				Err(errors) => Response::from_errors(errors),
	// 			}
	// 		}
	// 	};
	// 	futures_util::pin_mut!(request_fut);
	// 	extensions.request(&mut request_fut).await
	// }

	/// Returns the registry of this schema.
	pub fn registry(&self) -> &Registry {
		&self.0.registry
	}
}
