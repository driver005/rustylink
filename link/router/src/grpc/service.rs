use super::registry::DynamicService;
use crate::grpc::registry::ServiceRegistry;
use tonic::{Request, Response, Status};

pub struct MyDynamicService {
	registry: ServiceRegistry,
}

#[tonic::async_trait]
impl DynamicService for MyDynamicService {
	async fn handle_request(
		&self,
		name: &str,
		request: Request<tonic::body::BoxBody>,
	) -> Result<Response<tonic::body::BoxBody>, Status> {
		if let Some(handler) = self.registry.get_service(name) {
			handler(request)
		} else {
			Err(Status::not_found("Service not found"))
		}
	}
}
