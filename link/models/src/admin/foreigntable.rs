use crate::{
	admin::constant::{COLUMNS_SQL, FOREIGN_TABLE_SQL},
	utils::{coalesce_rows_to_array, filter_by_list, literal},
};
use common::{admin::foreigntable, DbPool};
use sea_orm::{FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct ForeignTable {
	pub id: i32,
	pub schema: String,
	pub name: String,
	pub columns: Vec<foreigntable::ColumnResponce>,
}

#[derive(Debug, Clone)]
pub enum RetrieveParams {
	ById {
		id: i32,
	},
	ByName {
		name: String,
		schema: String,
	},
}

impl ForeignTable {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!(
					"{} WHERE foreign_tables.id = {};",
					generate_enriched_foreign_tables_sql(true),
					literal(&id.to_string())
				)
			}
			RetrieveParams::ByName {
				name,
				schema,
			} => {
				format!(
					"{} WHERE foreign_tables.name = {} AND foreign_tables.schema = {};",
					generate_enriched_foreign_tables_sql(true),
					literal(name),
					literal(schema)
				)
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(column) => return Ok(column),
			None => Err(sea_orm::DbErr::Custom(
				"Invalid parameters on foreign table retrieve".to_string(),
			)),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: foreigntable::GetForeignTablesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = generate_enriched_foreign_tables_sql(parms.include_columns.unwrap_or(false));

		if let Some(filter) =
			filter_by_list(parms.included_schemas.clone(), parms.excluded_schemas.clone(), None)
		{
			sql.push_str(&format!(" WHERE schema {}", filter));
		}

		if let Some(limit) = parms.limit {
			sql.push_str(&format!(" LIMIT {}", limit));
		}

		if let Some(offset) = parms.offset {
			sql.push_str(&format!(" OFFSET {}", offset));
		}

		Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).all(conn).await
	}
}

fn generate_enriched_foreign_tables_sql(include_columns: bool) -> String {
	let mut sql = format!("WITH foreign_tables AS ({})", FOREIGN_TABLE_SQL);

	if include_columns {
		sql.push_str(&format!(", columns AS ({})", COLUMNS_SQL));
	}

	sql.push_str("SELECT *");

	if include_columns {
		sql.push_str(&format!(
			", {}",
			coalesce_rows_to_array("columns", "columns.table_id = foreign_tables.id")
		));
	}

	sql.push_str(" FROM foreign_tables");

	sql
}
