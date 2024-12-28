use crate::{
	admin::constant::{DEFAULT_SYSTEM_SCHEMAS, FUNCTION_SQL},
	utils::{filter_by_list, ident, literal},
};
use common::{admin::function, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Function {
	pub id: i32,
	pub schema: String,
	pub name: String,
	pub language: String,
	pub definition: String,
	pub complete_statement: String,
	pub args: Vec<function::Argument>,
	pub argument_types: String,
	pub identity_argument_types: String,
	pub return_type_id: i32,
	pub return_type: String,
	pub return_type_relation_id: Option<i32>,
	pub is_set_returning_function: bool,
	pub behavior: i32,
	pub security_definer: bool,
	pub config_params: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum RetrieveParams {
	ById {
		id: String,
	},
	ByName {
		name: String,
		args: Vec<String>,
		schema: String,
	},
}

impl Function {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} WHERE id = {};", FUNCTION_SQL, literal(&id))
			}
			RetrieveParams::ByName {
				name,
				args,
				schema,
			} => generate_retrieve_function_sql(schema, name, args),
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(column) => Ok(column),
			None => {
				Err(sea_orm::DbErr::Custom("Invalid parameters on function retrieve".to_string()))
			}
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: function::GetFunctionsRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = FUNCTION_SQL.to_string();

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

	pub async fn create(
		conn: &DbPool,
		parms: function::CreateFunctionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let sql = generate_create_function_sql(&parms, false);

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve the updated column
		Self::retrieve(
			conn,
			&RetrieveParams::ByName {
				name: parms.name,
				args: parms.args,
				schema: parms.schema.unwrap_or("public".to_string()),
			},
		)
		.await
	}

	pub async fn update(
		conn: &DbPool,
		parms: function::UpdateFunctionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		// Retrieve current function
		let current_func = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		// Generate SQL statements
		let update_definition_sql = if let Some(def) = parms.definition {
			generate_create_function_sql(
				&function::CreateFunctionRequest {
					name: current_func.name.clone(),
					schema: Some(current_func.schema.clone()),
					args: current_func.argument_types.split(", ").map(String::from).collect(),
					definition: def,
					//TODO: config_params: current_func.config_params.clone(),
					behavior: current_func.behavior,
					language: Some(current_func.language),
					security_definer: Some(current_func.security_definer),
					return_type: Some(current_func.return_type),
					..Default::default()
				},
				true,
			)
		} else {
			String::new()
		};

		let update_name_sql = if let Some(new_name) = parms.name.clone() {
			if new_name != current_func.name {
				format!(
					"ALTER FUNCTION {}.{}({}) RENAME TO {};",
					ident(&current_func.schema),
					ident(&current_func.name),
					current_func.identity_argument_types,
					ident(&new_name)
				)
			} else {
				String::new()
			}
		} else {
			String::new()
		};

		let update_schema_sql = if let Some(new_schema) = parms.schema {
			if new_schema != current_func.schema {
				format!(
					"ALTER FUNCTION {}.{}({}) SET SCHEMA {};",
					ident(&current_func.schema),
					ident(&parms.name.clone().unwrap_or(current_func.name)),
					current_func.identity_argument_types,
					ident(&new_schema)
				)
			} else {
				String::new()
			}
		} else {
			String::new()
		};

		// Combine and execute SQL statements
		let sql = format!(
			"
                DO LANGUAGE plpgsql $$
                BEGIN
                    {};
                    {};
                    {};
                END;
                $$;
            ",
			update_definition_sql, update_name_sql, update_schema_sql
		);

		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Retrieve and return updated function
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
		parms: function::DeleteFunctionRequest,
	) -> Result<Self, sea_orm::DbErr> {
		// Retrieve the function
		let func = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.to_string(),
			},
		)
		.await?;

		// Construct the SQL statement
		let sql = format!(
			"DROP FUNCTION {}.{}({}) {};",
			ident(&func.schema),
			ident(&func.name),
			func.identity_argument_types,
			if parms.cascade.unwrap_or(false) {
				"CASCADE"
			} else {
				"RESTRICT"
			}
		);

		// Execute the SQL statement
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		// Return the removed function
		Ok(func)
	}
}

fn generate_create_function_sql(func: &function::CreateFunctionRequest, replace: bool) -> String {
	let replace_str = if replace {
		"OR REPLACE "
	} else {
		""
	};
	let args_str = func.args.join(", ");
	let security_str = if func.security_definer.unwrap_or(false) {
		"SECURITY DEFINER"
	} else {
		"SECURITY INVOKER"
	};

	let config_params_str = func
		.config_params
		.iter()
		.map(|(param, value)| {
			if value == "FROM CURRENT" {
				format!("SET {} FROM CURRENT", param)
			} else {
				format!("SET {} TO {}", param, value)
			}
		})
		.collect::<Vec<String>>()
		.join("\n");

	format!(
		r#"
        CREATE {} FUNCTION {}.{}({})
        RETURNS {}
        AS $func$
        {}
        $func$
        LANGUAGE {}
        {}
        CALLED ON NULL INPUT
        {}
        {};"#,
		replace_str,
		ident(&func.schema.clone().unwrap_or("public".to_string())),
		ident(&func.name),
		args_str,
		func.return_type.clone().unwrap_or("".to_string()),
		func.definition,
		func.language.clone().unwrap_or("".to_string()),
		func.behavior,
		security_str,
		config_params_str
	)
}

fn generate_retrieve_function_sql(schema: &str, name: &str, args: &[String]) -> String {
	let args_sql = if !args.is_empty() {
		format!(
			r#"(
                SELECT STRING_AGG(type_oid::text, ' ') FROM (
                    SELECT (
                        split_args.arr[
                            array_length(
                                split_args.arr,
                                1
                            )
                        ]::regtype::oid
                    ) AS type_oid FROM (
                        SELECT STRING_TO_ARRAY(
                            UNNEST(
                                ARRAY[{}]
                            ),
                            ' '
                        ) AS arr
                    ) AS split_args
                ) args
            )"#,
			args.iter().map(|arg| literal(arg)).collect::<Vec<_>>().join(", "),
		)
	} else {
		String::new()
	};

	format!(
		r#"{} JOIN pg_proc AS p ON id = p.oid WHERE schema = {} AND name = {} AND p.proargtypes::text = {}"#,
		enriched_functions_sql(),
		literal(schema),
		literal(name),
		args_sql
	)
}

fn enriched_functions_sql() -> String {
	format!(
		r#"
        WITH f AS (
            {}
        )
        SELECT
            f.*
        FROM f
        "#,
		FUNCTION_SQL
	)
}
