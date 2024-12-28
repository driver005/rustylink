use axum::Router;
use tower::ServiceBuilder;

use crate::middleware::schema::SchemaMiddlewareLayer;

pub fn router() -> Router {
	Router::new()
		.nest("/:schema", crate::routes::name::router())
		.layer(ServiceBuilder::new().layer(SchemaMiddlewareLayer))
}

async fn get_handler() -> &'static str {
	"Hello, world!"
}
