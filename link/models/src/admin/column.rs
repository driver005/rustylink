use crate::utils::{ident, literal, type_ident};
use common::{admin::column, DbPool};
use regex::Regex;
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

use crate::constant::COLUMNS_SQL;

use super::{
	admin::constant::{COLUMN_PRIVILEGE_SQL, DEFAULT_SYSTEM_SCHEMAS},
	utils::filter_by_list,
};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Column {
	pub table_id: i32,
	pub schema: String,
	pub table: String,
	pub id: String,
	pub ordinal_position: i32,
	pub name: String,
	pub default_value: String,
	pub data_type: String,
	pub format: String,
	pub is_identity: bool,
	pub identity_generation: column::IdentityGeneration,
	pub is_generated: bool,
	pub is_nullable: bool,
	pub is_updatable: bool,
	pub is_unique: bool,
	pub enums: Vec<String>,
	pub check: Option<String>,
	pub comment: Option<String>,
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

impl Column {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				let regexp = Regex::new(r"^(\d+)\.(\d+)$").unwrap();
				if !regexp.is_match(&id) {
					return Err(sea_orm::DbErr::Custom("Invalid format for column ID".to_string()));
				}
				let captures = regexp.captures(&id).unwrap();
				let table_id = captures.get(1).unwrap().as_str();
				let ordinal_pos = captures.get(2).unwrap().as_str();
				format!("{} AND c.oid = {} AND a.attnum = {};", COLUMNS_SQL, table_id, ordinal_pos)
			}
			RetrieveParams::ByName {
				name,
				table,
				schema,
			} => {
				format!(
					"{} AND a.attname = '{}' AND c.relname = '{}' AND nc.nspname = '{}';",
					COLUMNS_SQL, name, table, schema
				)
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(column) => return Ok(column),
			None => {
				let error_message = match parms {
					RetrieveParams::ById {
						id,
					} => format!("Cannot find a column with ID {}", id),
					RetrieveParams::ByName {
						name,
						table,
						schema,
					} => {
						format!("Cannot find a column named {} in table {}.{}", name, schema, table)
					}
				};

				return Err(sea_orm::DbErr::Custom(error_message));
			}
		};
	}

	pub async fn list(
		conn: &DbPool,
		parms: column::GetColumnsRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = format!(
			"WITH columns AS ({})
            SELECT *
            FROM columns
            WHERE true",
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
			sql.push_str(&format!(" AND schema {}", filter_clause));
		}

		if let Some(table_id_value) = parms.table_id {
			sql.push_str(&format!("  AND table_id = {}", table_id_value));
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
		parms: column::CreateColumnRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let table_data = crate::table::Table::retrieve(
			conn,
			&crate::table::RetrieveParams::ById {
				id: parms.table_id,
			},
		)
		.await?;

		// Construct default value clause
		let default_value_clause = if parms.is_identity.unwrap_or(false) {
			if parms.default_value.is_some() {
				return Err(sea_orm::DbErr::Custom(
					"Columns cannot both be identity and have a default value".to_string(),
				));
			}
			format!(
				"GENERATED {} AS IDENTITY",
				match parms.identity_generation() {
					IdentityGeneration::ByDefault => "BY DEFAULT",
					IdentityGeneration::Always => "ALWAYS",
				}
			)
		} else {
			match &parms.default_value {
				Some(value) => match parms.default_value_format() {
					DefaultValueFormat::Expression => format!("DEFAULT {}", value),
					DefaultValueFormat::Literal => format!("DEFAULT {}", literal(&value)),
				},
				None => String::new(),
			}
		};

		// Construct other clauses
		let is_nullable_clause = match parms.is_nullable {
			Some(true) => "NULL",
			Some(false) => "NOT NULL",
			None => "",
		};
		let is_primary_key_clause = if parms.is_primary_key.unwrap_or(false) {
			"PRIMARY KEY"
		} else {
			""
		};
		let is_unique_clause = if parms.is_unique.unwrap_or(false) {
			"UNIQUE"
		} else {
			""
		};
		let check_sql =
			parms.check.as_ref().map_or(String::new(), |check| format!("CHECK ({})", check));
		let comment_sql = parms.comment.as_ref().map_or(String::new(), |comment| {
			format!(
				"COMMENT ON COLUMN {}.{}.{} IS {};",
				ident(&table_data.schema),
				ident(&table_data.name),
				ident(&parms.name),
				literal(comment)
			)
		});

		// Construct the final SQL
		let sql = format!(
			"BEGIN;
	        ALTER TABLE {}.{} ADD COLUMN {} {}
	        {}
	        {}
	        {}
	        {}
	        {};
	        {}
	    COMMIT;",
			ident(&table_data.schema),
			ident(&table_data.name),
			ident(&parms.name),
			type_ident(&parms.r#type),
			default_value_clause,
			is_nullable_clause,
			is_primary_key_clause,
			is_unique_clause,
			check_sql,
			comment_sql
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the newly created column
		Self::retrieve(
			conn,
			&RetrieveParams::ByName {
				name: parms.name,
				table: table_data.name,
				schema: table_data.schema,
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: column::UpdateColumnRequest,
	) -> Result<Self, sea_orm::DbErr> {
		// Retrieve the old column data
		let old = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await?;

		// Construct SQL parts
		let name_sql = construct_name_sql(&old, &parms.name);
		let type_sql = construct_type_sql(&old, &parms.r#type);
		let default_value_sql = construct_default_value_sql(&old, &parms);
		let identity_sql = construct_identity_sql(&old, &parms);
		let is_nullable_sql = construct_is_nullable_sql(&old, &parms.is_nullable);
		let is_unique_sql = construct_is_unique_sql(&old, &parms.is_unique);
		let comment_sql = construct_comment_sql(&old, &parms.comment);
		let check_sql = construct_check_sql(&old, &parms.check);

		// Combine all SQL parts
		let sql = format!(
			"BEGIN;
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
        COMMIT;",
			is_nullable_sql,
			type_sql,
			default_value_sql,
			identity_sql,
			is_unique_sql,
			comment_sql,
			check_sql,
			name_sql // name_sql must be last
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the updated column
		Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await
	}

	pub async fn delete(
		conn: &DbPool,
		parms: column::DeleteColumnRequest,
	) -> Result<Self, sea_orm::DbErr> {
		// Retrieve the column
		let column = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id,
			},
		)
		.await?;

		// Construct the SQL query
		let sql = format!(
			"ALTER TABLE {}.{} DROP COLUMN {} {};",
			ident(&column.schema),
			ident(&column.table),
			ident(&column.name),
			if parms.cascade.unwrap_or(false) {
				"CASCADE"
			} else {
				"RESTRICT"
			}
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(column)
	}
}

// Helper functions for constructing SQL parts

fn construct_name_sql(old: &Column, new_name: &Option<String>) -> String {
	match new_name {
		Some(name) if name != &old.name => format!(
			"ALTER TABLE {}.{} RENAME COLUMN {} TO {};",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name),
			ident(name)
		),
		_ => String::new(),
	}
}

fn construct_type_sql(old: &Column, new_type: &Option<String>) -> String {
	match new_type {
		Some(r#type) => format!(
			"ALTER TABLE {}.{} ALTER COLUMN {} SET DATA TYPE {} USING {}::{};",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name),
			type_ident(r#type),
			ident(&old.name),
			type_ident(r#type)
		),
		None => String::new(),
	}
}

fn construct_default_value_sql(old: &Column, parms: &column::UpdateColumnRequest) -> String {
	if parms.drop_default.unwrap_or(false) {
		format!(
			"ALTER TABLE {}.{} ALTER COLUMN {} DROP DEFAULT;",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name)
		)
	} else if let Some(default_value) = &parms.default_value {
		let value = match parms.default_value_format() {
			DefaultValueFormat::Expression => default_value.clone(),
			DefaultValueFormat::Literal => literal(default_value),
		};
		format!(
			"ALTER TABLE {}.{} ALTER COLUMN {} SET DEFAULT {};",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name),
			value
		)
	} else {
		String::new()
	}
}

fn construct_identity_sql(old: &Column, parms: &column::UpdateColumnRequest) -> String {
	let base_sql = format!(
		"ALTER TABLE {}.{} ALTER COLUMN {}",
		ident(&old.schema),
		ident(&old.table),
		ident(&old.name)
	);

	match (old.is_identity, parms.is_identity) {
		(_, Some(false)) => format!("{} DROP IDENTITY IF EXISTS;", base_sql),
		(true, Some(true)) => match parms.identity_generation() {
			IdentityGeneration::ByDefault => {
				format!("{} SET GENERATED BY DEFAULT;", base_sql)
			}
			IdentityGeneration::Always => format!("{} SET GENERATED ALWAYS;", base_sql),
		},
		(false, Some(true)) => format!(
			"{} ADD GENERATED {} AS IDENTITY;",
			base_sql,
			match parms.identity_generation() {
				IdentityGeneration::ByDefault => "BY DEFAULT",
				IdentityGeneration::Always => "ALWAYS",
			}
		),
		_ => String::new(),
	}
}

fn construct_is_nullable_sql(old: &Column, is_nullable: &Option<bool>) -> String {
	match is_nullable {
		Some(true) => format!(
			"ALTER TABLE {}.{} ALTER COLUMN {} DROP NOT NULL;",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name)
		),
		Some(false) => format!(
			"ALTER TABLE {}.{} ALTER COLUMN {} SET NOT NULL;",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name)
		),
		None => String::new(),
	}
}

fn construct_is_unique_sql(old: &Column, is_unique: &Option<bool>) -> String {
	match (old.is_unique, is_unique) {
		(true, Some(false)) => format!(
			r#"
            DO $$
            DECLARE
                r record;
            BEGIN
                FOR r IN
                    SELECT conname FROM pg_constraint WHERE
                        contype = 'u'
                        AND cardinality(conkey) = 1
                        AND conrelid = {}
                        AND conkey[1] = {}
                LOOP
                    EXECUTE {} || ident(r.conname);
                END LOOP;
            END
            $$;
            "#,
			literal(&old.table_id.to_string()),
			literal(&old.ordinal_position.to_string()),
			literal(&format!(
				"ALTER TABLE {}.{} DROP CONSTRAINT ",
				ident(&old.schema),
				ident(&old.table)
			))
		),
		(false, Some(true)) => format!(
			"ALTER TABLE {}.{} ADD UNIQUE ({});",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name)
		),
		_ => String::new(),
	}
}

