use std::fmt;

#[derive(Debug, PartialEq)]
pub enum FilterOperator<T> {
	Eq(T),
	Gt(T),
	Gte(T),
	Lt(T),
	Lte(T),
	Neq(T),
	Like(T),
	ILike(T),
	Match(T),
	IMatch(T),
	In(Vec<T>),
	Is(T),
	IsDistinct(T),
	Fts(T),
	Plfts(T),
	Phfts(T),
	Wfts(T),
	Cs(T),
	Cd(T),
	Ov(T),
	Sl(T),
	Sr(T),
	Nxr(T),
	Nxl(T),
	Adj(T),
	Not(Box<FilterOperator<T>>),
	Or(Box<FilterOperator<T>>, Box<FilterOperator<T>>),
	And(Box<FilterOperator<T>>, Box<FilterOperator<T>>),
	All(Vec<T>),
	Any(Vec<T>),
}

impl<T: fmt::Display> fmt::Display for FilterOperator<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			FilterOperator::Eq(val) => write!(f, "= {}", val),
			FilterOperator::Gt(val) => write!(f, "> {}", val),
			FilterOperator::Gte(val) => write!(f, ">= {}", val),
			FilterOperator::Lt(val) => write!(f, "< {}", val),
			FilterOperator::Lte(val) => write!(f, "<= {}", val),
			FilterOperator::Neq(val) => write!(f, "!= {}", val),
			FilterOperator::Like(val) => write!(f, "LIKE {}", val),
			FilterOperator::ILike(val) => write!(f, "ILIKE {}", val),
			FilterOperator::Match(val) => write!(f, "~ {}", val),
			FilterOperator::IMatch(val) => write!(f, "~* {}", val),
			FilterOperator::In(vals) => write!(
				f,
				"IN ({})",
				vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
			),
			FilterOperator::Is(val) => write!(f, "IS {}", val),
			FilterOperator::IsDistinct(val) => write!(f, "IS DISTINCT FROM {}", val),
			FilterOperator::Fts(val) => write!(f, "@@ {}", val),
			FilterOperator::Plfts(val) => write!(f, "@@ {}", val),
			FilterOperator::Phfts(val) => write!(f, "@@ {}", val),
			FilterOperator::Wfts(val) => write!(f, "@@ {}", val),
			FilterOperator::Cs(val) => write!(f, "@> {}", val),
			FilterOperator::Cd(val) => write!(f, "<@ {}", val),
			FilterOperator::Ov(val) => write!(f, "&& {}", val),
			FilterOperator::Sl(val) => write!(f, "<< {}", val),
			FilterOperator::Sr(val) => write!(f, ">> {}", val),
			FilterOperator::Nxr(val) => write!(f, "&< {}", val),
			FilterOperator::Nxl(val) => write!(f, "&> {}", val),
			FilterOperator::Adj(val) => write!(f, "-|- {}", val),
			FilterOperator::Not(op) => write!(f, "NOT ({})", op),
			FilterOperator::Or(left, right) => write!(f, "({}) OR ({})", left, right),
			FilterOperator::And(left, right) => write!(f, "({}) AND ({})", left, right),
			FilterOperator::All(vals) => write!(
				f,
				"ALL ({})",
				vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
			),
			FilterOperator::Any(vals) => write!(
				f,
				"ANY ({})",
				vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
			),
		}
	}
}

