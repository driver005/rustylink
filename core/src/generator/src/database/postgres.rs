use sea_orm::{sea_query::TableCreateStatement, sqlx::PgPool};
use sea_schema::postgres::discovery::SchemaDiscovery;

pub async fn generate(
	connection: PgPool,
	include_hidden_tables: bool,
	schema: &str,
) -> Result<Vec<TableCreateStatement>, Box<sea_orm::SqlxError>> {
	let schema_discovery = SchemaDiscovery::new(connection, schema);
	let schema = schema_discovery.discover().await.map_err(|_| sea_orm::SqlxError::RowNotFound)?;

	Ok(schema
		.tables
		.into_iter()
		// .filter(|schema| filter_tables(&schema.info.name))
		.filter(|schema| {
			crate::util::filter_hidden_tables(include_hidden_tables, &schema.info.name)
		})
		// .filter(|schema| filter_skip_tables(&schema.info.name))
		.map(|schema| schema.write())
		.collect())
}
