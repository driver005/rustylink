use common::{admin::table, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

use crate::{
	admin::constant::{COLUMNS_SQL, DEFAULT_SYSTEM_SCHEMAS, TABLE_SQL},
	utils::{coalesce_rows_to_array, filter_by_list, ident, literal},
};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Table {
	pub id: i32,
	pub schema: String,
	pub name: String,
	pub rls_enabled: bool,
	pub rls_forced: bool,
	pub replica_identity: table::ReplicaIdentity,
	pub bytes: i64,
	pub size: String,
	pub live_rows_estimate: i64,
	pub dead_rows_estimate: i64,
	pub comment: Option<String>,
	pub columns: Vec<crate::admin::column::Column>,
	pub primary_keys: Vec<common::admin::table::PrimaryKeySchema>,
	pub relationships: Vec<common::admin::table::RelationshipOldSchema>,
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

impl Table {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} where tables.id = {};", generate_enriched_tables_sql(true), id)
			}
			RetrieveParams::ByName {
				name,
				schema,
			} => {
				format!(
					"{} where tables.name = {} and tables.schema = {};",
					generate_enriched_tables_sql(true),
					ident(name),
					ident(schema),
				)
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(column) => return Ok(column),
			None => Err(sea_orm::DbErr::Custom("Invalid parameters on table retrieve".to_string())),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: table::GetTablesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = generate_enriched_tables_sql(parms.include_columns());

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
			sql.push_str(&format!(" where schema {}", filter_clause));
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
		parms: table::CreateTablesRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let schema = parms.schema.unwrap_or("public".to_string());

		let table_sql =
			format!("CREATE TABLE {}.{} ();", ident(schema.as_str()), ident(&parms.name));

		let comment_sql = if let Some(comment) = parms.comment {
			Some(format!(
				"COMMENT ON TABLE {}.{} IS {};",
				ident(schema.as_str()),
				ident(&parms.name),
				literal(comment.as_str())
			))
		} else {
			None
		};

		let sql = format!("BEGIN; {} {} COMMIT;", table_sql, comment_sql.unwrap_or("".to_string()));

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			&RetrieveParams::ByName {
				name: parms.name,
				schema,
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: table::UpdateTableRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let old = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await?;

		let alter = format!("ALTER TABLE {}.{}", ident(&old.schema), ident(&old.name));

		let schema_sql = parms
			.schema
			.as_ref()
			.map(|schema| format!("{} SET SCHEMA {};", alter, ident(schema)))
			.unwrap_or_default();

		let name_sql = generate_name_sql(&parms, &old);
		let enable_rls = generate_rls_sql(&parms, &alter);
		let force_rls = generate_force_rls_sql(&parms, &alter);
		let replica_sql = generate_replica_sql(&parms, &alter);
		let primary_keys_sql = generate_primary_keys_sql(&parms, &old, &alter, parms.id.clone());

		let comment_sql = parms
			.comment
			.as_ref()
			.map(|comment| {
				format!(
					"COMMENT ON TABLE {}.{} IS {};",
					ident(&old.schema),
					ident(&old.name),
					literal(comment)
				)
			})
			.unwrap_or_default();

		let sql = format!(
			"BEGIN;
                {}
                {}
                {}
                {}
                {}
                {}
                {}
            COMMIT;",
			enable_rls, force_rls, replica_sql, primary_keys_sql, comment_sql, schema_sql, name_sql
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			&&RetrieveParams::ById {
				id: parms.id,
			},
		)
		.await
	}

	pub async fn delete(
		conn: &DbPool,
		parms: table::DeleteTableRequest,
	) -> Result<Self, sea_orm::DbErr> {
		// First, retrieve the table information
		let table = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id,
			},
		)
		.await?;

		// Construct the SQL for dropping the table
		let sql = format!(
			"DROP TABLE {}.{} {};",
			ident(&table.schema),
			ident(&table.name),
			if parms.cascade.unwrap_or(false) {
				"CASCADE"
			} else {
				"RESTRICT"
			}
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(table)
	}
}

fn generate_name_sql(update: &table::UpdateTableRequest, old: &Table) -> String {
	match &update.name {
		Some(name) if name != &old.name => {
			let current_schema = update.schema.as_ref().unwrap_or(&old.schema);
			format!(
				"ALTER TABLE {}.{} RENAME TO {};",
				ident(current_schema),
				ident(&old.name),
				ident(name)
			)
		}
		_ => String::new(),
	}
}

fn generate_rls_sql(update: &table::UpdateTableRequest, alter: &str) -> String {
	match update.rls_enabled {
		Some(true) => format!("{} ENABLE ROW LEVEL SECURITY;", alter),
		Some(false) => format!("{} DISABLE ROW LEVEL SECURITY;", alter),
		None => String::new(),
	}
}

fn generate_force_rls_sql(update: &table::UpdateTableRequest, alter: &str) -> String {
	match update.rls_forced {
		Some(true) => format!("{} FORCE ROW LEVEL SECURITY;", alter),
		Some(false) => format!("{} NO FORCE ROW LEVEL SECURITY;", alter),
		None => String::new(),
	}
}

fn generate_replica_sql(update: &table::UpdateTableRequest, alter: &str) -> String {
	if update.replica_identity() == table::ReplicaIdentity::Index {
		format!(
			"{} REPLICA IDENTITY USING INDEX {};",
			alter,
			update.replica_identity_index.as_ref().unwrap()
		)
	} else {
		format!("{} REPLICA IDENTITY {};", alter, update.replica_identity().as_str_name())
	}
}

fn generate_primary_keys_sql(
	update: &table::UpdateTableRequest,
	old: &Table,
	alter: &str,
	id: i32,
) -> String {
	let mut sql = String::new();
	if !old.primary_keys.is_empty() {
		sql.push_str(&format!(
			"DO $$
            DECLARE
                r record;
            BEGIN
                SELECT conname
                    INTO r
                    FROM pg_constraint
                    WHERE contype = 'p' AND conrelid = {};
                EXECUTE {} || ident(r.conname);
            END
            $$;
            ",
			literal(&id.to_string()),
			literal(&format!("{} DROP CONSTRAINT ", alter))
		));
	}
	if !update.primary_keys.is_empty() {
		sql.push_str(&format!(
			"{} ADD PRIMARY KEY ({});",
			alter,
			update.primary_keys.iter().map(|pk| ident(&pk.name)).collect::<Vec<String>>().join(",")
		));
	}
	sql
}

fn generate_enriched_tables_sql(include_columns: bool) -> String {
	let mut sql = format!("WITH tables AS ({})", TABLE_SQL);

	if include_columns {
		sql.push_str(&format!(", columns AS ({})", COLUMNS_SQL));
	}

	sql.push_str("\nSELECT *");

	if include_columns {
		sql.push_str(&format!(
			", {}",
			coalesce_rows_to_array("columns", "columns.table_id = tables.id")
		));
	}

	sql.push_str("\nFROM tables");

	sql
}
