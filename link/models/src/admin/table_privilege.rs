use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, TABLE_PRIVILEGE_SQL},
	utils::{filter_by_list, literal},
};
use common::{admin::table, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct TablePrivilege {
	pub relation_id: i32,
	pub schema: String,
	pub name: String,
	pub kind: table::RelationKind,
	pub privileges: Vec<table::Privilege>,
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

impl TablePrivilege {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!(
					"WITH table_privileges AS ({}) SELECT * FROM table_privileges WHERE relation_id = {};",
					TABLE_PRIVILEGE_SQL,
					literal(&id.to_string())
				)
			}
			RetrieveParams::ByName {
				name,
				schema,
			} => {
				format!(
					"WITH table_privileges AS ({}) SELECT * FROM table_privileges WHERE name = {} AND schema = {}",
					TABLE_PRIVILEGE_SQL,
					literal(&name),
					literal(&schema)
				)
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => Err(sea_orm::DbErr::Custom(
				"Invalid parameters on table privilege retrieve".to_string(),
			)),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: table::GetTablePrivilegesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = format!(
			"WITH table_privileges AS ({}) SELECT * FROM table_privileges",
			TABLE_PRIVILEGE_SQL
		);

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

	pub async fn grant(
		conn: &DbPool,
		parms: &Vec<table::GrantTablePrivilegesRequest>,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let sql = format!(
			"DO $$ BEGIN {} END $$;",
			parms
				.iter()
				.map(|grant| {
					format!(
						"EXECUTE format('GRANT {} ON TABLE %s TO {} {}', {}::regclass);",
						grant.privilege_type,
						if grant.grantee.to_lowercase() == "public" {
							"public".to_string()
						} else {
							format!("\"{}\"", grant.grantee.replace("\"", "\"\""))
						},
						if grant.is_grantable.unwrap_or(false) {
							"WITH GRANT OPTION"
						} else {
							""
						},
						grant.relation_id
					)
				})
				.collect::<Vec<_>>()
				.join("\n")
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Return the updated table privileges for modified relations
		let relation_ids: Vec<i32> = parms.iter().map(|grant| grant.relation_id).collect();
		let updated_sql = format!(
			"WITH table_privileges AS ({}) SELECT * FROM table_privileges WHERE relation_id in {}",
			TABLE_PRIVILEGE_SQL,
			relation_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","),
		);

		Self::find_by_statement(Statement::from_string(common::DBTYPE, updated_sql)).all(conn).await
	}

	pub async fn revoke(
		conn: &DbPool,
		parms: &Vec<table::RevokeTablePrivilegesRequest>,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let sql = format!(
			"DO $$ BEGIN {} END $$;",
			parms
				.iter()
				.map(|revoke| {
					format!(
						"EXECUTE format('REVOKE {} ON TABLE %s FROM {}', {}::regclass);",
						revoke.privilege_type, revoke.grantee, revoke.relation_id
					)
				})
				.collect::<Vec<_>>()
				.join("\n")
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Return the updated table privileges for modified relations
		let relation_ids: Vec<i32> = parms.iter().map(|grant| grant.relation_id).collect();
		let updated_sql = format!(
			"WITH table_privileges AS ({}) SELECT * FROM table_privileges WHERE relation_id in {}",
			TABLE_PRIVILEGE_SQL,
			relation_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","),
		);

		Self::find_by_statement(Statement::from_string(common::DBTYPE, updated_sql)).all(conn).await
	}
}
