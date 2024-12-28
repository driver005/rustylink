use prost::Message;
use std::collections::HashMap;
use tonic::async_trait;
use tonic::{Request, Response, Status};

type ServiceFn =
	fn(Request<tonic::body::BoxBody>) -> Result<Response<tonic::body::BoxBody>, Status>;

struct ServiceRegistry {
	services: HashMap<String, ServiceFn>,
}

impl ServiceRegistry {
	fn new() -> Self {
		ServiceRegistry {
			services: HashMap::new(),
		}
	}

	fn register_service(&mut self, name: &str, handler: ServiceFn) {
		self.services.insert(name.to_string(), handler);
	}

	fn get_service(&self, name: &str) -> Option<&ServiceFn> {
		self.services.get(name)
	}
}

#[async_trait]
pub trait DynamicService {
	async fn handle_request(
		&self,
		name: &str,
		request: Request<tonic::body::BoxBody>,
	) -> Result<Response<tonic::body::BoxBody>, Status>;
}
