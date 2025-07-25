use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	pub database: Database,
	pub servers: Vec<ServerConfig>,
}

impl Config {
	pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
		// Read the TOML file content
		let config_str = fs::read_to_string(file_path)?;

		// Parse and deserialize the TOML content
		let config: Config = toml::de::from_str(&config_str)?;

		Ok(config)
	}
}

// DATABASE
#[derive(Debug, Clone, Deserialize)]
pub struct Database {
	pub host: String,
	pub port: u16,
	pub user: String,
	pub password: String,
	pub name: String,
}

impl Database {
	pub fn get_uri(&self) -> String {
		format!(
			"postgresql://{}:{}@{}:{}/{}",
			self.user, self.password, self.host, self.port, self.name
		)
	}
}

// SERVER
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
	pub name: ServerType,
	pub host: String,
	pub port: u16,
	#[serde(default)]
	pub limit: Option<u16>, // Optional for Grpc servers
	#[serde(default)]
	pub complexity: Option<u16>, // Optional for Grpc servers
}

impl ServerConfig {
	pub fn get_uri(&self) -> String {
		format!("{}:{}", self.host, self.port)
	}
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum ServerType {
	Grpc,
	Http,
}
