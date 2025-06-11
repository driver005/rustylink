use dotenv::dotenv;
use dynamic::{grpc_server, http_server};
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv().ok();

	let database =
		Database::connect(&*DATABASE_URL).await.expect("Fail to initialize database connection");

	let schema = query_root::schema(&database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT).unwrap();

	let proto = query_root::proto(&database).unwrap();

	let grpc_server = tokio::spawn(grpc_server(proto));

	let http_server = tokio::spawn(http_server(schema));

	// Await all tasks concurrently
	let _ = tokio::try_join!(grpc_server, http_server)?;

	println!("Both servers shut down cleanly.");
	Ok(())
}
