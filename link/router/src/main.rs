// // use axum::Router;

// // pub mod middleware;
// // pub mod routes;

// // #[tokio::main]
// // async fn main() {
// // 	// build our application with a route
// // 	let app = Router::new().merge(routes::schema::router());

// // 	// run our app with hyper, listening globally on port 3000
// // 	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
// // 	axum::serve(listener, app).await.unwrap();
// // }
// use tokio::signal;

// use std::{net::TcpListener, sync::mpsc, thread};

// use actix_web::{
// 	dev::ServerHandle, middleware, rt, web, App, HttpRequest, HttpResponse, HttpServer, Result,
// };
// use sea_orm::Database;

// async fn my_handler(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse> {
// 	log::info!("{:?}", req.uri().path());

// 	// let mut parser = Parser::new();

// 	// parser.parse_proto().unwrap();

// 	// let message = parser.get_message(&"GetUser".to_string()).unwrap();

// 	// let test = DynamicMessage::parse_from_bytes(&body).map_err(|e| {
// 	// 	eprintln!("Failed to parse: {:?}", e);
// 	// 	eprintln!("Data: {:?}", body);
// 	// 	actix_web::error::ErrorBadRequest(e.to_string())
// 	// })?;

// 	// println!("{:?}", test);

// 	Ok(HttpResponse::Ok().body("Processed successfully"))
// }

// async fn run_app(tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
// 	let db =
// 		Database::connect("postgres://postgres:postgres@localhost:5432/medusa-3-ls").await.unwrap();

// 	common::generate(db.clone()).await.unwrap();

// 	let app_state = web::Data::new(db);
// 	log::info!("starting HTTP server at http://localhost:8080");

// 	let listener = TcpListener::bind("127.0.0.1:8080")?;

// 	// srv is server controller type, `dev::Server`
// 	let server = HttpServer::new(move || {
// 		App::new()
// 			.wrap(middleware::Logger::default())
// 			.app_data(app_state.clone())
// 			// .configure(customer::configure)
// 			// .configure(customer_address::configure)
// 			// .configure(customer_group::configure)
// 			.service(web::resource("/User/Get").route(web::post().to(my_handler)))
// 		// .default_service(web::to(my_handler))
// 	})
// 	.listen_auto_h2c(listener)?
// 	.workers(2)
// 	.run();

// 	// Send server handle back to the main thread
// 	let _ = tx.send(server.handle());

// 	server.await
// }

// #[actix_web::main]
// async fn main() {
// 	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

// 	let (tx, rx) = mpsc::channel();

// 	// log::info!("spawning thread for server");

// 	thread::spawn(move || {
// 		let server_future = run_app(tx);
// 		rt::System::new().block_on(server_future)
// 	});

// 	let server_handle = rx.recv().unwrap();

// 	signal::ctrl_c().await.expect("Failed to listen for shutdown signal");

// 	// Send a stop signal to the server, waiting for it to exit gracefully
// 	// log::info!("stopping server");
// 	rt::System::new().block_on(server_handle.stop(true));
// }

use actix_web::{
	guard,
	http::Method,
	web::{self, Data},
	App, HttpRequest, HttpResponse, HttpServer, Result,
};
use async_graphql::{
	dynamic::*,
	http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use dynamic::{graphql, prelude::Proto};
use lazy_static::lazy_static;
use sea_orm::Database;
use std::{env, net::TcpListener};

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

async fn proto_index(proto: web::Data<Proto>, body: web::Bytes) -> Result<HttpResponse> {
	let bytes = proto.execute_once(body.to_vec(), "user").await.unwrap();

	println!("{:?}", bytes);

	Ok(HttpResponse::Ok().body(bytes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_test_writer().init();

	let database =
		Database::connect(&*DATABASE_URL).await.expect("Fail to initialize database connection");

	let schema =
		common_test::plugin::query_root::schema(&database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT)
			.unwrap();

	let proto = common_test::plugin::query_root::proto(&database).unwrap();

	println!("Visit GraphQL Playground at http://{}{}", *URL, *ENDPOINT);

	HttpServer::new(move || App::new().configure(|cfg| graphql(cfg, schema.clone())))
		.bind("127.0.0.1:8000")?
		.run()
		.await
}
