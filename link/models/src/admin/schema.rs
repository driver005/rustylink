use common::{admin::schema, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, SCHEMA_SQL},
	utils::{ident, literal},
};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Schema {
	pub id: i32,
	pub name: String,
	pub owner: String,
}

#[derive(Debug, Clone)]
pub enum RetrieveParams {
	ById {
		id: String,
	},
	ByName {
		name: String,
	},
}

impl Schema {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} WHERE n.oid = {};", SCHEMA_SQL, literal(id))
			}
			RetrieveParams::ByName {
				name,
			} => {
				format!("{} WHERE n.pubname = {};", SCHEMA_SQL, literal(name))
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => {
				Err(sea_orm::DbErr::Custom("Invalid parameters on schema retrieve".to_string()))
			}
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: schema::GetSchemasRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = SCHEMA_SQL.to_string();

		if !parms.include_system_schemas.unwrap_or(false) {
			sql.push_str(&format!(
				" AND NOT (n.nspname = ANY(ARRAY[{}]))",
				DEFAULT_SYSTEM_SCHEMAS
					.iter()
					.map(|s| format!("'{}'", s))
					.collect::<Vec<_>>()
					.join(",")
			));
		}

		if let Some(limit_value) = &parms.limit {
			sql.push_str(&format!(" limit {}", limit_value));
		}

		if let Some(offset_value) = &parms.offset {
			sql.push_str(&format!(" offset {}", offset_value));
		}

		Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).all(conn).await
	}

	pub async fn create(
		conn: &DbPool,
		parms: schema::CreateSchemasRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let owner = parms.owner.unwrap_or("postgres".to_string());
		let sql = format!("CREATE SCHEMA {} AUTHORIZATION {};", ident(&parms.name), ident(&owner));

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			&RetrieveParams::ByName {
				name: parms.name,
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: schema::UpdateSchemaRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let old_schema = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		let name_sql = parms.name.as_ref().map_or(String::new(), |name| {
			format!("ALTER SCHEMA {} RENAME TO {};", ident(&old_schema.name), ident(name))
		});

		let owner_sql = parms.owner.as_ref().map_or(String::new(), |owner| {
			format!("ALTER SCHEMA {} OWNER TO {};", ident(&old_schema.name), ident(owner))
		});

		let sql = format!("BEGIN; {} {} COMMIT;", owner_sql, name_sql);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await
	}

	pub async fn delete(
		conn: &DbPool,
		parms: schema::DeleteSchemaRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let schema = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		let sql = format!(
			"DROP SCHEMA {} {};",
			ident(&schema.name),
			if parms.cascade.unwrap_or(false) {
				"CASCADE"
			} else {
				"RESTRICT"
			}
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(schema)
	}
}
