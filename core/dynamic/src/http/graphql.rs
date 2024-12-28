use actix_web::{
	guard, services,
	web::{self, Data},
	App, HttpResponse, HttpServer, Result,
};
use async_graphql::{
	dynamic::*,
	http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

async fn index(schema: Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}

async fn playground() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(playground_source(GraphQLPlaygroundConfig::new("/"))))
}

pub fn graphql(cfg: &mut web::ServiceConfig, schema: Schema) {
	cfg.service(
		web::scope("/graphql")
			.app_data(Data::new(schema.clone()))
			.service(web::resource("").guard(guard::Post()).to(index))
			.service(web::resource("").guard(guard::Get()).to(playground)), // .service(web::redirect("", "/graphql/")),
	);
}