fn construct_comment_sql(old: &Column, comment: &Option<String>) -> String {
	comment.as_ref().map_or(String::new(), |c| {
		format!(
			"COMMENT ON COLUMN {}.{}.{} IS {};",
			ident(&old.schema),
			ident(&old.table),
			ident(&old.name),
			literal(c)
		)
	})
}

fn construct_check_sql(old: &Column, check: &Option<String>) -> String {
	match check {
		Some(check_condition) => format!(
			r#"
            DO $$
            DECLARE
            v_conname name;
            v_conkey int2[];
            BEGIN
            SELECT conname into v_conname FROM pg_constraint WHERE
                contype = 'c'
                AND cardinality(conkey) = 1
                AND conrelid = {}
                AND conkey[1] = {}
                ORDER BY oid asc
                LIMIT 1;

            IF v_conname IS NOT NULL THEN
                EXECUTE format('ALTER TABLE {}.{} DROP CONSTRAINT %s', v_conname);
            END IF;

            ALTER TABLE {}.{} ADD CONSTRAINT {} CHECK ({});

            SELECT conkey into v_conkey FROM pg_constraint WHERE conname = {};

            ASSERT v_conkey IS NOT NULL, 'error creating column constraint: check condition must refer to this column';
            ASSERT cardinality(v_conkey) = 1, 'error creating column constraint: check condition cannot refer to multiple columns';
            ASSERT v_conkey[1] = {}, 'error creating column constraint: check condition cannot refer to other columns';
            END
            $$;
            "#,
			literal(&old.table_id.to_string()),
			literal(&old.ordinal_position.to_string()),
			ident(&old.schema),
			ident(&old.table),
			ident(&old.schema),
			ident(&old.table),
			ident(&format!("{}_{}_ check", old.table, old.name)),
			check_condition,
			literal(&format!("{}_{}_check", old.table, old.name)),
			literal(&old.ordinal_position.to_string())
		),
		None => String::new(),
	}
}
