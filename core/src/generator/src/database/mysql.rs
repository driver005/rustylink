use sea_orm::{sea_query::TableCreateStatement, sqlx::MySqlPool};
use sea_schema::mysql::discovery::SchemaDiscovery;

pub async fn generate(
	connection: MySqlPool,
	include_hidden_tables: bool,
	schema: &str,
) -> Result<Vec<TableCreateStatement>, sea_orm::SqlxError> {
	let schema_discovery = SchemaDiscovery::new(connection, schema);
	let schema = schema_discovery.discover().await.map_err(|_| sea_orm::SqlxError::RowNotFound)?;

	Ok(schema
		.tables
		.into_iter()
		.filter(|schema| {
			crate::util::filter_hidden_tables(include_hidden_tables, &schema.info.name)
		})
		.map(|schema| schema.write())
		.collect())
}