impl<T> FilterOperator<T> {
	pub fn as_str(&self) -> &'static str {
		match self {
			FilterOperator::Eq(_) => "=",
			FilterOperator::Gt(_) => ">",
			FilterOperator::Gte(_) => ">=",
			FilterOperator::Lt(_) => "<",
			FilterOperator::Lte(_) => "<=",
			FilterOperator::Neq(_) => "<> or !=",
			FilterOperator::Like(_) => "LIKE",
			FilterOperator::ILike(_) => "ILIKE",
			FilterOperator::Match(_) => "~",
			FilterOperator::IMatch(_) => "~*",
			FilterOperator::In(_) => "IN",
			FilterOperator::Is(_) => "IS",
			FilterOperator::IsDistinct(_) => "IS DISTINCT FROM",
			FilterOperator::Fts(_) => "@@",
			FilterOperator::Plfts(_) => "@@",
			FilterOperator::Phfts(_) => "@@",
			FilterOperator::Wfts(_) => "@@",
			FilterOperator::Cs(_) => "@>",
			FilterOperator::Cd(_) => "<@",
			FilterOperator::Ov(_) => "&&",
			FilterOperator::Sl(_) => "<<",
			FilterOperator::Sr(_) => ">>",
			FilterOperator::Nxr(_) => "&<",
			FilterOperator::Nxl(_) => "&>",
			FilterOperator::Adj(_) => "-|-",
			FilterOperator::Not(_) => "NOT",
			FilterOperator::Or(_, _) => "OR",
			FilterOperator::And(_, _) => "AND",
			FilterOperator::All(_) => "ALL",
			FilterOperator::Any(_) => "ANY",
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			FilterOperator::Eq(_) => "eq",
			FilterOperator::Gt(_) => "gt",
			FilterOperator::Gte(_) => "gte",
			FilterOperator::Lt(_) => "lt",
			FilterOperator::Lte(_) => "lte",
			FilterOperator::Neq(_) => "neq",
			FilterOperator::Like(_) => "like",
			FilterOperator::ILike(_) => "ilike",
			FilterOperator::Match(_) => "match",
			FilterOperator::IMatch(_) => "imatch",
			FilterOperator::In(_) => "in",
			FilterOperator::Is(_) => "is",
			FilterOperator::IsDistinct(_) => "isdistinct",
			FilterOperator::Fts(_) => "fts",
			FilterOperator::Plfts(_) => "plfts",
			FilterOperator::Phfts(_) => "phfts",
			FilterOperator::Wfts(_) => "wfts",
			FilterOperator::Cs(_) => "cs",
			FilterOperator::Cd(_) => "cd",
			FilterOperator::Ov(_) => "ov",
			FilterOperator::Sl(_) => "sl",
			FilterOperator::Sr(_) => "sr",
			FilterOperator::Nxr(_) => "nxr",
			FilterOperator::Nxl(_) => "nxl",
			FilterOperator::Adj(_) => "adj",
			FilterOperator::Not(_) => "not",
			FilterOperator::Or(_, _) => "or",
			FilterOperator::And(_, _) => "and",
			FilterOperator::All(_) => "all",
			FilterOperator::Any(_) => "any",
		}
	}

	pub fn from_query_string(query: &str) -> Result<(), Box<dyn std::error::Error>> {
		let parts: Vec<&str> = query.split('.').collect();
		if parts.len() < 2 {
			return Err("Invalid query format".into());
		}

		let op = parts[parts.len() - 2];
		let value = parts[parts.len() - 1];

		println!("op: {}", op);
		println!("value: {}", value);

		// let parsed_value =
		// 	value.parse::<T>().map_err(|e| format!("Failed to parse value: {}", e))?;

		// match op {
		// 	"eq" => Ok(FilterOperator::Eq(parsed_value)),
		// 	"gt" => Ok(FilterOperator::Gt(parsed_value)),
		// 	"gte" => Ok(FilterOperator::Gte(parsed_value)),
		// 	"lt" => Ok(FilterOperator::Lt(parsed_value)),
		// 	"lte" => Ok(FilterOperator::Lte(parsed_value)),
		// 	"neq" => Ok(FilterOperator::Neq(parsed_value)),
		// 	"like" => Ok(FilterOperator::Like(parsed_value)),
		// 	"ilike" => Ok(FilterOperator::ILike(parsed_value)),
		// 	"is" => Ok(FilterOperator::Is(parsed_value)),
		// 	// Add more operators as needed
		// 	_ => Err(format!("Unsupported operator: {}", op).into()),
		// }

		Ok(())
	}
}
