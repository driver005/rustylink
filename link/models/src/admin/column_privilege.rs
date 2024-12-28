use crate::{
	admin::constant::{COLUMN_PRIVILEGE_SQL, DEFAULT_SYSTEM_SCHEMAS},
	utils::filter_by_list,
};
use common::{
	admin::{column, core},
	DbPool,
};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct ColumnPrivilege {
	pub column_id: String,
	pub relation_schema: String,
	pub relation_name: String,
	pub column_name: String,
	pub privileges: Vec<core::Privilege>,
}

impl ColumnPrivilege {
	pub async fn list(
		conn: &DbPool,
		parms: column::GetColumnPrivilegesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = format!(
			"with column_privileges as ({})
            select *
            from column_privileges",
			COLUMN_PRIVILEGE_SQL
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
			sql.push_str(&format!(" where relation_schema {}", filter_clause));
		}

		if let Some(limit_value) = parms.limit {
			sql.push_str(&format!(" limit {}", limit_value));
		}

		if let Some(offset_value) = parms.offset {
			sql.push_str(&format!(" offset {}", offset_value));
		}

		ColumnPrivilege::find_by_statement(Statement::from_string(common::DBTYPE, sql))
			.all(conn)
			.await
	}

	pub async fn grant(
		conn: &DbPool,
		parms: &Vec<column::GrantColumnPrivilegeRequest>,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = String::from(
			"
            DO $$
            DECLARE
                col record;
            BEGIN
            ",
		);

		for grant in parms {
			let (relation_id, column_number) = grant.column_id.split_once('.').unwrap();
			sql.push_str(&format!(
				"
                SELECT *
                FROM pg_attribute a
                WHERE a.attrelid = {}
                    AND a.attnum = {}
                INTO col;
                EXECUTE format(
                    'GRANT {} (%I) ON %s TO {} {}',
                    col.attname,
                    col.attrelid::regclass,
                    '{}'::text,
                    '{}'::text,
                    '{}'::text
                );
                ",
				relation_id,
				column_number,
				grant.privilege_type,
				if grant.grantee.to_lowercase() == "public" {
					"PUBLIC"
				} else {
					&grant.grantee
				},
				if grant.is_grantable {
					"WITH GRANT OPTION"
				} else {
					""
				},
				grant.privilege_type,
				grant.grantee,
				if grant.is_grantable {
					"WITH GRANT OPTION"
				} else {
					""
				}
			));
		}

		sql.push_str("END $$;");

		// Execute the DO block
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Now, let's query for the updated column privileges
		let column_ids: Vec<String> = parms.iter().map(|g| g.column_id.clone()).collect();

		let query = format!(
			"
            WITH column_privileges AS ({})
            SELECT *
            FROM column_privileges
            WHERE column_id IN ({})",
			COLUMN_PRIVILEGE_SQL,
			column_ids.iter().map(|id| format!("'{}'", id)).collect::<Vec<String>>().join(",")
		);

		ColumnPrivilege::find_by_statement(Statement::from_string(common::DBTYPE, query))
			.all(conn)
			.await
	}

	pub async fn revoke(
		conn: &DbPool,
		parms: &Vec<column::RevokeColumnPrivilegeRequest>,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = String::from(
			"
            DO $$
            DECLARE
                col record;
            BEGIN
            ",
		);

		for revoke in parms {
			let (relation_id, column_number) = revoke.column_id.split_once('.').unwrap();
			sql.push_str(&format!(
				"
                SELECT *
                FROM pg_attribute a
                WHERE a.attrelid = {}
                    AND a.attnum = {}
                INTO col;
                EXECUTE format(
                    'REVOKE {} (%I) ON %s FROM {}',
                    col.attname,
                    col.attrelid::regclass,
                    '{}'::text,
                    '{}'::text
                );
                ",
				relation_id,
				column_number,
				revoke.privilege_type,
				if revoke.grantee.to_lowercase() == "public" {
					"PUBLIC"
				} else {
					&revoke.grantee
				},
				revoke.privilege_type,
				revoke.grantee
			));
		}

		sql.push_str("END $$;");

		// Execute the DO block
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Now, let's query for the updated column privileges
		let column_ids: Vec<String> = parms.iter().map(|g| g.column_id.clone()).collect();

		let query = format!(
			"
            WITH column_privileges AS ({})
            SELECT *
            FROM column_privileges
            WHERE column_id IN ({})",
			COLUMN_PRIVILEGE_SQL,
			column_ids.iter().map(|id| format!("'{}'", id)).collect::<Vec<String>>().join(",")
		);

		ColumnPrivilege::find_by_statement(Statement::from_string(common::DBTYPE, query))
			.all(conn)
			.await
	}
}
