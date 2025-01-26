use actix_web::{
	guard,
	web::{self, Bytes, Data, Path},
	HttpResponse,
};

use crate::prelude::Proto;

async fn index(proto: Data<Proto>, name: Path<String>, req: Bytes) -> HttpResponse {
	println!("{}", name);
	let bytes = match proto.execute_once(req.to_vec(), &name).await {
		Ok(bytes) => bytes,
		Err(e) => {
			println!("Error: {:?}", e);
			return HttpResponse::InternalServerError().body(format!("{:?}", e));
		}
	};

	println!("{:?}", bytes);

	HttpResponse::Ok().body(bytes)
}

pub fn http_proto(cfg: &mut web::ServiceConfig, proto: Proto) {
	cfg.service(
		web::scope("/proto")
			.app_data(Data::new(proto.clone()))
			.service(web::resource("/{name}").guard(guard::Post()).to(index)),
	);
}
