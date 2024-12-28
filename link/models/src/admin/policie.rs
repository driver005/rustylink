use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, POLICIE_SQL},
	utils::{filter_by_list, ident, literal},
};
use common::{admin::polices, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Policie {
	pub id: i32,
	pub schema: String,
	pub table: String,
	pub table_id: i32,
	pub name: String,
	pub action: polices::Action,
	pub roles: Vec<String>,
	pub command: polices::Command,
	pub definition: Option<String>,
	pub check: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RetrieveParams {
	ById {
		id: String,
	},
	ByName {
		name: String,
		table: String,
		schema: String,
	},
}

impl Policie {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} WHERE pol.oid = {};", POLICIE_SQL, literal(&id.to_string()))
			}
			RetrieveParams::ByName {
				name,
				table,
				schema,
			} => {
				format!(
					"{} WHERE pol.polname = {} AND n.nspname = {} AND c.relname = {};",
					POLICIE_SQL,
					literal(&name),
					literal(&schema),
					literal(&table)
				)
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => {
				Err(sea_orm::DbErr::Custom("Invalid parameters on policy retrieve".to_string()))
			}
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: polices::GetPoliciesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = POLICIE_SQL.to_string();

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
			sql.push_str(&format!(" WHERE n.nspname {}", filter_clause));
		}

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
		parms: polices::CreatePolicieRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let definition_clause = if let Some(definition) = &parms.definition {
			format!("USING ({})", definition)
		} else {
			String::new()
		};

		let check_clause = if let Some(check) = &parms.check {
			format!("WITH CHECK ({})", check)
		} else {
			String::new()
		};

		let sql = format!(
			"CREATE POLICY {} ON {}.{} AS {} FOR {} TO {} {} {};",
			ident(&parms.name),
			ident(&parms.schema.clone().unwrap_or("public".to_string())),
			ident(&parms.table),
			parms.action(),
			parms.command(),
			parms.roles.iter().map(|r| ident(r)).collect::<Vec<_>>().join(","),
			definition_clause,
			check_clause
		);

		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Self::retrieve(
			conn,
			&RetrieveParams::ByName {
				name: parms.name,
				table: parms.table,
				schema: parms.schema.unwrap_or("public".to_string()),
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: polices::UpdatePolicieRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let old = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		let alter = format!(
			"ALTER POLICY {} ON {}.{}",
			ident(&old.name),
			ident(&old.schema),
			ident(&old.table)
		);

		let name_sql = if let Some(new_name) = parms.name {
			format!("{} RENAME TO {};", alter, ident(&new_name))
		} else {
			String::new()
		};

		let definition_sql = if let Some(def) = parms.definition {
			format!("{} USING ({});", alter, def)
		} else {
			String::new()
		};

		let check_sql = if let Some(chk) = parms.check {
			format!("{} WITH CHECK ({});", alter, chk)
		} else {
			String::new()
		};

		let roles_sql = if !parms.roles.is_empty() {
			format!(
				"{} TO {};",
				alter,
				parms.roles.iter().map(|r| ident(r)).collect::<Vec<_>>().join(",")
			)
		} else {
			String::new()
		};

		let sql =
			format!("BEGIN; {} {} {} {} COMMIT;", definition_sql, check_sql, roles_sql, name_sql);

		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

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
		parms: polices::DeletePolicieRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let policy = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		let sql = format!(
			"DROP POLICY {} ON {}.{};",
			ident(&policy.name),
			ident(&policy.schema),
			ident(&policy.table)
		);

		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(policy)
	}
}
