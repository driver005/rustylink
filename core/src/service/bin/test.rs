use actix_web::{App, HttpServer};
use dotenv::dotenv;
use dynamic::{graphql, http_proto};
use lazy_static::lazy_static;
use sea_orm::Database;
use service::query_root;
use std::env;

lazy_static! {
	static ref URL: String = env::var("URL").unwrap_or("localhost:8000".into());
	static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/graphql".into());
	static ref DATABASE_URL: String =
		env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
	static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT")
		.map_or(None, |data| Some(data.parse().expect("DEPTH_LIMIT is not a number")));
	static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
		.map_or(None, |data| { Some(data.parse().expect("COMPLEXITY_LIMIT is not a number")) });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	let database =
		Database::connect(&*DATABASE_URL).await.expect("Fail to initialize database connection");

	let schema = query_root::schema(&database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT).unwrap();

	let proto = query_root::proto(&database).unwrap();

	println!("Visit GraphQL Playground at http://{}{}", *URL, *ENDPOINT);

	HttpServer::new(move || {
		App::new()
			.configure(|cfg| http_proto(cfg, proto.clone()))
			.configure(|cfg| graphql(cfg, schema.clone()))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}
