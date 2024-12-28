use crate::{
	admin::constant::ROLE_SQL,
	utils::{ident, literal},
};
use common::{admin::role, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Role {
	pub id: i32,
	pub name: String,
	pub is_superuser: bool,
	pub can_create_db: bool,
	pub can_create_role: bool,
	pub inherit_role: bool,
	pub can_login: bool,
	pub is_replication_role: bool,
	pub can_bypass_rls: bool,
	pub active_connections: i32,
	pub connection_limit: i32,
	pub password: String,
	pub valid_until: Option<String>,
	pub config: Option<String>,
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

impl Role {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} WHERE oid = {};", ROLE_SQL, literal(id))
			}
			RetrieveParams::ByName {
				name,
			} => {
				format!("{} WHERE rolname = {};", ROLE_SQL, literal(name))
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => Err(sea_orm::DbErr::Custom("Invalid parameters on role retrieve".to_string())),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: role::GetRolesRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = format!(
			"WITH roles AS ({})
            SELECT *
            FROM roles
            WHERE true",
			ROLE_SQL
		);

		if !parms.include_default_roles.unwrap_or(false) {
			sql.push_str(" AND NOT pg_catalog.starts_with(name, 'pg_')");
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
		parms: role::CreateRolesRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let is_superuser_clause = if parms.is_superuser {
			"SUPERUSER"
		} else {
			"NOSUPERUSER"
		};

		let can_create_db_clause = if parms.can_create_db {
			"CREATEDB"
		} else {
			"NOCREATEDB"
		};

		let can_create_role_clause = if parms.can_create_role {
			"CREATEROLE"
		} else {
			"NOCREATEROLE"
		};

		let inherit_role_clause = if parms.inherit_role {
			"INHERIT"
		} else {
			"NOINHERIT"
		};

		let can_login_clause = if parms.can_login {
			"LOGIN"
		} else {
			"NOLOGIN"
		};

		let is_replication_role_clause = if parms.is_replication_role {
			"REPLICATION"
		} else {
			"NOREPLICATION"
		};

		let can_bypass_rls_clause = if parms.can_bypass_rls {
			"BYPASSRLS"
		} else {
			"NOBYPASSRLS"
		};

		let connection_limit_clause = format!("CONNECTION LIMIT {}", parms.connection_limit);

		let password_clause =
			parms.password.as_ref().map_or(String::new(), |p| format!("PASSWORD {}", literal(p)));

		let valid_until_clause = parms
			.valid_until
			.as_ref()
			.map_or(String::new(), |v| format!("VALID UNTIL {}", literal(v)));

		let member_of_clause = if !parms.member_of.is_empty() {
			format!("IN ROLE {}", parms.member_of.join(","))
		} else {
			String::new()
		};

		let members_clause = if !parms.members.is_empty() {
			format!("ROLE {}", parms.member_of.join(","))
		} else {
			String::new()
		};

		let admins_clause = if !parms.members.is_empty() {
			format!("ADMIN {}", parms.member_of.join(","))
		} else {
			String::new()
		};

		let config_clause = if !parms.config.is_empty() {
			parms
				.config
				.iter()
				.filter(|(k, v)| !k.is_empty() && !v.is_empty())
				.map(|(k, v)| format!("ALTER ROLE {} SET {} = {};", ident(&parms.name), k, v))
				.collect::<Vec<String>>()
				.join("\n")
		} else {
			String::new()
		};

		let sql = format!(
			r#"
            BEGIN;
            CREATE ROLE {}
            WITH
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {};
            {}
            COMMIT;"#,
			ident(&parms.name),
			is_superuser_clause,
			can_create_db_clause,
			can_create_role_clause,
			inherit_role_clause,
			can_login_clause,
			is_replication_role_clause,
			can_bypass_rls_clause,
			connection_limit_clause,
			password_clause,
			valid_until_clause,
			member_of_clause,
			members_clause,
			admins_clause,
			config_clause
		);

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
		parms: role::UpdateRoleRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let old = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		let is_superuser_clause = if parms.is_superuser.unwrap_or(false) {
			"SUPERUSER"
		} else {
			"NOSUPERUSER"
		};

		let can_create_db_clause = if parms.can_create_db.unwrap_or(false) {
			"CREATEDB"
		} else {
			"NOCREATEDB"
		};

		let can_create_role_clause = if parms.can_create_role.unwrap_or(false) {
			"CREATEROLE"
		} else {
			"NOCREATEROLE"
		};

		let inherit_role_clause = if parms.inherit_role.unwrap_or(false) {
			"INHERIT"
		} else {
			"NOINHERIT"
		};

		let can_login_clause = if parms.can_login.unwrap_or(false) {
			"LOGIN"
		} else {
			"NOLOGIN"
		};

		let is_replication_role_clause = if parms.is_replication_role.unwrap_or(false) {
			"REPLICATION"
		} else {
			"NOREPLICATION"
		};

		let can_bypass_rls_clause = if parms.can_bypass_rls.unwrap_or(false) {
			"BYPASSRLS"
		} else {
			"NOBYPASSRLS"
		};

		let name_sql = parms.name.as_ref().map_or(String::new(), |name| {
			if !name.contains(&old.name) {
				format!("ALTER ROLE {} RENAME TO {};", ident(&old.name), ident(name))
			} else {
				String::new()
			}
		});

		let connection_limit_clause =
			parms.connection_limit.map_or(String::new(), |connection_limit| {
				format!("CONNECTION LIMIT {}", connection_limit)
			});

		let password_clause =
			parms.password.as_ref().map_or(String::new(), |p| format!("PASSWORD {}", literal(p)));

		let valid_until_clause = parms
			.valid_until
			.as_ref()
			.map_or(String::new(), |v| format!("VALID UNTIL {}", literal(v)));

		let config_clause = if !parms.config.is_empty() {
			parms
				.config
				.iter()
				.filter_map(|c| {
					let k = &c.path;
					let v = c.value.as_ref().map(|s| s.as_str());
					if k.is_empty() {
						return None;
					}
					match role::Operation::try_from(c.op) {
						Ok(role::Operation::Add) | Ok(role::Operation::Replace) => Some(format!(
							"ALTER ROLE {} SET {} = {};",
							ident(&old.name),
							ident(k),
							literal(v.unwrap_or(""))
						)),
						Ok(role::Operation::Remove) => {
							Some(format!("ALTER ROLE {} RESET {};", ident(&old.name), ident(k)))
						}
						_ => None,
					}
				})
				.collect::<Vec<String>>()
				.join("")
		} else {
			String::new()
		};

		let sql = format!(
			r#"
            BEGIN;
                ALTER ROLE {}
                    {}
                    {}
                    {}
                    {}
                    {}
                    {}
                    {}
                    {}
                    {}
                    {};
                {}
                {}
            COMMIT;"#,
			ident(&old.name),
			is_superuser_clause,
			can_create_db_clause,
			can_create_role_clause,
			inherit_role_clause,
			can_login_clause,
			is_replication_role_clause,
			can_bypass_rls_clause,
			connection_limit_clause,
			password_clause,
			valid_until_clause,
			config_clause,
			name_sql
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			&&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await
	}

	pub async fn delete(
		conn: &DbPool,
		parms: role::DeleteRoleRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let role = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await?;

		let sql = format!("DROP ROLE {};", ident(&role.name));

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(role)
	}
}
