use std::error::Error;

use generator::prelude::{DateTimeCrate, WithSerde, WriterContext};

pub type DbPool = sea_orm::DatabaseConnection;
pub const DBTYPE: sea_orm::DatabaseBackend = sea_orm::DatabaseBackend::Postgres;

// pub mod admin;
pub mod app;
pub mod plugin;
pub mod types;

pub async fn generate(db: DbPool) -> Result<(), Box<dyn Error>> {
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
		Some("plugin".to_string()),
		generator::types::WebFrameworkEnum::Actix,
		false,
		false,
		true,
		Some(vec!["migrations".to_string()]),
		None,
	);
	generator::generate(db, "./link/common/src/plugin/", writer_context).await?;

	Ok(())
}
