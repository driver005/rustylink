use crate::{Value, graphql::Schema};
use actix_web::{
	App, HttpResponse, HttpServer, Result, guard,
	web::{self, Data},
};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

async fn index(schema: Data<Schema>, request: web::Json<GraphQLRequest<Value>>) -> HttpResponse {
	schema.execute(request.into_inner()).await
}

async fn playground() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(graphiql_source("/graphql", None)))
}

fn graphql(cfg: &mut web::ServiceConfig, schema: Schema) {
	cfg.service(
		web::scope("/graphql")
			.app_data(Data::new(schema.clone()))
			.service(web::resource("").guard(guard::Post()).to(index))
			.service(web::resource("").guard(guard::Get()).to(playground)), // .service(web::resource("/sdl").guard(guard::Get()).to(sdl)), // .service(web::redirect("", "/graphql/")),
	);
}

pub async fn http_server(schema: Schema) {
	println!("Visit GraphQL Playground at http://127.0.0.1:8000");
	HttpServer::new(move || App::new().configure(|cfg| graphql(cfg, schema.clone())))
		.bind("127.0.0.1:8000")
		.unwrap()
		.run()
		.await
		.unwrap();
}
