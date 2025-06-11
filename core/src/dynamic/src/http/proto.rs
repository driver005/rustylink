use tokio::signal;
use tonic::transport::Server;
use tonic_health::server::health_reporter;
use tonic_reflection::server::Builder;
use tonic_web::GrpcWebLayer;

use crate::proto::{Proto, WrapperMutation, WrapperQuery};

pub async fn grpc_server(proto: Proto) {
	let reflection = Builder::configure()
		.register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
		.register_file_descriptor_set(proto.registry())
		.build_v1alpha()
		.unwrap();

	let (mut _health_reporter, health_service) = health_reporter();

	println!("Visit gRPC at grpc://127.0.0.1:50051");
	Server::builder()
		.accept_http1(true)
		.layer(GrpcWebLayer::new())
		.add_service(reflection)
		.add_service(WrapperQuery::new(proto.get_data()))
		.add_service(WrapperMutation::new(proto.get_data()))
		.add_service(health_service)
		.serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), async {
			signal::ctrl_c().await.expect("failed to listen for ctrl_c");
		})
		.await
		.unwrap();
}
