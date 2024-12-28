use common::{proto, DbPool};

use super::{
	admin::constant::{COLUMN_PRIVILEGE_SQL, DEFAULT_SYSTEM_SCHEMAS},
	utils::filter_by_list,
};

pub struct ColumnService {}

impl ColumnService {
	pub async fn retrieve(
		conn: &DbPool,
		parms: proto::GetColumnsRequest,
	) -> Result<proto::ColumnResponce, Error> {
		unimplemented!()
	}

	pub async fn list(
		conn: &DbPool,
		table_id: i32,
		parms: proto::GetColumnsRequest,
	) -> Result<Vec<proto::ColumnResponce>, Error> {
		unimplemented!()
	}

	pub async fn create(
		conn: &DbPool,
		parms: proto::CreateColumnRequest,
	) -> Result<proto::ColumnResponce, Error> {
		unimplemented!()
	}

	pub async fn update(
		conn: &DbPool,
		parms: proto::UpdateColumnRequest,
	) -> Result<proto::ColumnResponce, Error> {
		unimplemented!()
	}

	pub async fn delete(
		conn: &DbPool,
		parms: proto::DeleteColumnRequest,
	) -> Result<proto::ColumnResponce, Error> {
		unimplemented!()
	}

	pub async fn listColumnPrivilege(
		conn: &DbPool,
		parms: proto::GetColumnPrivilegesRequest,
	) -> Result<Vec<proto::ColumnPrivilegesResponse>, Error> {
		let mut sql = format!(
			"with column_privileges as ({})
            select *
            from column_privileges",
			COLUMN_PRIVILEGE_SQL
		);

		let filter = filter_by_list(
			parms.included_schemas,
			parms.excluded_schemas,
			if !parms.include_system_schemas {
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

		// let result: Vec<proto::ColumnPrivilegesResponse> =
		// 	sqlx::query_as(sql.as_str()).fetch_all(conn).await?;

		unimplemented!()
	}

	pub async fn grantColumnPrivilege(
		conn: &DbPool,
		parms: proto::GrantColumnPrivilegeRequest,
	) -> Result<proto::ColumnPrivilegesResponse, Error> {
		// let grant_statements: Vec<String> = parms.grants
		// 	.iter()
		// 	.map(|grant| {
		// 		let (relation_id, column_number) = grant.column_id.split_once('.').unwrap();
		// 		format!(
		// 			r#"
		//             select *
		//             from pg_attribute a
		//             where a.attrelid = {}
		//                 and a.attnum = {}
		//             into col;
		//             execute format(
		//                 'grant {} (%I) on %s to {} {}',
		//                 col.attname,
		//                 col.attrelid::regclass
		//             );"#,
		// 			relation_id,
		// 			column_number,
		// 			grant.privilege_type,
		// 			if grant.grantee.to_lowercase() == "public" {
		// 				"public".to_string()
		// 			} else {
		// 				format!("\"{}\"", grant.grantee)
		// 			},
		// 			if grant.is_grantable {
		// 				"with grant option"
		// 			} else {
		// 				""
		// 			}
		// 		)
		// 	})
		// 	.collect();

		// let grant_sql = format!(
		// 	r#"
		//     do $$
		//     declare
		//     col record;
		//     begin
		//     {}
		//     end $$;
		//     "#,
		// 	grant_statements.join("\n")
		// );

		// sqlx::query(&grant_sql).execute(conn).await?;

		// // Return the updated column privileges for modified columns.
		// let column_ids: Vec<&str> = grants.iter().map(|grant| grant.column_id.as_str()).collect();
		// let column_ids_list = column_ids.join(",");

		// let results = sqlx::query_as!(
		// 	PostgresColumnPrivileges,
		// 	r#"
		// with column_privileges as (
		//     {0}
		// )
		// select *
		// from column_privileges
		// where column_id in ({1})
		// "#,
		// 	COLUMN_PRIVILEGES_SQL,
		// 	column_ids_list
		// )
		// .fetch_all(conn)
		// .await?;

		// Ok(results)
		unimplemented!()
	}

	pub async fn revokeColumnPrivilege(
		conn: &DbPool,
		parms: proto::RevokeColumnPrivilegeRequest,
	) -> Result<proto::ColumnPrivilegesResponse, Error> {
		unimplemented!()
	}
}
