use sdk::{Builder, Config};

fn main() {
	Builder::build(Config::new().graphql_url("http://localhost:8000/graphql")).unwrap();
}
