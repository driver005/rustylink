use service::App;
use service::handles::query_root::{Proto, Schema};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// dotenv().ok();

	// let database =
	// 	Database::connect(&*DATABASE_URL).await.expect("Fail to initialize database connection");

	// let schema = query_root::schema(&database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT).unwrap();

	// let proto = query_root::proto(&database).unwrap();

	// let grpc_server = tokio::spawn(grpc_server(proto));

	// let http_server = tokio::spawn(http_server(schema));

	// // Await all tasks concurrently
	// let _ = tokio::try_join!(grpc_server, http_server)?;

	// println!("Both servers shut down cleanly.");
	// Ok(())

	let app = App::from_config("./src/service/bin/example.toml").await?;

	app.add_grpc_service(Box::new(Proto {}))
		.add_http_service(Box::new(Schema::new()))
		.build()
		.await?;

	// let mut tasks = Vec::new();

	// for i in 0..5 {
	// 	let task = tokio::spawn(async move {
	// 		println!("Server {} running...", i);
	// 		Ok::<_, Box<dyn std::error::Error + Send + Sync>>(format!("Server {} done", i))
	// 	});
	// 	tasks.push(task);
	// }

	// let results = try_join_all(tasks).await?;

	// for r in results {
	// 	println!("{}", r?); // Unwrap each JoinHandle result
	// }

	Ok(())
}
