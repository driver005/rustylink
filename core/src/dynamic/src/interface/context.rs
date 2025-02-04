#[derive(Debug)]
pub enum Type {
	GraphQL,
	Proto,
}

pub struct Context {
	r#type: Type,
}
