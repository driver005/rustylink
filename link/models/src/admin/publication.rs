use common::{admin::publication, DbPool};
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

use crate::{
	admin::constant::PUBLICATION_SQL,
	utils::{ident, literal},
};

#[derive(
	Debug,
	serde::Serialize,
	serde::Deserialize,
	sea_orm::FromJsonQueryResult,
	sea_orm::FromQueryResult,
)]
pub struct Publication {
	pub id: i32,
	pub name: String,
	pub owner: String,
	pub publish_insert: bool,
	pub publish_update: bool,
	pub publish_delete: bool,
	pub publish_truncate: bool,
	pub tables: Vec<publication::PublicationTable>,
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

impl Publication {
	pub async fn retrieve(conn: &DbPool, parms: &RetrieveParams) -> Result<Self, sea_orm::DbErr> {
		let sql = match parms {
			RetrieveParams::ById {
				id,
			} => {
				format!("{} WHERE p.oid = {};", PUBLICATION_SQL, literal(id))
			}
			RetrieveParams::ByName {
				name,
			} => {
				format!("{} WHERE p.pubname = {};", PUBLICATION_SQL, literal(name))
			}
		};

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => Err(sea_orm::DbErr::Custom(
				"Invalid parameters on publication retrieve".to_string(),
			)),
		}
	}

	pub async fn list(
		conn: &DbPool,
		parms: publication::GetPublicationsRequest,
	) -> Result<Vec<Self>, sea_orm::DbErr> {
		let mut sql = PUBLICATION_SQL.to_string();

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
		parms: publication::CreatePublicationRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let table_clause = if !parms.tables.is_empty() {
			format!(
				"FOR TABLE {}",
				parms
					.tables
					.iter()
					.map(|table| {
						if !table.contains('.') {
							return ident(table);
						}
						let parts: Vec<&str> = table.split('.').collect();
						let schema = parts[0];
						let table = parts[1..].join(".");
						format!("{}.{}", ident(schema), ident(&table))
					})
					.collect::<Vec<String>>()
					.join(",")
			)
		} else {
			"FOR ALL TABLES".to_string()
		};

		let mut publish_ops = Vec::new();

		if parms.publish_insert.unwrap_or(false) {
			publish_ops.push("insert");
		}
		if parms.publish_update.unwrap_or(false) {
			publish_ops.push("update");
		}
		if parms.publish_delete.unwrap_or(false) {
			publish_ops.push("delete");
		}
		if parms.publish_truncate.unwrap_or(false) {
			publish_ops.push("truncate");
		}

		let sql = format!(
			"CREATE PUBLICATION {} {}
            WITH (publish = '{}');",
			ident(&parms.name),
			table_clause,
			publish_ops.join(",")
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
		parms: publication::UpdatePublicationRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let sql = format!(
			r#"
	        do $$
	        declare
	            id oid := {};
	            old record;
	            new_name text := {};
	            new_owner text := {};
	            new_publish_insert bool := {};
	            new_publish_update bool := {};
	            new_publish_delete bool := {};
	            new_publish_truncate bool := {};
	            new_tables text := {};
	        begin
	            select * into old from pg_publication where oid = id;
	            if old is null then
	                raise exception 'Cannot find publication with id %', id;
	            end if;

	            if new_tables is null then
	                null;
	            elsif new_tables = 'all tables' then
	            if old.puballtables then
	                null;
	            else
	                -- Need to recreate because going from list of tables <-> all tables with alter is not possible.
	                execute(format('drop publication %1$I; create publication %1$I for all tables;', old.pubname));
	            end if;
	            else
	                if old.puballtables then
	                    -- Need to recreate because going from list of tables <-> all tables with alter is not possible.
	                    execute(format('drop publication %1$I; create publication %1$I;', old.pubname));
	                elsif exists(select from pg_publication_rel where prpubid = id) then
	                execute(
	                    format(
	                        'alter publication %I drop table %s',
	                        old.pubname,
	                        (select string_agg(prrelid::regclass::text, ', ') from pg_publication_rel where prpubid = id)
	                    )
	                );
	                end if;

	            -- At this point the publication must have no tables.

	                if new_tables != '' then
	                    execute(format('alter publication %I add table %s', old.pubname, new_tables));
	                end if;
	            end if;

	            execute(
	                format(
	                    'alter publication %I set (publish = %L);',
	                    old.pubname,
	                    concat_ws(
	                        ', ',
	                        case when coalesce(new_publish_insert, old.pubinsert) then 'insert' end,
	                        case when coalesce(new_publish_update, old.pubupdate) then 'update' end,
	                        case when coalesce(new_publish_delete, old.pubdelete) then 'delete' end,
	                        case when coalesce(new_publish_truncate, old.pubtruncate) then 'truncate' end
	                    )
	                )
	            );

	            execute(format('alter publication %I owner to %I;', old.pubname, coalesce(new_owner, old.pubowner::regrole::name)));

	            -- Using the same name in the rename clause gives an error, so only do it if the new name is different.
	            if new_name is not null and new_name != old.pubname then
	                execute(format('alter publication %I rename to %I;', old.pubname, coalesce(new_name, old.pubname)));
	            end if;

	            -- We need to retrieve the publication later, so we need a way to uniquely quote_identify which publication this is.
	            -- We can't rely on id because it gets changed if it got recreated.
	            -- We use a temp table to store the unique name - DO blocks can't return a value.
	            create temp table pg_meta_publication_tmp (name) on commit drop as values (coalesce(new_name, old.pubname));
	        end $$;

	        with publications as ({}) select * from publications where name = (select name from pg_meta_publication_tmp);
	        "#,
			literal(&parms.id.to_string()),
			parms.name.as_ref().map_or("null".to_string(), |n| literal(n)),
			parms.owner.as_ref().map_or("null".to_string(), |o| literal(o)),
			parms.publish_insert.map_or("null".to_string(), |p| p.to_string()),
			parms.publish_update.map_or("null".to_string(), |p| p.to_string()),
			parms.publish_delete.map_or("null".to_string(), |p| p.to_string()),
			parms.publish_truncate.map_or("null".to_string(), |p| p.to_string()),
			// literal(
			if !parms.tables.is_empty() {
				parms
					.tables
					.iter()
					.map(|table| {
						if !table.contains('.') {
							return ident(table);
						}
						let parts: Vec<&str> = table.split('.').collect();
						let schema = parts[0];
						let table = parts[1..].join(".");
						format!("{}.{}", ident(schema), ident(&table))
					})
					.collect::<Vec<String>>()
					.join(",")
			} else {
				"ALL TABLES".to_string()
			},
			// ),
			PUBLICATION_SQL,
		);

		let result =
			Self::find_by_statement(Statement::from_string(common::DBTYPE, sql)).one(conn).await?;

		match result {
			Some(data) => Ok(data),
			None => Err(sea_orm::DbErr::Custom("publication not found".to_string())),
		}
	}

	pub async fn delete(
		conn: &DbPool,
		parms: publication::DeletePublicationRequest,
	) -> Result<Self, sea_orm::DbErr> {
		let publication = Self::retrieve(
			conn,
			&RetrieveParams::ById {
				id: parms.id.clone(),
			},
		)
		.await?;

		let sql = format!("DROP PUBLICATION IF EXISTS {};", ident(&publication.name));

		// Execute the SQL
		conn.execute(Statement::from_string(common::DBTYPE, sql)).await?;

		Ok(publication)
	}
}
