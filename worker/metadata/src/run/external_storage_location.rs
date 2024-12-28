use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalStorageLocation {
	pub uri: String,
	pub path: String,
}

impl ExternalStorageLocation {
	pub fn new(uri: String, path: String) -> Self {
		ExternalStorageLocation {
			uri,
			path,
		}
	}
}
