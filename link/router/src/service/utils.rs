use std::error::Error;

pub fn filter_by_list(
	include: Vec<String>,
	exclude: Vec<String>,
	default_exclude: Option<&[&str]>,
) -> Option<String> {
	let mut final_exclude = exclude;
	if let Some(default) = default_exclude {
		final_exclude.extend(default.iter().map(|s| s.to_string()));
	}

	if !include.is_empty() {
		return Some(format!(
			"IN ({})",
			include.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(",")
		));
	}

	if !final_exclude.is_empty() {
		return Some(format!(
			"NOT IN ({})",
			final_exclude.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(",")
		));
	}

	None
}

pub fn quote_ident(ident: &str) -> String {
	format!("\"{}\"", ident.replace("\"", "\"\""))
}

pub fn type_ident(type_: &str) -> String {
	if type_.ends_with("[]") {
		format!("{}[]", quote_ident(&type_[..type_.len() - 2]))
	} else if type_.contains('.') {
		type_.to_string()
	} else {
		quote_ident(type_)
	}
}

pub fn parse_column_id(id: &str) -> Result<(i32, i32), Box<dyn Error>> {
	let parts: Vec<&str> = id.split('.').collect();
	if parts.len() != 2 {
		return Err("Invalid format for column ID".into());
	}
	let table_id = parts[0].parse()?;
	let ordinal_pos = parts[1].parse()?;
	Ok((table_id, ordinal_pos))
}
