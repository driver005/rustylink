use sea_query::TableRef;

pub(crate) fn filter_hidden_tables(include_hidden_tables: bool, table: &str) -> bool {
	if include_hidden_tables {
		true
	} else {
		!table.starts_with('_')
	}
}

pub(crate) fn escape_rust_keyword<T>(string: T) -> String
where
	T: ToString,
{
	let string = string.to_string();
	if RUST_KEYWORDS.iter().any(|s| s.eq(&string)) {
		format!("r#{string}")
	} else if RUST_SPECIAL_KEYWORDS.iter().any(|s| s.eq(&string)) {
		format!("{string}_")
	} else {
		string
	}
}

pub(crate) const RUST_KEYWORDS: [&str; 49] = [
	"as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern", "false",
	"fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
	"return", "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use",
	"where", "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
	"try", "typeof", "unsized", "virtual", "yield",
];

pub(crate) const RUST_SPECIAL_KEYWORDS: [&str; 3] = ["crate", "Self", "self"];

pub(crate) fn unpack_table_ref(table_ref: &TableRef) -> String {
	match table_ref {
		TableRef::Table(tbl)
		| TableRef::SchemaTable(_, tbl)
		| TableRef::DatabaseSchemaTable(_, _, tbl)
		| TableRef::TableAlias(tbl, _)
		| TableRef::SchemaTableAlias(_, tbl, _)
		| TableRef::DatabaseSchemaTableAlias(_, _, tbl, _)
		| TableRef::SubQuery(_, tbl)
		| TableRef::ValuesList(_, tbl)
		| TableRef::FunctionCall(_, tbl) => tbl.to_string(),
	}
}

// pub(crate) fn add_line_break(content: proc_macro2::TokenStream) -> String {
// 	let file_parsed: syn::File = syn::parse2(content).unwrap();
// 	let blocks: Vec<String> =
// 		file_parsed.items.iter().enumerate().fold(Vec::new(), |mut acc, (i, item)| {
// 			let mut s = item.into_token_stream().to_string();
// 			if !acc.is_empty() && no_line_break_in_between(&file_parsed.items[i - 1], item) {
// 				let last = acc.swap_remove(acc.len() - 1);
// 				s = format!("{}{}", last, s);
// 			}
// 			acc.push(s);
// 			acc
// 		});
// 	replace_fully_qualified_spaces(blocks.join("\n\n"))
// }

// pub(crate) fn no_line_break_in_between(this: &syn::Item, that: &syn::Item) -> bool {
// 	matches!(
// 		(this, that),
// 		(syn::Item::Mod(_), syn::Item::Mod(_)) | (syn::Item::Use(_), syn::Item::Use(_))
// 	)
// }

// pub(crate) fn replace_fully_qualified_spaces(mut str: String) -> String {
// 	let targets = [
// 		("seaography :: macros :: ", "seaography::macros::"),
// 		("async_graphql :: ", "async_graphql::"),
// 	];
// 	for (from, to) in targets {
// 		str = str.replace(from, to);
// 	}
// 	str
// }
