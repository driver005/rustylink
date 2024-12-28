use crate::{
	admin::constant::{COLUMNS_SQL, DEFAULT_SYSTEM_SCHEMAS, VIEW_SQL},
	utils::{filter_by_list, literal},
};
use common::{admin::view, DbPool};
use sea_orm::{FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct View {
	pub id: i32,
	pub schema: String,
	pub name: String,
	pub is_updatable: bool,
	pub comment: Option<String>,
	pub columns: Vec<view::ColumnResponce>,
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

impl View {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!(
					"{} WHERE views.id = {};",
					generate_enriched_views_sql(true),
					literal(&id.to_string())
				)
			}
			RetrieveParams::ByName {
				name,
				schema,
			} => {
				format!(
					"{} WHERE views.name = {} AND views.schema = {}",
					generate_enriched_views_sql(true),
					literal(&name),
					literal(&schema)
				)
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => Err(sea_orm::DbErr::Custom("Invalid parameters on view retrieve".to_string())),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: view::GetViewsRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = generate_enriched_views_sql(parms.include_columns.unwrap_or(false));

		let filter = filter_by_list(
			parms.included_schemas.clone(),
			parms.excluded_schemas.clone(),
			if !parms.include_system_schemas() {
				Some(DEFAULT_SYSTEM_SCHEMAS)
			} else {
				None
			},
		);

		if let Some(filter_clause) = filter {
			sql.push_str(&format!(" WHERE schema {}", filter_clause));
		}

		if let Some(limit_value) = parms.limit {
			sql.push_str(&format!(" limit {}", limit_value));
		}

		if let Some(offset_value) = parms.offset {
			sql.push_str(&format!(" offset {}", offset_value));
		}

		Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).all(conn).await
	}
}

fn generate_enriched_views_sql(include_columns: bool) -> String {
	let mut sql = format!("WITH views AS ({})", VIEW_SQL);

	if include_columns {
		sql.push_str(&format!(", columns AS ({})", COLUMNS_SQL));
	}

	sql.push_str("SELECT *");

	if include_columns {
		sql.push_str(", (SELECT coalesce(json_agg(columns.*), '[]'::json) FROM columns WHERE columns.table_id = views.id) AS columns");
	}

	sql.push_str(" FROM views");

	sql
}
