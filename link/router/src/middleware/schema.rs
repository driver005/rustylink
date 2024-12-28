use axum::{
	async_trait,
	body::Body,
	extract::{FromRequestParts, Path, Request},
	http::StatusCode,
	response::Response,
};
use futures::future::BoxFuture;
use models::schema::Schema;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Debug)]
struct SchemaExtractor(String);

// Implement a custom extractor that extracts `schema` from the route.
#[async_trait]
impl<S> FromRequestParts<S> for SchemaExtractor
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(
		parts: &mut axum::http::request::Parts,
		_: &S,
	) -> Result<Self, Self::Rejection> {
		let Path(schema) = Path::from_request_parts(parts, &())
			.await
			.map_err(|_| (StatusCode::BAD_REQUEST, "Missing schema in path"))?;

		Ok(SchemaExtractor(schema))
	}
}

#[derive(Clone)]
pub struct SchemaMiddleware<S> {
	inner: S,
}

impl<S> SchemaMiddleware<S> {
	pub fn new(inner: S) -> Self {
		Self {
			inner,
		}
	}
}

impl<S, B> Service<Request<B>> for SchemaMiddleware<S>
where
	S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
	S::Future: Send + 'static,
	B: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, req: Request<B>) -> Self::Future {
		// Clone the inner service to use in the future
		let mut inner = self.inner.clone();

		// Convert the request into parts
		let (mut parts, body) = req.into_parts();

		// Extract the schema from the path
		let future = async move {
			let schema: Result<Path<(String, String)>, _> =
				Path::from_request_parts(&mut parts, &()).await;

			match schema {
				Ok(Path((schema, _))) => {
					// Reconstruct the request and call the next service in the stack
					let req = Request::from_parts(parts, body);
					inner.call(req).await
				}
				Err(err) => {
					println!("Error: {}", err);
					// Return a 400 Bad Request if extraction fails
					Ok(Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::empty())
						.unwrap())
				}
			}
		};

		Box::pin(future)
	}
}

#[derive(Clone)]
pub struct SchemaMiddlewareLayer;

impl<S> Layer<S> for SchemaMiddlewareLayer {
	type Service = SchemaMiddleware<S>;

	fn layer(&self, inner: S) -> Self::Service {
		SchemaMiddleware::new(inner)
	}
}
