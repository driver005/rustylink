use axum::{
	async_trait,
	body::Body,
	extract::{FromRequestParts, Path, Request},
	http::StatusCode,
	response::Response,
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Debug)]
struct NameExtractor(String);

// Implement a custom extractor that extracts `name` from the route.
#[async_trait]
impl<S> FromRequestParts<S> for NameExtractor
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(
		parts: &mut axum::http::request::Parts,
		_: &S,
	) -> Result<Self, Self::Rejection> {
		let Path(name) = Path::from_request_parts(parts, &())
			.await
			.map_err(|_| (StatusCode::BAD_REQUEST, "Missing name in path"))?;

		Ok(NameExtractor(name))
	}
}

#[derive(Clone)]
pub struct NameMiddleware<S> {
	inner: S,
}

impl<S> NameMiddleware<S> {
	pub fn new(inner: S) -> Self {
		Self {
			inner,
		}
	}
}

impl<S, B> Service<Request<B>> for NameMiddleware<S>
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
			let name: Result<Path<(String, String)>, _> =
				Path::from_request_parts(&mut parts, &()).await;

			match name {
				Ok(Path((_, name))) => {
					println!("Name: {}", name);
					// Reconstruct the request and call the next service in the stack
					let req = Request::from_parts(parts, body);
					inner.call(req).await
				}
				Err(_) => {
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
pub struct NameMiddlewareLayer;

impl<S> Layer<S> for NameMiddlewareLayer {
	type Service = NameMiddleware<S>;

	fn layer(&self, inner: S) -> Self::Service {
		NameMiddleware::new(inner)
	}
}
