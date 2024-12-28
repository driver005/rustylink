use queue::MemQueue;
use sea_orm::DatabaseConnection;

pub struct Context {
	pub db: DatabaseConnection,
	pub queue: Box<MemQueue>,
}
