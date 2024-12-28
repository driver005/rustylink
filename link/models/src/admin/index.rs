use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, INDEX_SQL},
	utils::filter_by_list,
};
use common::{admin::index, DbPool};
use sea_orm::{FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Index {
	pub id: i32,
	pub table_id: i32,
	pub schema: String,
	pub number_of_attributes: i32,
	pub number_of_key_attributes: i32,
	pub is_unique: bool,
	pub is_primary: bool,
	pub is_exclusion: bool,
	pub is_immediate: bool,
	pub is_clustered: bool,
	pub is_valid: bool,
	pub check_xmin: bool,
	pub is_ready: bool,
	pub is_live: bool,
	pub is_replica_identity: bool,
	pub key_attributes: Vec<i32>,
	pub collation: Vec<i32>,
	pub class: Vec<i32>,
	pub options: Vec<i32>,
	pub index_predicate: Option<String>,
	pub comment: Option<String>,
	pub index_definition: String,
	pub access_method: String,
	pub index_attributes: Vec<index::IndexAttribute>,
}

impl Index {
	pub async fn retrieve(
		conn: &DbPool,
		parms: index::GetIndexRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let sql = format!("{} WHERE id = {}", INDEX_SQL, parms.id);

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => Err(sea_orm::DbErr::Custom("Invalid parameters on index retrieve".to_string())),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: index::GetIndexesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = INDEX_SQL.to_string();

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
			sql.push_str(&format!(" AND schema {}", filter_clause));
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
