use queue::MemQueue;
use sea_orm::DatabaseConnection;

pub struct Context {
	pub db: DatabaseConnection,
	pub queue: Box<MemQueue>,
}

impl Context {
	pub fn new(db: DatabaseConnection, queue: MemQueue) -> Context {
		Context {
			db,
			queue: Box::new(queue),
		}
	}

	pub fn get_queue(&self) -> Box<MemQueue> {
		self.queue.clone()
	}
}
