use super::types::Pool;

struct Table {
	pool: Pool,
}

impl Table {
	pub async fn list(
		&self,
		include_system_schemas: bool,
		included_schemas: Option<Vec<String>>,
		excluded_schemas: Option<Vec<String>>,
		limit: Option<i64>,
		offset: Option<i64>,
		include_columns: bool,
	) -> Result<Vec<PostgresTable>> {
		let mut sql = self.generate_enriched_tables_sql(options.include_columns);

		// Apply filters, limit, offset here

		let tables = sqlx::query_as::<_, PostgresTable>(&sql).fetch_all(&self.pool).await?;

		Ok(tables)
	}

	pub async fn retrieve(
		&self,
		id: Option<i64>,
		name: Option<String>,
		schema: Option<String>,
	) -> Result<Option<PostgresTable>> {
		let sql = self.generate_enriched_tables_sql(true);
		let sql = if let Some(id) = id {
			format!("{} WHERE tables.id = $1", sql)
		} else if let Some(name) = name {
			format!("{} WHERE tables.name = $1 AND tables.schema = $2", sql)
		} else {
			return Err(sqlx::Error::Configuration("Invalid parameters for table retrieve".into()));
		};

		let table = if let Some(id) = id {
			sqlx::query_as::<_, PostgresTable>(&sql).bind(id).fetch_optional(&self.pool).await?
		} else {
			sqlx::query_as::<_, PostgresTable>(&sql)
				.bind(name.unwrap())
				.bind(schema.unwrap_or_else(|| "public".to_string()))
				.fetch_optional(&self.pool)
				.await?
		};

		Ok(table)
	}

	pub async fn create(&self, table: PostgresTableCreate) -> Result<PostgresTable> {
		let schema = table.schema.unwrap_or_else(|| "public".to_string());
		let sql = format!("CREATE TABLE {}.{} ();", self.ident(&schema), self.ident(&table.name));

		let comment_sql = if let Some(comment) = table.comment {
			format!(
				"COMMENT ON TABLE {}.{} IS {};",
				self.ident(&schema),
				self.ident(&table.name),
				self.literal(&comment)
			)
		} else {
			String::new()
		};

		let sql = format!("BEGIN; {} {} COMMIT;", sql, comment_sql);

		sqlx::query(&sql).execute(&self.pool).await?;

		self.retrieve(None, Some(table.name), Some(schema))
			.await?
			.ok_or_else(|| sqlx::Error::RowNotFound)
	}

	// Implement update, remove, and other methods...

	fn generate_enriched_tables_sql(&self, include_columns: bool) -> String {
		// Implement this method to generate the SQL for listing tables
		// You'll need to port the SQL queries from the TypeScript version
		todo!()
	}

	fn ident(&self, name: &str) -> String {
		format!("\"{}\"", name.replace("\"", "\"\""))
	}

	fn literal(&self, value: &str) -> String {
		format!("'{}'", value.replace("'", "''"))
	}
}
