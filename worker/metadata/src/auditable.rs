use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Auditable {
	pub owner_app: Option<String>,
	pub create_time: Option<u64>,
	pub update_time: Option<u64>,
	pub created_by: Option<String>,
	pub updated_by: Option<String>,
}

pub trait AuditableBehavior {
	fn owner_app(&self) -> Option<&String>;
	fn set_owner_app(&mut self, owner_app: String);

	fn create_time(&self) -> Option<u64>;
	fn set_create_time(&mut self, create_time: u64);

	fn update_time(&self) -> Option<u64>;
	fn set_update_time(&mut self, update_time: u64);

	fn created_by(&self) -> Option<&String>;
	fn set_created_by(&mut self, created_by: String);

	fn updated_by(&self) -> Option<&String>;
	fn set_updated_by(&mut self, updated_by: String);
}

impl AuditableBehavior for Auditable {
	fn owner_app(&self) -> Option<&String> {
		self.owner_app.as_ref()
	}

	fn set_owner_app(&mut self, owner_app: String) {
		self.owner_app = Some(owner_app);
	}

	fn create_time(&self) -> Option<u64> {
		self.create_time
	}

	fn set_create_time(&mut self, create_time: u64) {
		self.create_time = Some(create_time);
	}

	fn update_time(&self) -> Option<u64> {
		self.update_time
	}

	fn set_update_time(&mut self, update_time: u64) {
		self.update_time = Some(update_time);
	}

	fn created_by(&self) -> Option<&String> {
		self.created_by.as_ref()
	}

	fn set_created_by(&mut self, created_by: String) {
		self.created_by = Some(created_by);
	}

	fn updated_by(&self) -> Option<&String> {
		self.updated_by.as_ref()
	}

	fn set_updated_by(&mut self, updated_by: String) {
		self.updated_by = Some(updated_by);
	}
}
