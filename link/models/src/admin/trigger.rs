use common::{admin::trigger, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, TRIGGER_SQL},
	utils::{filter_by_list, ident, literal},
};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Trigger {
	pub id: i32,
	pub table_id: i32,
	pub enabled_mode: trigger::EnabledMode,
	pub name: String,
	pub table: String,
	pub schema: String,
	pub condition: Option<String>,
	pub orientation: trigger::Orientation,
	pub activation: trigger::Activation,
	pub events: Vec<String>,
	pub function_schema: String,
	pub function_name: String,
	pub function_args: Vec<String>,
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

impl Trigger {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} WHERE pol.oid = {};", TRIGGER_SQL, literal(&id.to_string()))
			}
			RetrieveParams::ByName {
				name,
				table,
				schema,
			} => {
				format!(
					"{} WHERE name = {} AND schema = {} AND triggers.table = {};",
					TRIGGER_SQL,
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
				Err(sea_orm::DbErr::Custom("Invalid parameters on trigger retrieve".to_string()))
			}
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: trigger::GetTriggersRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = TRIGGER_SQL.to_string();

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

	pub async fn create(
		conn: &DbPool,
		parms: trigger::CreateTriggerRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let schema = parms.schema.unwrap_or("public".to_string());
		let function_schema = parms.function_schema.unwrap_or("public".to_string());

		let qualified_table_name = format!("{}.{}", ident(&schema), ident(&parms.table));
		let qualified_function_name =
			format!("{}.{}", ident(&function_schema), ident(&parms.function_name));
		let trigger_events = parms.events.join(" OR ");
		let trigger_orientation =
			parms.orientation.map_or(String::new(), |o| format!("FOR EACH {}", o));
		let trigger_condition = parms.condition.map_or(String::new(), |c| format!("WHEN ({})", c));
		let function_args = parms.function_args.join(",");

		let sql = format!(
			"CREATE TRIGGER {} {} {} ON {} {} {} EXECUTE FUNCTION {}({});",
			ident(&parms.name),
			parms.activation,
			trigger_events,
			qualified_table_name,
			trigger_orientation,
			trigger_condition,
			qualified_function_name,
			function_args
		);

		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Self::retrieve(
			conn,
			&RetrieveParams::ByName {
				name: parms.name,
				table: parms.table,
				schema,
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: trigger::UpdateTriggerRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let old = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await?;

		let enabled_mode_sql = match parms.enabled_mode() {
			trigger::EnabledMode::Origin => format!(
				"ALTER TABLE {}.{} ENABLE TRIGGER {};",
				ident(&old.schema),
				ident(&old.table),
				ident(&old.name)
			),
			trigger::EnabledMode::Disabled => format!(
				"ALTER TABLE {}.{} DISABLE TRIGGER {};",
				ident(&old.schema),
				ident(&old.table),
				ident(&old.name)
			),
			trigger::EnabledMode::Replica | trigger::EnabledMode::Always => format!(
				"ALTER TABLE {}.{} ENABLE {} TRIGGER {};",
				ident(&old.schema),
				ident(&old.table),
				parms.enabled_mode().as_str_name(),
				ident(&old.name)
			),
		};

		let name_sql = parms.name.as_ref().map_or(String::new(), |new_name| {
			if new_name != &old.name {
				format!(
					"ALTER TRIGGER {} ON {}.{} RENAME TO {};",
					ident(&old.name),
					ident(&old.schema),
					ident(&old.table),
					ident(new_name)
				)
			} else {
				String::new()
			}
		});

		let sql = format!("BEGIN; {}; {}; COMMIT;", enabled_mode_sql, name_sql);

		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

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
		parms: trigger::DeleteTriggerRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let trigger = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await?;

		let sql = format!(
			"DROP TRIGGER {} ON {}.{} {};",
			ident(&trigger.name),
			ident(&trigger.schema),
			ident(&trigger.table),
			if parms.cascade.unwrap_or(false) {
				"CASCADE"
			} else {
				""
			}
		);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(trigger)
	}
}
