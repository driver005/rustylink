use crate::prelude::{DateTimeCrate, EntityTransformer, WriterContext};
use sea_orm::DatabaseConnection;
use std::error::Error;

pub mod api;
pub mod database;
pub mod entity;
pub mod error;
pub mod prelude;
pub mod templates;
pub mod types;
pub mod util;
pub mod writer;

#[cfg(test)]
mod tests_cfg;

pub async fn generate(
	conn: DatabaseConnection,
	output_dir: &str,
	writer_context: &WriterContext,
) -> Result<(), Box<dyn Error>> {
	let schema = writer_context.schema_name.clone().unwrap_or("public".to_string());
	let table_stmts = match conn {
		DatabaseConnection::SqlxPostgresPoolConnection(_) => Some(
			database::postgres::generate(
				conn.get_postgres_connection_pool().clone(),
				true,
				&schema,
			)
			.await?,
		),
		DatabaseConnection::SqlxSqlitePoolConnection(_) => {
			Some(database::sqlite::generate(conn.get_sqlite_connection_pool().clone(), true).await?)
		}
		DatabaseConnection::SqlxMySqlPoolConnection(_) => Some(
			database::mysql::generate(conn.get_mysql_connection_pool().clone(), true, &schema)
				.await?,
		),
		DatabaseConnection::Disconnected => None,
	};

	if let Some(table_stmts) = table_stmts {
		let output = EntityTransformer::transform(table_stmts)?.generate(writer_context);

		output.create(output_dir)?;
	}
	Ok(())
}
