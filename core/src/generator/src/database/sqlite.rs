use std::error::Error;

use sea_orm::{sea_query::TableCreateStatement, sqlx::SqlitePool};
use sea_schema::sqlite::discovery::SchemaDiscovery;

pub async fn generate(
	connection: SqlitePool,
	include_hidden_tables: bool,
) -> Result<Vec<TableCreateStatement>, Box<dyn Error>> {
	let schema_discovery = SchemaDiscovery::new(connection);
	let schema = schema_discovery.discover().await?.merge_indexes_into_table();

	Ok(schema
		.tables
		.into_iter()
		.filter(|schema| crate::util::filter_hidden_tables(include_hidden_tables, &schema.name))
		.map(|schema| schema.write())
		.collect())
}
