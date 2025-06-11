use super::{Error, Field, PCKNAME, Result, TYPE_REGISTRY, Type, well_known_types};
use crate::{
	ApiType, BoxResolverFn, Data, FieldFuture, ObjectAccessor, ProtoRegistry, ResolverContext,
	SchemaError, Value,
};
use binary::proto::{Decoder, Encoder};
use bytes::Bytes;
use prost_types::{FileDescriptorProto, FileDescriptorSet};
use std::{any::Any, borrow::Cow, collections::BTreeMap, fmt::Debug, sync::Arc};
// use std::{
// 	convert::Infallible,
// 	task::{Context, Poll},
// };
// // use tonic::body::Body;
// // use tonic::codegen::{
// // 	BoxFuture, Service,
// // 	http::{Request, Response},
// // };
// use tonic::server::NamedService;

// use futures::future::BoxFuture;
// use tonic::codegen::{
// 	Body, Service, StdError,
// 	http::{Request, Response},
// };

// #[derive(Clone)]
// pub struct WrapperService;

// impl NamedService for WrapperService {
// 	const NAME: &'static str = "test.Query"; // Trick reflection & grpcurl
// }

// impl<B> Service<Request<B>> for WrapperService
// where
// 	B: Body + std::marker::Send + 'static,
// 	B::Error: Into<StdError> + std::marker::Send + 'static,
// {
// 	type Response = Response<Empty<Bytes>>;
// 	type Error = Infallible;
// 	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

// 	fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
// 		Poll::Ready(Ok(()))
// 	}

// 	fn call(&mut self, req: Request<B>) -> Self::Future {
// 		let path = req.uri().path().to_string();
// 		println!("WrapperService called: {}", path);
// 		// println!("body: {:?}", req.body());

// 		Box::pin(async move {
// 			// Handle the specific gRPC method
// 			if path == "/test.Query/doWhile" {
// 				let response = Response::builder()
// 					.status(200)
// 					.header("content-type", "application/grpc")
// 					.header("grpc-status", "0")
// 					.body(Empty::new())
// 					.unwrap();
// 				Ok(response)
// 			} else {
// 				// Return a 404 or unimplemented status instead of an error
// 				let response = Response::builder()
// 					.status(404)
// 					.header("content-type", "application/grpc")
// 					.header("grpc-status", "12") // UNIMPLEMENTED
// 					.body(Empty::new())
// 					.unwrap();
// 				Ok(response)
// 			}
// 		})

// 		// Box::pin(async move {
// 		// 	// Create a response with an empty body
// 		// 	let response =
// 		// 		Response::builder().status(200).body(http_body_util::Empty::new()).unwrap();
// 		// 	Ok(response)
// 		// })
// 		// Box::pin(async move { Ok(Response::new(body)) })
// 	}
// }

/// Dynamic schema builder
pub struct ProtoBuilder {
	pub data: Data,
	services: Vec<String>,
	entity_resolver: Option<BoxResolverFn>,
}

impl ProtoBuilder {
	/// Register a GraphQL type
	#[must_use]
	pub fn register(self, ty: impl Into<Type>) -> Self {
		let ty = ty.into();
		TYPE_REGISTRY.add(ty.type_name().to_owned(), ty);
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
		let mut registry = ProtoRegistry {
			types: Default::default(),
			proto_type: self.services[0].clone(),
			ignore_name_conflicts: Default::default(),
		};

		// // create system scalars
		// for ty in [
		// 	"double", "float", "int32", "int64", "uint32", "uint64", "sint32", "sint64", "fixed32",
		// 	"fixed64", "sfixed32", "sfixed64", "bool", "string", "bytes",
		// ] {
		// 	add_type(ty.to_string(), Type::Scalar(Scalar::new(ty)));
		// }

		// for ty in TYPES.read().unwrap().values() {
		// 	// ty.register(&mut registry)?;
		// }

		let inner = ProtoInner {
			registry,
			services: self.services,
			data: Arc::new(self.data),
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
	pub(crate) registry: ProtoRegistry,
	pub(crate) services: Vec<String>,
	pub(crate) data: Arc<Data>,
}

impl Proto {
	/// Create a schema builder
	pub fn build(services: Vec<&str>) -> ProtoBuilder {
		ProtoBuilder {
			services: services.into_iter().map(|s| s.to_string()).collect(),
			data: Default::default(),
			entity_resolver: None,
		}
	}

	// pub(crate) fn decode(
	// 	&self,
	// 	buf: &mut Vec<u8>,
	// 	parent: &Field,
	// ) -> Result<BTreeMap<Value, Value>> {
	// 	let mut decoder = Decoder::default();
	// 	let mut dst = vec![];
	// 	let mut arguments = BTreeMap::new();
	// 	decoder.decode(buf, &mut dst)?;

	// 	// for (tag, _, byt) in dst.drain(..) {
	// 	// 	let field = match parent.argument_by_tag(tag) {
	// 	// 		Some(field) => field,
	// 	// 		None => parent,
	// 	// 	};
	// 	// 	field.decode(&mut decoder, byt, tag, &mut arguments)?
	// 	// }

	// 	Ok(arguments)
	// }

	// pub(crate) fn to_object_accessor(
	// 	&self,
	// 	buf: &mut Vec<u8>,
	// 	field: &Field,
	// ) -> Result<ObjectAccessor> {
	// 	Ok(ObjectAccessor(Cow::Owned(self.decode(buf, field)?)))
	// }

	// pub async fn execute_once<'a>(
	// 	&self,
	// 	// ctx: &'a Context<'a>,
	// 	mut buf: Vec<u8>,
	// 	name: &str,
	// ) -> Result<Bytes> {
	// 	let mut ctx = crate::ContextBase::new(ApiType::Proto);

	// 	ctx.execute_data = Some(self.0.data.clone());

	// 	async move {
	// 		match TYPE_REGISTRY.get(&self.0.registry.proto_type) {
	// 			Some(inner) => match inner.as_message() {
	// 				Some(service) => match service.get_field(name) {
	// 					Some(field) => {
	// 						let arguments = self.to_object_accessor(&mut buf, field)?;

	// 						let res = field.collect(&ctx, &arguments, None).await?;

	// 						println!("res: {:#?}", res.0);
	// 						println!("val: {:#?}", res.1);

	// 						let encoder = Encoder::default();
	// 						let mut output = Vec::new();
	// 						field.encode(0u32, &encoder, &mut output, res.1)?;
	// 						Ok(Bytes::from(output))
	// 					}
	// 					None => Err(Error::new(format!("Method not found: {}", name))),
	// 				},
	// 				None => Err(Error::new(format!("Type '{}' not of type Service", name))),
	// 			},
	// 			None => Err(Error::new(format!("Service not found: {}", name))),
	// 		}
	// 	}
	// 	.await
	// }

	pub fn get_data(&self) -> Arc<Data> {
		self.0.data.clone()
	}

	/// Returns the registry of this schema.
	pub fn registry(&self) -> FileDescriptorSet {
		let mut file_set = FileDescriptorSet::default();

		let mut file = FileDescriptorProto::default();

		file.name = Some(format!("{}.proto", PCKNAME));
		file.package = Some(PCKNAME.to_string());

		file.dependency.push("google/protobuf/wrappers.proto".to_string());

		for ty in TYPE_REGISTRY.all().values() {
			ty.register(&mut file, self.0.services.iter().any(|s| s == ty.type_name()));
		}

		file_set.file.push(file);
		file_set.file.push(well_known_types());

		file_set
	}
}
