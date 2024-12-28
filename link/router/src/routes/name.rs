use axum::{extract::Path, routing::get, Router};
use protobuf::axum::Protobuf;
use tower::ServiceBuilder;

use crate::middleware::name::NameMiddlewareLayer;

#[derive(prost::Message)]
struct User {
	#[prost(string, tag = "1")]
	pub username: String,
}

pub fn router() -> Router {
	Router::new().route("/:name", get(get_handler))
	// .layer(ServiceBuilder::new().layer(NameMiddlewareLayer))
}

async fn get_handler(Path(name): Path<String>) -> Protobuf<User> {
	Protobuf(User {
		username: name,
	})
}
