use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, TYPE_SQL},
	utils::filter_by_list,
};
use common::{admin::types, DbPool};
use sea_orm::{FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Types {
	pub id: i32,
	pub name: String,
	pub schema: String,
	pub format: String,
	pub enums: Vec<String>,
	pub attributes: Vec<types::Attribute>,
	pub comment: Option<String>,
}

impl Types {
	pub async fn list(
		conn: &DbPool,
		parms: types::GetTypesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = TYPE_SQL.to_string();

		if !parms.include_array_types.unwrap_or(false) {
			sql.push_str(
				" and not exists (
                    select
                    from
                        pg_type el
                    where
                        el.oid = t.typelem
                        and el.typarray = t.oid
                )",
			);
		}

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
			sql.push_str(&format!(" and n.nspname {}", filter_clause));
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
