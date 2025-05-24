use serde_json::Value;

#[derive(strum_macros::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum OperationType {
	Query,
	Mutation,
	Subscription,
}

#[derive(Clone, Debug)]
pub struct SelectionSet {
	pub operation: &'static str,
	pub alias: Option<&'static str>,
	pub fields: Option<Fields>,
	pub arguments: Option<Value>,
	pub is_union: bool,
}

type Fields = Vec<SelectionSet>;

pub fn tabs(level: usize) -> String {
	"\t".repeat(level)
}

pub struct QueryBuilder;

impl QueryBuilder {
	pub fn new(operation: OperationType, selection_set: &SelectionSet) -> String {
		Self::generate_field(
			0,
			SelectionSet {
				operation: operation.into(),
				alias: None,
				fields: Some(vec![selection_set.clone()]),
				arguments: None,
				is_union: false,
			},
		)
	}

	fn parse_arguments(args: &Value) -> String {
		match args {
			Value::Null => format!(""),
			Value::Bool(value) => format!("{}", value),
			Value::Number(number) => format!("{}", number),
			Value::String(value) => format!("\"{}\"", value),
			Value::Array(vec) => format!(
				"[{}]",
				vec.iter()
					.map(|i| Self::parse_arguments(i))
					.filter(|s| !s.is_empty())
					.collect::<Vec<String>>()
					.join(", ")
			),
			Value::Object(map) => map
				.iter()
				.map(|(key, value)| match value {
					Value::Object(_) => {
						format!("{}: {{{}}}", key, Self::parse_arguments(value))
					}
					Value::Null => format!(""),
					_ => format!("{}: {}", key, Self::parse_arguments(value)),
				})
				.filter(|s| !s.is_empty())
				.collect::<Vec<String>>()
				.join(", "),
		}
	}

	fn generate_field(level: usize, field: SelectionSet) -> String {
		let tabs = tabs(level);
		let operation = field.operation;
		let union = if field.is_union {
			"... on "
		} else {
			""
		};
		let alias = if let Some(alias) = field.alias {
			format!("{}: ", alias)
		} else {
			String::from("")
		};
		let arguments = if let Some(args) = field.arguments {
			format!("({})", Self::parse_arguments(&args))
		} else {
			String::from("")
		};
		let sub_field = if let Some(sf) = field.fields {
			let sub_field = Self::map_fields(level, sf).join("\n");
			format!(" {{\n{sub_field}\n{tabs}}}")
		} else {
			String::from("")
		};
		format!("{tabs}{union}{alias}{operation}{arguments}{sub_field}")
	}

	fn map_fields(level: usize, selection_sets: Fields) -> Vec<String> {
		selection_sets
			.clone()
			.into_iter()
			.map(|field| Self::generate_field(level + 1, field))
			.collect()
	}
}
