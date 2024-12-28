pub const DEFAULT_SYSTEM_SCHEMAS: &[&str] = &["information_schema", "pg_catalog", "pg_toast"];

pub const COLUMN_PRIVILEGE_SQL: &str = r#"
    -- Lists each column's privileges in the form of:
    --
    -- [
    --   {
    --     "column_id": "12345.1",
    --     "relation_schema": "public",
    --     "relation_name": "mytable",
    --     "column_name": "mycolumn",
    --     "privileges": [
    --       {
    --         "grantor": "postgres",
    --         "grantee": "myrole",
    --         "privilege_type": "SELECT",
    --         "is_grantable": false
    --       },
    --       ...
    --     ]
    --   },
    --   ...
    -- ]
    --
    -- Modified from information_schema.column_privileges. We try to be as close as
    -- possible to the view definition, obtained from:
    --
    -- select pg_get_viewdef('information_schema.column_privileges');
    --
    -- The main differences are:
    -- - we include column privileges for materialized views
    --   (reason for exclusion in information_schema.column_privileges:
    --    https://www.postgresql.org/message-id/9136.1502740844%40sss.pgh.pa.us)
    -- - we query a.attrelid and a.attnum to generate `column_id`
    -- - `table_catalog` is omitted
    -- - table_schema -> relation_schema, table_name -> relation_name
    --
    -- Column privileges are intertwined with table privileges in that table
    -- privileges override column privileges. E.g. if we do:
    --
    -- grant all on mytable to myrole;
    --
    -- Then `myrole` is granted privileges for ALL columns. Likewise, if we do:
    --
    -- grant all (id) on mytable to myrole;
    -- revoke all on mytable from myrole;
    --
    -- Then the grant on the `id` column is revoked.
    --
    -- This is unlike how grants for schemas and tables interact, where you need
    -- privileges for BOTH the schema the table is in AND the table itself in order
    -- to access the table.

    select (x.attrelid || '.' || x.attnum) as column_id,
        nc.nspname as relation_schema,
        x.relname as relation_name,
        x.attname as column_name,
        coalesce(
            jsonb_agg(
            jsonb_build_object(
                'grantor', u_grantor.rolname,
                'grantee', grantee.rolname,
                'privilege_type', x.prtype,
                'is_grantable', x.grantable
            )
            ),
            '[]'
        ) as privileges
    from
    (select pr_c.grantor,
            pr_c.grantee,
            a.attrelid,
            a.attnum,
            a.attname,
            pr_c.relname,
            pr_c.relnamespace,
            pr_c.prtype,
            pr_c.grantable,
            pr_c.relowner
    from
        (select pg_class.oid,
                pg_class.relname,
                pg_class.relnamespace,
                pg_class.relowner,
                (aclexplode(coalesce(pg_class.relacl, acldefault('r', pg_class.relowner)))).grantor as grantor,
                (aclexplode(coalesce(pg_class.relacl, acldefault('r', pg_class.relowner)))).grantee as grantee,
                (aclexplode(coalesce(pg_class.relacl, acldefault('r', pg_class.relowner)))).privilege_type as privilege_type,
                (aclexplode(coalesce(pg_class.relacl, acldefault('r', pg_class.relowner)))).is_grantable as is_grantable
        from pg_class
        where (pg_class.relkind = any (array['r',
                                            'v',
                                            'm',
                                            'f',
                                            'p'])) ) pr_c(oid, relname, relnamespace, relowner, grantor, grantee, prtype, grantable),
                                                        pg_attribute a
    where ((a.attrelid = pr_c.oid)
            and (a.attnum > 0)
            and (not a.attisdropped))
    union select pr_a.grantor,
                    pr_a.grantee,
                    pr_a.attrelid,
                    pr_a.attnum,
                    pr_a.attname,
                    c.relname,
                    c.relnamespace,
                    pr_a.prtype,
                    pr_a.grantable,
                    c.relowner
    from
        (select a.attrelid,
                a.attnum,
                a.attname,
                (aclexplode(coalesce(a.attacl, acldefault('c', cc.relowner)))).grantor as grantor,
                (aclexplode(coalesce(a.attacl, acldefault('c', cc.relowner)))).grantee as grantee,
                (aclexplode(coalesce(a.attacl, acldefault('c', cc.relowner)))).privilege_type as privilege_type,
                (aclexplode(coalesce(a.attacl, acldefault('c', cc.relowner)))).is_grantable as is_grantable
        from (pg_attribute a
                join pg_class cc on ((a.attrelid = cc.oid)))
        where ((a.attnum > 0)
                and (not a.attisdropped))) pr_a(attrelid, attnum, attname, grantor, grantee, prtype, grantable),
                                            pg_class c
    where ((pr_a.attrelid = c.oid)
            and (c.relkind = any (ARRAY['r',
                                        'v',
                                        'm',
                                        'f',
                                        'p'])))) x,
        pg_namespace nc,
        pg_authid u_grantor,
    (select pg_authid.oid,
            pg_authid.rolname
    from pg_authid
    union all select (0)::oid as oid,
                        'PUBLIC') grantee(oid, rolname)
    where ((x.relnamespace = nc.oid)
        and (x.grantee = grantee.oid)
        and (x.grantor = u_grantor.oid)
        and (x.prtype = any (ARRAY['INSERT',
                                    'SELECT',
                                    'UPDATE',
                                    'REFERENCES']))
        and (pg_has_role(u_grantor.oid, 'USAGE')
                or pg_has_role(grantee.oid, 'USAGE')
                or (grantee.rolname = 'PUBLIC')))
    group by column_id,
            nc.nspname,
            x.relname,
            x.attname
"#;

pub const COLUMNS_SQL: &str = r#"
    SELECT
    c.oid :: int8 AS table_id,
    nc.nspname AS schema,
    c.relname AS table,
    (c.oid || '.' || a.attnum) AS id,
    a.attnum AS ordinal_position,
    a.attname AS name,
    CASE
        WHEN a.atthasdef THEN pg_get_expr(ad.adbin, ad.adrelid)
        ELSE NULL
    END AS default_value,
    CASE
        WHEN t.typtype = 'd' THEN CASE
        WHEN bt.typelem <> 0 :: oid
        AND bt.typlen = -1 THEN 'ARRAY'
        WHEN nbt.nspname = 'pg_catalog' THEN format_type(t.typbasetype, NULL)
        ELSE 'USER-DEFINED'
        END
        ELSE CASE
        WHEN t.typelem <> 0 :: oid
        AND t.typlen = -1 THEN 'ARRAY'
        WHEN nt.nspname = 'pg_catalog' THEN format_type(a.atttypid, NULL)
        ELSE 'USER-DEFINED'
        END
    END AS data_type,
    COALESCE(bt.typname, t.typname) AS format,
    a.attidentity IN ('a', 'd') AS is_identity,
    CASE
        a.attidentity
        WHEN 'a' THEN 'ALWAYS'
        WHEN 'd' THEN 'BY DEFAULT'
        ELSE NULL
    END AS identity_generation,
    a.attgenerated IN ('s') AS is_generated,
    NOT (
        a.attnotnull
        OR t.typtype = 'd' AND t.typnotnull
    ) AS is_nullable,
    (
        c.relkind IN ('r', 'p')
        OR c.relkind IN ('v', 'f') AND pg_column_is_updatable(c.oid, a.attnum, FALSE)
    ) AS is_updatable,
    uniques.table_id IS NOT NULL AS is_unique,
    check_constraints.definition AS "check",
    array_to_json(
        array(
        SELECT
            enumlabel
        FROM
            pg_catalog.pg_enum enums
        WHERE
            enums.enumtypid = coalesce(bt.oid, t.oid)
            OR enums.enumtypid = coalesce(bt.typelem, t.typelem)
        ORDER BY
            enums.enumsortorder
        )
    ) AS enums,
    col_description(c.oid, a.attnum) AS comment
    FROM
    pg_attribute a
    LEFT JOIN pg_attrdef ad ON a.attrelid = ad.adrelid
    AND a.attnum = ad.adnum
    JOIN (
        pg_class c
        JOIN pg_namespace nc ON c.relnamespace = nc.oid
    ) ON a.attrelid = c.oid
    JOIN (
        pg_type t
        JOIN pg_namespace nt ON t.typnamespace = nt.oid
    ) ON a.atttypid = t.oid
    LEFT JOIN (
        pg_type bt
        JOIN pg_namespace nbt ON bt.typnamespace = nbt.oid
    ) ON t.typtype = 'd'
    AND t.typbasetype = bt.oid
    LEFT JOIN (
        SELECT DISTINCT ON (table_id, ordinal_position)
        conrelid AS table_id,
        conkey[1] AS ordinal_position
        FROM pg_catalog.pg_constraint
        WHERE contype = 'u' AND cardinality(conkey) = 1
    ) AS uniques ON uniques.table_id = c.oid AND uniques.ordinal_position = a.attnum
    LEFT JOIN (
        -- We only select the first column check
        SELECT DISTINCT ON (table_id, ordinal_position)
        conrelid AS table_id,
        conkey[1] AS ordinal_position,
        substring(
            pg_get_constraintdef(pg_constraint.oid, true),
            8,
            length(pg_get_constraintdef(pg_constraint.oid, true)) - 8
        ) AS "definition"
        FROM pg_constraint
        WHERE contype = 'c' AND cardinality(conkey) = 1
        ORDER BY table_id, ordinal_position, oid asc
    ) AS check_constraints ON check_constraints.table_id = c.oid AND check_constraints.ordinal_position = a.attnum
    WHERE
    NOT pg_is_other_temp_schema(nc.oid)
    AND a.attnum > 0
    AND NOT a.attisdropped
    AND (c.relkind IN ('r', 'v', 'm', 'f', 'p'))
    AND (
        pg_has_role(c.relowner, 'USAGE')
        OR has_column_privilege(
        c.oid,
        a.attnum,
        'SELECT, INSERT, UPDATE, REFERENCES'
        )
    )
"#;

pub const CONFIG_SQL: &str = r#"
    select
        name,
        setting,
        category,
        TRIM(split_part(category, '/', 1)) as group,
        TRIM(split_part(category, '/', 2)) as subgroup,
        unit,
        short_desc,
        extra_desc,
        context,
        vartype,
        source,
        min_val,
        max_val,
        enumvals,
        boot_val,
        reset_val,
        sourcefile,
        sourceline,
        pending_restart
    from
        pg_settings
    order by
        category,
        name
"#;

pub const EXTENSION_SQL: &str = r#"
    select
        e.name,
        n.nspname as schema,
        e.default_version,
        x.extversion as installed_version,
        e.comment
    from
        pg_available_extensions() e(name,
        default_version,
        comment)
    left join pg_extension x on
        e.name = x.extname
    left join pg_namespace n on
        x.extnamespace = n.oid
"#;

pub const FOREIGN_TABLE_SQL: &str = r#"
    select
        c.oid :: int8 as id,
        n.nspname as schema,
        c.relname as name,
        obj_description(c.oid) as comment
    from
        pg_class c
    join pg_namespace n on
        n.oid = c.relnamespace
    where
        c.relkind = 'f'
"#;

pub const FUNCTION_SQL: &str = r#"
    -- CTE with sane arg_modes, arg_names, and arg_types.
    -- All three are always of the same length.
    -- All three include all args, including OUT and TABLE args.
    with functions as (
    select
        *,
        -- proargmodes is null when all arg modes are IN
        coalesce(
            p.proargmodes,
            array_fill('i'::text, array[cardinality(coalesce(p.proallargtypes, p.proargtypes))])
        ) as arg_modes,
        -- proargnames is null when all args are unnamed
        coalesce(
            p.proargnames,
            array_fill(''::text, array[cardinality(coalesce(p.proallargtypes, p.proargtypes))])
        ) as arg_names,
        -- proallargtypes is null when all arg modes are IN
        coalesce(p.proallargtypes, p.proargtypes) as arg_types,
        array_cat(array_fill(false, array[pronargs - pronargdefaults]), array_fill(true, array[pronargdefaults])) as arg_has_defaults
    from
        pg_proc as p
    where
        p.prokind = 'f'
    )
    select
        f.oid::int8 as id,
        n.nspname as schema,
        f.proname as name,
        l.lanname as language,
        case
            when l.lanname = 'internal' then ''
            else f.prosrc
        end as definition,
        case
            when l.lanname = 'internal' then f.prosrc
            else pg_get_functiondef(f.oid)
        end as complete_statement,
        coalesce(f_args.args,
        '[]') as args,
        pg_get_function_arguments(f.oid) as argument_types,
        pg_get_function_identity_arguments(f.oid) as identity_argument_types,
        f.prorettype::int8 as return_type_id,
        pg_get_function_result(f.oid) as return_type,
        nullif(rt.typrelid::int8, 0) as return_type_relation_id,
        f.proretset as is_set_returning_function,
        case
            when f.provolatile = 'i' then 'IMMUTABLE'
            when f.provolatile = 's' then 'STABLE'
            when f.provolatile = 'v' then 'VOLATILE'
        end as behavior,
        f.prosecdef as security_definer,
        f_config.config_params as config_params
    from
        functions f
    left join pg_namespace n on
        f.pronamespace = n.oid
    left join pg_language l on
        f.prolang = l.oid
    left join pg_type rt on
        rt.oid = f.prorettype
    left join (
        select
            oid,
            jsonb_object_agg(param,
            value) filter (
            where param is not null) as config_params
        from
            (
            select
                oid,
                (string_to_array(unnest(proconfig), '='))[1] as param,
                (string_to_array(unnest(proconfig), '='))[2] as value
            from
                functions
        ) as t
        group by
            oid
    ) f_config on
        f_config.oid = f.oid
    left join (
        select
            oid,
            jsonb_agg(jsonb_build_object('mode',t2.mode,'name',name,'type_id',type_id,'has_default',has_default)
        ) as args
        from
            (
            select
                oid,
                unnest(arg_modes) as mode,
                unnest(arg_names) as name,
                unnest(arg_types)::int8 as type_id,
                unnest(arg_has_defaults) as has_default
            from
                functions
        ) as t1,
            lateral (
            select
                case
                    when t1.mode = 'i' then 'in'
                    when t1.mode = 'o' then 'out'
                    when t1.mode = 'b' then 'inout'
                    when t1.mode = 'v' then 'variadic'
                    else 'table'
                end as mode
        ) as t2
        group by
            t1.oid
    ) f_args on
        f_args.oid = f.oid
"#;

pub const INDEX_SQL: &str = r#"
    select
        idx.indexrelid::int8 as id,
        idx.indrelid::int8 as table_id,
        n.nspname as schema,
        idx.indnatts as number_of_attributes,
        idx.indnkeyatts as number_of_key_attributes,
        idx.indisunique as is_unique,
        idx.indisprimary as is_primary,
        idx.indisexclusion as is_exclusion,
        idx.indimmediate as is_immediate,
        idx.indisclustered as is_clustered,
        idx.indisvalid as is_valid,
        idx.indcheckxmin as check_xmin,
        idx.indisready as is_ready,
        idx.indislive as is_live,
        idx.indisreplident as is_replica_identity,
        idx.indkey as key_attributes,
        idx.indcollation as collation,
        idx.indclass as class,
        idx.indoption as options,
        idx.indpred as index_predicate,
        obj_description(idx.indexrelid,
        'pg_class') as comment,
        ix.indexdef as index_definition,
        am.amname as access_method,
        jsonb_agg(
        jsonb_build_object(
            'attribute_number',
        a.attnum,
        'attribute_name',
        a.attname,
        'data_type',
        format_type(a.atttypid,
        a.atttypmod)
        )
    order by
        a.attnum
        ) as index_attributes
    from
        pg_index idx
    join pg_class c on
        c.oid = idx.indexrelid
    join pg_namespace n on
        c.relnamespace = n.oid
    join pg_am am on
        c.relam = am.oid
    join pg_attribute a on
        a.attrelid = c.oid
        and a.attnum = any(idx.indkey)
    join pg_indexes ix on
        c.relname = ix.indexname
    group by
        idx.indexrelid,
        idx.indrelid,
        n.nspname,
        idx.indnatts,
        idx.indnkeyatts,
        idx.indisunique,
        idx.indisprimary,
        idx.indisexclusion,
        idx.indimmediate,
        idx.indisclustered,
        idx.indisvalid,
        idx.indcheckxmin,
        idx.indisready,
        idx.indislive,
        idx.indisreplident,
        idx.indkey,
        idx.indcollation,
        idx.indclass,
        idx.indoption,
        idx.indexprs,
        idx.indpred,
        ix.indexdef,
        am.amname
"#;

pub const MATERIALIZED_VIEW_SQL: &str = r#"
    select
        c.oid::int8 as id,
        n.nspname as schema,
        c.relname as name,
        c.relispopulated as is_populated,
        obj_description(c.oid) as comment
    from
        pg_class c
    join pg_namespace n on
        n.oid = c.relnamespace
    where
        c.relkind = 'm'
"#;

pub const POLICIE_SQL: &str = r#"
    select
        pol.oid :: int8 as id,
        n.nspname as schema,
        c.relname as table,
        c.oid :: int8 as table_id,
        pol.polname as name,
        case
            when pol.polpermissive then 'PERMISSIVE' :: text
            else 'RESTRICTIVE' :: text
        end as action,
        case
            when pol.polroles = '{0}' :: oid [] then array_to_json(
        string_to_array('public' :: text,
            '' :: text) :: name []
        )
            else array_to_json(
        array(
            select
                pg_roles.rolname
            from
                pg_roles
            where
                pg_roles.oid = any (pol.polroles)
            order by
                pg_roles.rolname
        )
        )
        end as roles,
        case
            pol.polcmd
        when 'r' :: "char" then 'SELECT' :: text
            when 'a' :: "char" then 'INSERT' :: text
            when 'w' :: "char" then 'UPDATE' :: text
            when 'd' :: "char" then 'DELETE' :: text
            when '*' :: "char" then 'ALL' :: text
            else null :: text
        end as command,
        pg_get_expr(pol.polqual,
        pol.polrelid) as definition,
        pg_get_expr(pol.polwithcheck,
        pol.polrelid) as check
    from
        pg_policy pol
    join pg_class c on
        c.oid = pol.polrelid
    left join pg_namespace n on
        n.oid = c.relnamespace
"#;

pub const PUBLICATION_SQL: &str = r#"
    select
        p.oid :: int8 as id,
        p.pubname as name,
        p.pubowner::regrole::text as owner,
        p.pubinsert as publish_insert,
        p.pubupdate as publish_update,
        p.pubdelete as publish_delete,
        p.pubtruncate as publish_truncate,
        case
            when p.puballtables then null
            else pr.tables
        end as tables
    from
        pg_catalog.pg_publication as p
    left join lateral (
        select
            coalesce(
            array_agg(
            json_build_object(
                'id',
            c.oid :: int8,
            'name',
            c.relname,
            'schema',
            nc.nspname
            )
            ),
            '{}'
        ) as tables
        from
            pg_catalog.pg_publication_rel as pr
        join pg_class as c on
            pr.prrelid = c.oid
        join pg_namespace as nc on
            c.relnamespace = nc.oid
        where
            pr.prpubid = p.oid
    ) as pr on
        1 = 1
"#;

pub const ROLE_SQL: &str = r#"
    -- TODO: Consider using pg_authid vs. pg_roles for unencrypted password field
    select
        oid :: int8 as id,
        rolname as name,
        rolsuper as is_superuser,
        rolcreatedb as can_create_db,
        rolcreaterole as can_create_role,
        rolinherit as inherit_role,
        rolcanlogin as can_login,
        rolreplication as is_replication_role,
        rolbypassrls as can_bypass_rls,
        (
        select
            COUNT(*)
        from
            pg_stat_activity
        where
            pg_roles.rolname = pg_stat_activity.usename
    ) as active_connections,
        case
            when rolconnlimit = -1 then current_setting('max_connections') :: int8
            else rolconnlimit
        end as connection_limit,
        rolpassword as password,
        rolvaliduntil as valid_until,
        rolconfig as config
    from
        pg_roles
"#;

pub const SCHEMA_SQL: &str = r#"
    -- Adapted from information_schema.schemata

    select
        n.oid::int8 as id,
        n.nspname as name,
        u.rolname as owner
    from
        pg_namespace n,
        pg_roles u
    where
        n.nspowner = u.oid
        and (
        pg_has_role(n.nspowner,
        'USAGE')
            or has_schema_privilege(n.oid,
            'CREATE, USAGE')
    )
        and not pg_catalog.starts_with(n.nspname,
        'pg_temp_')
        and not pg_catalog.starts_with(n.nspname,
        'pg_toast_temp_')
"#;

pub const TABLE_PRIVILEGE_SQL: &str = r#"
    -- Despite the name `table_privileges`, this includes other kinds of relations:
    -- views, matviews, etc. "Relation privileges" just doesn't roll off the tongue.
    --
    -- For each relation, get its relacl in a jsonb format,
    -- e.g.
    --
    -- '{postgres=arwdDxt/postgres}'
    --
    -- becomes
    --
    -- [
    --   {
    --     "grantee": "postgres",
    --     "grantor": "postgres",
    --     "is_grantable": false,
    --     "privilege_type": "INSERT"
    --   },
    --   ...
    -- ]
    select
        c.oid as relation_id,
        nc.nspname as schema,
        c.relname as name,
        case
            when c.relkind = 'r' then 'table'
            when c.relkind = 'v' then 'view'
            when c.relkind = 'm' then 'materialized_view'
            when c.relkind = 'f' then 'foreign_table'
            when c.relkind = 'p' then 'partitioned_table'
        end as kind,
        coalesce(
            jsonb_agg(
                jsonb_build_object(
                    'grantor',
                    grantor.rolname,
                    'grantee',
                    grantee.rolname,
                    'privilege_type',
                    _priv.privilege_type,
                    'is_grantable',
                    _priv.is_grantable
                )
            ) filter (
            where _priv is not null),
            '[]'
        ) as privileges
        from
            pg_class c
        join pg_namespace as nc
        on
            nc.oid = c.relnamespace
        left join lateral (
            select
                grantor,
                grantee,
                privilege_type,
                is_grantable
            from
                aclexplode(coalesce(c.relacl,
                acldefault('r',
                c.relowner)))
        ) as _priv on
            true
        left join pg_roles as grantor
        on
            grantor.oid = _priv.grantor
        left join (
            select
                pg_roles.oid,
                pg_roles.rolname
            from
                pg_roles
        union all
            select
                (0)::oid as oid,
                'PUBLIC'
        ) as grantee (oid,
            rolname)
        on
            grantee.oid = _priv.grantee
        where
            c.relkind in ('r', 'v', 'm', 'f', 'p')
            and not pg_is_other_temp_schema(c.relnamespace)
            and (
            pg_has_role(c.relowner,
            'USAGE')
                or has_table_privilege(
            c.oid,
                'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'
            )
                    or has_any_column_privilege(c.oid,
                    'SELECT, INSERT, UPDATE, REFERENCES')
        )
        group by
            c.oid,
            nc.nspname,
            c.relname,
            c.relkind
"#;

pub const TABLE_RELATION_SQL: &str = r#"
    -- Adapted from
    -- https://github.com/PostgREST/postgrest/blob/f9f0f79fa914ac00c11fbf7f4c558e14821e67e2/src/PostgREST/SchemaCache.hs#L722
    WITH
    pks_uniques_cols AS (
    SELECT
        connamespace,
        conrelid,
        jsonb_agg(column_info.cols) AS cols
    FROM
        pg_constraint
    JOIN LATERAL (
        SELECT
            array_agg(cols.attname
        ORDER BY
            cols.attnum) AS cols
        FROM
            (
            SELECT
                UNNEST(conkey) AS col) _
        JOIN pg_attribute cols ON
            cols.attrelid = conrelid
            AND cols.attnum = col
    ) column_info ON
        TRUE
    WHERE
        contype IN ('p', 'u')
        AND
        connamespace::regnamespace::text <> 'pg_catalog'
    GROUP BY
        connamespace,
        conrelid
    )
    SELECT
        traint.conname AS foreign_key_name,
        ns1.nspname AS SCHEMA,
        tab.relname AS relation,
        column_info.cols AS columns,
        ns2.nspname AS referenced_schema,
        other.relname AS referenced_relation,
        column_info.refs AS referenced_columns,
        (column_info.cols IN (
        SELECT
            *
        FROM
            jsonb_array_elements(pks_uqs.cols))) AS is_one_to_one
    FROM
        pg_constraint traint
    JOIN LATERAL (
        SELECT
            jsonb_agg(cols.attname
        ORDER BY
            ord) AS cols,
            jsonb_agg(refs.attname
        ORDER BY
            ord) AS refs
        FROM
            UNNEST(traint.conkey,
            traint.confkey) WITH ORDINALITY AS _(col,
            REF,
            ord)
        JOIN pg_attribute cols ON
            cols.attrelid = traint.conrelid
                AND cols.attnum = col
            JOIN pg_attribute refs ON
                refs.attrelid = traint.confrelid
                AND refs.attnum = REF
    ) AS column_info ON
        TRUE
    JOIN pg_namespace ns1 ON
        ns1.oid = traint.connamespace
    JOIN pg_class tab ON
        tab.oid = traint.conrelid
    JOIN pg_class other ON
        other.oid = traint.confrelid
    JOIN pg_namespace ns2 ON
        ns2.oid = other.relnamespace
    LEFT JOIN pks_uniques_cols pks_uqs ON
        pks_uqs.connamespace = traint.connamespace
        AND pks_uqs.conrelid = traint.conrelid
    WHERE
        traint.contype = 'f'
        AND traint.conparentid = 0
"#;

pub const TABLE_SQL: &str = r#"
    select
        c.oid :: int8 as id,
        nc.nspname as schema,
        c.relname as name,
        c.relrowsecurity as rls_enabled,
        c.relforcerowsecurity as rls_forced,
        case
            when c.relreplident = 'd' then 'DEFAULT'
            when c.relreplident = 'i' then 'INDEX'
            when c.relreplident = 'f' then 'FULL'
            else 'NOTHING'
        end as replica_identity,
        pg_total_relation_size(format('%I.%I',
        nc.nspname,
        c.relname)) :: int8 as bytes,
        pg_size_pretty(
        pg_total_relation_size(format('%I.%I',
        nc.nspname,
        c.relname))
    ) as size,
        pg_stat_get_live_tuples(c.oid) as live_rows_estimate,
        pg_stat_get_dead_tuples(c.oid) as dead_rows_estimate,
        obj_description(c.oid) as comment,
        coalesce(pk.primary_keys,
        '[]') as primary_keys,
        coalesce(
        jsonb_agg(relationships) filter (
        where relationships is not null),
        '[]'
    ) as relationships
    from
        pg_namespace nc
    join pg_class c on
        nc.oid = c.relnamespace
    left join (
        select
            table_id,
            jsonb_agg(_pk.*) as primary_keys
        from
            (
            select
                n.nspname as schema,
                c.relname as table_name,
                a.attname as name,
                c.oid :: int8 as table_id
            from
                pg_index i,
                pg_class c,
                pg_attribute a,
                pg_namespace n
            where
                i.indrelid = c.oid
                and c.relnamespace = n.oid
                and a.attrelid = c.oid
                and a.attnum = any (i.indkey)
                    and i.indisprimary
        ) as _pk
        group by
            table_id
    ) as pk
    on
        pk.table_id = c.oid
    left join (
        select
            c.oid :: int8 as id,
            c.conname as constraint_name,
            nsa.nspname as source_schema,
            csa.relname as source_table_name,
            sa.attname as source_column_name,
            nta.nspname as target_table_schema,
            cta.relname as target_table_name,
            ta.attname as target_column_name
        from
            pg_constraint c
        join (
        pg_attribute sa
        join pg_class csa on
            sa.attrelid = csa.oid
        join pg_namespace nsa on
            csa.relnamespace = nsa.oid
        ) on
            sa.attrelid = c.conrelid
                and sa.attnum = any (c.conkey)
            join (
        pg_attribute ta
            join pg_class cta on
                ta.attrelid = cta.oid
            join pg_namespace nta on
                cta.relnamespace = nta.oid
        ) on
                ta.attrelid = c.confrelid
                    and ta.attnum = any (c.confkey)
                where
                    c.contype = 'f'
    ) as relationships
    on
        (relationships.source_schema = nc.nspname
            and relationships.source_table_name = c.relname)
        or (relationships.target_table_schema = nc.nspname
            and relationships.target_table_name = c.relname)
    where
        c.relkind in ('r', 'p')
        and not pg_is_other_temp_schema(nc.oid)
        and (
        pg_has_role(c.relowner,
        'USAGE')
            or has_table_privilege(
        c.oid,
            'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'
        )
                or has_any_column_privilege(c.oid,
                'SELECT, INSERT, UPDATE, REFERENCES')
    )
    group by
        c.oid,
        c.relname,
        c.relrowsecurity,
        c.relforcerowsecurity,
        c.relreplident,
        nc.nspname,
        pk.primary_keys
"#;

pub const TRIGGER_SQL: &str = r#"
    select
        pg_t.oid as id,
        pg_t.tgrelid as table_id,
        case
            when pg_t.tgenabled = 'D' then 'DISABLED'
            when pg_t.tgenabled = 'O' then 'ORIGIN'
            when pg_t.tgenabled = 'R' then 'REPLICA'
            when pg_t.tgenabled = 'A' then 'ALWAYS'
        end as enabled_mode,
        (
        STRING_TO_ARRAY(
            ENCODE(pg_t.tgargs, 'escape'), '\000'
        )
    )[:pg_t.tgnargs] as function_args,
        is_t.trigger_name as name,
        is_t.event_object_table as table,
        is_t.event_object_schema as schema,
        is_t.action_condition as condition,
        is_t.action_orientation as orientation,
        is_t.action_timing as activation,
        ARRAY_AGG(is_t.event_manipulation)::text[] as events,
        pg_p.proname as function_name,
        pg_n.nspname as function_schema
    from
        pg_trigger as pg_t
    join
    pg_class as pg_c
    on
        pg_t.tgrelid = pg_c.oid
    join information_schema.triggers as is_t
    on
        is_t.trigger_name = pg_t.tgname
        and pg_c.relname = is_t.event_object_table
        and pg_c.relnamespace = is_t.event_object_schema::regnamespace
    join pg_proc as pg_p
    on
        pg_t.tgfoid = pg_p.oid
    join pg_namespace as pg_n
    on
        pg_p.pronamespace = pg_n.oid
    group by
        pg_t.oid,
        pg_t.tgrelid,
        pg_t.tgenabled,
        pg_t.tgargs,
        pg_t.tgnargs,
        is_t.trigger_name,
        is_t.event_object_table,
        is_t.event_object_schema,
        is_t.action_condition,
        is_t.action_orientation,
        is_t.action_timing,
        pg_p.proname,
        pg_n.nspname
"#;

pub const TYPE_SQL: &str = r#"
    select
        t.oid::int8 as id,
        t.typname as name,
        n.nspname as schema,
        format_type (t.oid, null) as format,
        coalesce(t_enums.enums, '[]') as enums,
        coalesce(t_attributes.attributes, '[]') as attributes,
        obj_description (t.oid, 'pg_type') as comment
    from
        pg_type t
        left join pg_namespace n on n.oid = t.typnamespace
        left join (
            select
            enumtypid,
            jsonb_agg(enumlabel order by enumsortorder) as enums
            from
            pg_enum
            group by
            enumtypid
        ) as t_enums on t_enums.enumtypid = t.oid
        left join (
            select
            oid,
            jsonb_agg(
                jsonb_build_object('name', a.attname, 'type_id', a.atttypid::int8)
                order by a.attnum asc
            ) as attributes
            from
            pg_class c
            join pg_attribute a on a.attrelid = c.oid
            where
            c.relkind = 'c' and not a.attisdropped
            group by
            c.oid
        ) as t_attributes on t_attributes.oid = t.typrelid
    where
        (
            t.typrelid = 0
            or (
            select
                c.relkind = 'c'
            from
                pg_class c
            where
                c.oid = t.typrelid
            )
        )
"#;

pub const VERSION_SQL: &str = r#"
    SELECT
    version(),
    current_setting('server_version_num') :: int8 AS version_number,
    (
        SELECT
        COUNT(*) AS active_connections
        FROM
        pg_stat_activity
    ) AS active_connections,
    current_setting('max_connections') :: int8 AS max_connections
"#;

pub const VIEW_SQL: &str = r#"
    SELECT
        c.oid :: int8 AS id,
        n.nspname AS schema,
        c.relname AS name,
        -- See definition of information_schema.views
        (pg_relation_is_updatable(c.oid, false) & 20) = 20 AS is_updatable,
        obj_description(c.oid) AS comment
    FROM
        pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE
        c.relkind = 'v'
"#;

pub const VIEW_KEY_DEPENDENCIE_SQL: &str = r#"
    -- Adapted from
    -- https://github.com/PostgREST/postgrest/blob/f9f0f79fa914ac00c11fbf7f4c558e14821e67e2/src/PostgREST/SchemaCache.hs#L820
    with recursive
    pks_fks as (
    -- pk + fk referencing col
    select
        contype::text as contype,
        conname,
        array_length(conkey, 1) as ncol,
        conrelid as resorigtbl,
        col as resorigcol,
        ord
    from pg_constraint
    left join lateral unnest(conkey) with ordinality as _(col, ord) on true
    where contype IN ('p', 'f')
    union
    -- fk referenced col
    select
        concat(contype, '_ref') as contype,
        conname,
        array_length(confkey, 1) as ncol,
        confrelid,
        col,
        ord
    from pg_constraint
    left join lateral unnest(confkey) with ordinality as _(col, ord) on true
    where contype='f'
    ),
    views as (
    select
        c.oid       as view_id,
        n.nspname   as view_schema,
        c.relname   as view_name,
        r.ev_action as view_definition
    from pg_class c
    join pg_namespace n on n.oid = c.relnamespace
    join pg_rewrite r on r.ev_class = c.oid
    where c.relkind in ('v', 'm') and n.nspname not in (__EXCLUDED_SCHEMAS)
    ),
    transform_json as (
    select
        view_id, view_schema, view_name,
        -- the following formatting is without indentation on purpose
        -- to allow simple diffs, with less whitespace noise
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        regexp_replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
        replace(
            view_definition::text,
        -- This conversion to json is heavily optimized for performance.
        -- The general idea is to use as few regexp_replace() calls as possible.
        -- Simple replace() is a lot faster, so we jump through some hoops
        -- to be able to use regexp_replace() only once.
        -- This has been tested against a huge schema with 250+ different views.
        -- The unit tests do NOT reflect all possible inputs. Be careful when changing this!
        -- -----------------------------------------------
        -- pattern           | replacement         | flags
        -- -----------------------------------------------
        -- `<>` in pg_node_tree is the same as `null` in JSON, but due to very poor performance of json_typeof
        -- we need to make this an empty array here to prevent json_array_elements from throwing an error
        -- when the targetList is null.
        -- We'll need to put it first, to make the node protection below work for node lists that start with
        -- null: `(<> ...`, too. This is the case for coldefexprs, when the first column does not have a default value.
            '<>'              , '()'
        -- `,` is not part of the pg_node_tree format, but used in the regex.
        -- This removes all `,` that might be part of column names.
        ), ','               , ''
        -- The same applies for `{` and `}`, although those are used a lot in pg_node_tree.
        -- We remove the escaped ones, which might be part of column names again.
        ), E'\\{'            , ''
        ), E'\\}'            , ''
        -- The fields we need are formatted as json manually to protect them from the regex.
        ), ' :targetList '   , ',"targetList":'
        ), ' :resno '        , ',"resno":'
        ), ' :resorigtbl '   , ',"resorigtbl":'
        ), ' :resorigcol '   , ',"resorigcol":'
        -- Make the regex also match the node type, e.g. `{QUERY ...`, to remove it in one pass.
        ), '{'               , '{ :'
        -- Protect node lists, which start with `({` or `((` from the greedy regex.
        -- The extra `{` is removed again later.
        ), '(('              , '{(('
        ), '({'              , '{({'
        -- This regex removes all unused fields to avoid the need to format all of them correctly.
        -- This leads to a smaller json result as well.
        -- Removal stops at `,` for used fields (see above) and `}` for the end of the current node.
        -- Nesting can't be parsed correctly with a regex, so we stop at `{` as well and
        -- add an empty key for the followig node.
        ), ' :[^}{,]+'       , ',"":'              , 'g'
        -- For performance, the regex also added those empty keys when hitting a `,` or `}`.
        -- Those are removed next.
        ), ',"":}'           , '}'
        ), ',"":,'           , ','
        -- This reverses the "node list protection" from above.
        ), '{('              , '('
        -- Every key above has been added with a `,` so far. The first key in an object doesn't need it.
        ), '{,'              , '{'
        -- pg_node_tree has `()` around lists, but JSON uses `[]`
        ), '('               , '['
        ), ')'               , ']'
        -- pg_node_tree has ` ` between list items, but JSON uses `,`
        ), ' '             , ','
        )::json as view_definition
    from views
    ),
    target_entries as(
    select
        view_id, view_schema, view_name,
        json_array_elements(view_definition->0->'targetList') as entry
    from transform_json
    ),
    results as(
    select
        view_id, view_schema, view_name,
        (entry->>'resno')::int as view_column,
        (entry->>'resorigtbl')::oid as resorigtbl,
        (entry->>'resorigcol')::int as resorigcol
    from target_entries
    ),
    -- CYCLE detection according to PG docs: https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-CYCLE
    -- Can be replaced with CYCLE clause once PG v13 is EOL.
    recursion(view_id, view_schema, view_name, view_column, resorigtbl, resorigcol, is_cycle, path) as(
    select
        r.*,
        false,
        ARRAY[resorigtbl]
    from results r
    where view_schema not in (__EXCLUDED_SCHEMAS)
    union all
    select
        view.view_id,
        view.view_schema,
        view.view_name,
        view.view_column,
        tab.resorigtbl,
        tab.resorigcol,
        tab.resorigtbl = ANY(path),
        path || tab.resorigtbl
    from recursion view
    join results tab on view.resorigtbl=tab.view_id and view.resorigcol=tab.view_column
    where not is_cycle
    ),
    repeated_references as(
    select
        view_id,
        view_schema,
        view_name,
        resorigtbl,
        resorigcol,
        array_agg(attname) as view_columns
    from recursion
    join pg_attribute vcol on vcol.attrelid = view_id and vcol.attnum = view_column
    group by
        view_id,
        view_schema,
        view_name,
        resorigtbl,
        resorigcol
    )
    select
    sch.nspname as table_schema,
    tbl.relname as table_name,
    rep.view_schema,
    rep.view_name,
    pks_fks.conname as constraint_name,
    pks_fks.contype as constraint_type,
    jsonb_agg(
        jsonb_build_object('table_column', col.attname, 'view_columns', view_columns) order by pks_fks.ord
    ) as column_dependencies
    from repeated_references rep
    join pks_fks using (resorigtbl, resorigcol)
    join pg_class tbl on tbl.oid = rep.resorigtbl
    join pg_attribute col on col.attrelid = tbl.oid and col.attnum = rep.resorigcol
    join pg_namespace sch on sch.oid = tbl.relnamespace
    group by sch.nspname, tbl.relname,  rep.view_schema, rep.view_name, pks_fks.conname, pks_fks.contype, pks_fks.ncol
    -- make sure we only return key for which all columns are referenced in the view - no partial PKs or FKs
    having ncol = array_length(array_agg(row(col.attname, view_columns) order by pks_fks.ord), 1)
"#;
