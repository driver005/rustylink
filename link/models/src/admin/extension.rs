use common::{admin::extension, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

use crate::{
	admin::constant::{COLUMN_PRIVILEGE_SQL, EXTENSION_SQL},
	utils::{ident, literal},
};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Extension {
	pub name: String,
	pub schema: Option<String>,
	pub default_version: String,
	pub installed_version: Option<String>,
	pub comment: Option<String>,
}

impl Extension {
	pub async fn retrieve(
		conn: &DbPool,
		parms: extension::GetExtensionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let sql = format!("{} WHERE name = {}", EXTENSION_SQL, parms.name);

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(column) => Ok(column),
			None => Err(sea_orm::DbErr::Custom(format!(
				"Cannot find an extension named {}",
				parms.name
			))),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: extension::GetExtensionsRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = COLUMN_PRIVILEGE_SQL.to_string();

		if let Some(limit_value) = parms.limit {
			sql.push_str(&format!(" limit {}", limit_value));
		}

		if let Some(offset_value) = parms.offset {
			sql.push_str(&format!(" offset {}", offset_value));
		}

		Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).all(conn).await
	}

	pub async fn create(
		conn: &DbPool,
		parms: extension::CreateExtensionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let mut sql = format!("CREATE EXTENSION {}", ident(&parms.name));

		if let Some(schema_name) = parms.schema {
			sql.push_str(&format!(" SCHEMA {}", ident(&schema_name)));
		}

		if let Some(version_str) = parms.version {
			sql.push_str(&format!(" VERSION {}", literal(&version_str)));
		}

		if parms.cascade.unwrap_or(false) {
			sql.push_str(" CASCADE");
		}

		sql.push(';');

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			extension::GetExtensionRequest {
				name: parms.name,
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: extension::UpdateExtensionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let mut sql = String::from("BEGIN;");

		if parms.update {
			sql.push_str(&format!(" ALTER EXTENSION {}", ident(&parms.name)));
			if let Some(v) = parms.version {
				sql.push_str(&format!(" UPDATE TO {}", literal(&v)));
			} else {
				sql.push_str(" UPDATE");
			}
			sql.push(';');
		}

		if let Some(s) = parms.schema {
			sql.push_str(&format!(
				" ALTER EXTENSION {} SET SCHEMA {};",
				ident(&parms.name),
				ident(&s)
			));
		}

		sql.push_str(" COMMIT;");

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			extension::GetExtensionRequest {
				name: parms.name,
			},
		)
		.await
	}

	pub async fn delete(
		conn: &DbPool,
		parms: extension::DeleteExtensionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let extension = Self::retrieve(
			conn,
			extension::GetExtensionRequest {
				name: parms.name.clone(),
			},
		)
		.await?;

		let sql = format!(
			"DROP EXTENSION {} {};",
			ident(&parms.name),
			if parms.cascade.unwrap_or(false) {
				"CASCADE"
			} else {
				"RESTRICT"
			}
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(extension)
	}
}
