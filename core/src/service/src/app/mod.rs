use crate::{
	Config, ServerConfig, ServerType,
	server::{grpc_server, http_server},
};
use apy::QueryRoot;
use dynamic::{
	SchemaError,
	prelude::{Proto, Schema},
};
use futures::future::try_join_all;
use sea_orm::{Database, DatabaseConnection};

pub struct App {
	pub config: Config,
	pub database: DatabaseConnection,
	pub http: Option<Box<dyn QueryRoot<Schema>>>,
	pub grpc: Option<Box<dyn QueryRoot<Proto>>>,
}

impl App {
	pub async fn from_config(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let config = Config::load_config(file_path)?;

		let database = Database::connect(config.database.get_uri()).await?;

		Ok(App {
			config,
			database,
			grpc: None,
			http: None,
		})
	}

	pub fn add_grpc_service(mut self, service: Box<dyn QueryRoot<Proto>>) -> Self {
		self.grpc = Some(service);

		self
	}
	pub fn add_http_service(mut self, service: Box<dyn QueryRoot<Schema>>) -> Self {
		self.http = Some(service);

		self
	}

	pub fn get_http(&self, database: &DatabaseConnection) -> Result<Schema, SchemaError> {
		if let Some(service) = &self.http {
			return service.root(database);
		}

		Err(SchemaError("Please add the HTTP service to your App instance".to_string()))
	}

	pub fn get_grpc(
		&mut self,
		database: &DatabaseConnection,
		config: &ServerConfig,
	) -> Result<Proto, SchemaError> {
		if let Some(service) = self.grpc.as_mut() {
			service.config_schema(config.limit.unwrap_or(0), config.complexity.unwrap_or(0));
			return service.root(database);
		}

		Err(SchemaError("Please add the GRPC service to your App instance".to_string()))
	}

	pub async fn build(&mut self) -> Result<(), SchemaError> {
		let mut services = Vec::new();

		let database = self.database.clone();
		let servers = self.config.servers.clone();

		for service in &servers {
			match service.name {
				ServerType::Grpc => {
					let query_root = self.get_grpc(&database, service)?;
					services.push(tokio::spawn(grpc_server(query_root, service.clone())));
				}
				ServerType::Http => {
					let query_root = self.get_http(&database)?;
					services.push(tokio::spawn(http_server(query_root, service.clone())));
				}
			};
		}

		let _results = try_join_all(services).await.map_err(|err| SchemaError(err.to_string()))?;

		// for r in results {
		// 	println!("{}", r?); // Unwrap each JoinHandle result
		// }

		println!("Both servers shut down cleanly.");

		Ok(())
	}
}

pub enum Servers {
	Http(Box<dyn QueryRoot<Schema>>),
	Grpc(Box<dyn QueryRoot<Proto>>),
}

impl Servers {
	pub fn get_http(&self, database: &DatabaseConnection) -> Result<Schema, SchemaError> {
		if let Self::Http(service) = self {
			return service.root(database);
		}

		Err(SchemaError("Please add the HTTP service to your App instance".to_string()))
	}

	pub fn get_grpc(&self, database: &DatabaseConnection) -> Result<Proto, SchemaError> {
		if let Self::Grpc(service) = self {
			return service.root(database);
		}

		Err(SchemaError("Please add the GRPC service to your App instance".to_string()))
	}
}
