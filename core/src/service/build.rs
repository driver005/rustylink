use dotenv::dotenv;
use generator::prelude::{DateTimeCrate, WithSerde, WriterContext};
use lazy_static::lazy_static;
use sea_orm::Database;
use std::{env, error::Error};

lazy_static! {
	static ref DATABASE_URL: String =
		env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	tonic_build::configure()
		.compile_well_known_types(true)
		.build_client(false)
		.build_server(true)
		.build_transport(false)
		.compile_protos(&["./src/voting.proto"], &["proto"])?;

	dotenv().ok();

	let db =
		Database::connect(&*DATABASE_URL).await.expect("Fail to initialize database connection");

	let writer_context = &WriterContext::new(
		false,
		WithSerde::Both,
		true,
		DateTimeCrate::Chrono,
		Some("public".to_string()),
		false,
		true,
		Vec::new(),
		Vec::new(),
		Vec::new(),
		Vec::new(),
		true,
		None,
		Some("handles".to_string()),
		generator::types::WebFrameworkEnum::Actix,
		true,
		true,
		true,
		Some(vec!["migrations".to_string()]),
		None,
	);
	generator::generate(db, "./src/handles", writer_context).await?;

	Ok(())
}
