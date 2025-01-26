#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use http_body_util::{BodyExt, Empty};
use hyper::{body::Bytes, Request};
use hyper_util::rt::TokioIo;
use common::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{stdout, AsyncWriteExt};
use tokio::{io, net::TcpStream};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "http")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub uri: String,
	pub method: String,
	pub accept: Option<String>,
	pub content_type: Option<String>,
	pub termination_condition: Option<String>,
	pub polling_interval: Option<i64>,
	pub polling_strategy: Option<String>,
	pub headers: Option<serde_json::Value>,
	pub body: Option<serde_json::Value>,
	pub encode: Option<bool>,
	pub async_complete: Option<bool>,
	// Reference task model id
	pub task_model_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "crate::data::model::Entity",
		from = "Column::TaskModelId",
		to = "crate::data::model::Column::TaskId",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Taskmodel,
}

impl Related<crate::model::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Taskmodel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskMapper for Model {
	fn get_task_type() -> TaskType {
		TaskType::Http
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.queue.push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let task_def = self
			.get_task_def(context, self.task_configuration.task_reference_name.to_owned())
			.await?;

		task.task_type = TaskType::Http;
		task.status = TaskStatus::Scheduled;
		task.retry_count = self.task_configuration.retry_count;
		task.callback_after_seconds = self.task_configuration.start_delay;
		task.rate_limit_per_frequency = task_def.rate_limit_per_frequency;
		task.rate_limit_frequency_in_seconds = task_def.rate_limit_frequency_in_seconds;
		task.isolation_group_id = task_def.isolation_group_id.clone();
		task.execution_name_space = task_def.execution_name_space.clone();

		task.to_owned().insert(context).await?;

		Ok(())
	}

	async fn execute(&mut self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model).await?;

		self.task_model_id = Some(task_model.task_id);

		self.to_owned().save(context).await?;

		Ok(task_model)
	}
}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskStorage for Model {
	type Entity = Entity;
	type Model = Self;
	type PrimaryKey = Uuid;
	type ActiveModel = ActiveModel;

	async fn insert(self, context: &Context) -> Result<Self::Model> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn update(self, context: &Context) -> Result<Self::Model> {
		ActiveModel::update(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn save(self, context: &Context) -> Result<Self::ActiveModel> {
		ActiveModel::save(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))
	}

	async fn delete(self, context: &Context) -> Result<()> {
		ActiveModel::delete(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}

	fn find() -> Select<Self::Entity> {
		Entity::find()
	}

	async fn find_by_id(context: &Context, task_id: Self::PrimaryKey) -> Result<Self::Model> {
		let task = Entity::find_by_id(task_id)
			.one(&context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		if let Some(m) = task {
			Ok(m)
		} else {
			Err(Error::NotFound(format!(
				"Could not find {} task with id: {}",
				Self::get_task_type(),
				task_id
			)))
		}
	}
}

#[cfg(feature = "worker")]
#[async_trait::async_trait]
impl TaskExecutor for Model {
	async fn execute(&mut self, context: &mut Context) -> Result<()> {
		let url = match self.uri.parse::<hyper::Uri>() {
			Ok(url) => url,
			Err(err) => return Err(Error::IllegalArgument(format!("Invalid URI: {}", err))),
		};
		let host = url
			.host()
			.ok_or_else(|| Error::IllegalArgument("Invalid URI: missing host".to_owned()))?;
		let port = url
			.port_u16()
			.ok_or_else(|| Error::IllegalArgument("Invalid URI: missing port".to_owned()))?;
		let addr = format!("{}:{}", host, port);

		let stream = TcpStream::connect(addr.clone()).await.map_err(|err| {
			Error::IllegalArgument(format!("Failed to connect to {}: {}", addr, err))
		})?;

		let io = TokioIo::new(stream);

		let (mut sender, conn) = match hyper::client::conn::http1::handshake(io).await {
			Ok((sender, conn)) => (sender, conn),
			Err(err) => {
				return Err(Error::IllegalArgument(format!("Failed to handshake: {}", err)))
			}
		};
		tokio::task::spawn(async move {
			if let Err(err) = conn.await {
				println!("Connection failed: {:?}", err);
			}
		});

		let authority = url.authority().unwrap().clone();

		let path = url.path();
		let req = Request::builder()
			.uri(path)
			.header(hyper::header::HOST, authority.as_str())
			.body(Empty::<Bytes>::new())
			.map_err(|err| Error::IllegalArgument(format!("Failed to build request: {}", err)))?;

		let mut res = sender
			.send_request(req)
			.await
			.map_err(|err| Error::IllegalArgument(format!("Failed to send request: {}", err)))?;

		println!("Response: {}", res.status());
		println!("Headers: {:#?}\n", res.headers());

		// Stream the body, writing each chunk to stdout as we get it
		// (instead of buffering and printing at the end).
		while let Some(next) = res.frame().await {
			let frame = next
				.map_err(|err| Error::IllegalArgument(format!("Failed to read frame: {}", err)))?;
			if let Some(chunk) = frame.data_ref() {
				stdout().write_all(chunk).await.map_err(|err| {
					Error::IllegalArgument(format!("Failed to write chunk: {}", err))
				})?;
			}
		}

		println!("\n\nDone!");

		Ok(())
	}
}

#[cfg(feature = "handler")]
impl TryFrom<Arc<TaskConfig>> for Model {
	type Error = Error;

	fn try_from(value: Arc<TaskConfig>) -> Result<Self, Self::Error> {
		let task_configuration = Arc::clone(&value);
		let owned = match Arc::try_unwrap(value) {
			Ok(val) => val,
			Err(_) => return Err(Error::conflict("could not unwrap workflow task")),
		};
		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			uri: owned.get_input_parameter_required("uri")?.to_string(),
			method: owned.get_input_parameter_required("method")?.to_string(),
			accept: owned.get_input_parameter_optinal("accept").map(|v| v.to_string()),
			content_type: owned.get_input_parameter_optinal("content_type").map(|v| v.to_string()),
			termination_condition: owned
				.get_input_parameter_optinal("termination_condition")
				.map(|v| v.to_string()),
			polling_interval: owned
				.get_input_parameter_optinal("polling_interval")
				.and_then(|v| v.as_i64()),
			polling_strategy: owned
				.get_input_parameter_optinal("polling_strategy")
				.map(|v| v.to_string()),
			headers: owned
				.get_input_parameter_optinal("headers")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			body: owned
				.get_input_parameter_optinal("body")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			encode: owned.get_input_parameter_optinal("encode").and_then(|v| v.as_bool()),
			async_complete: owned
				.get_input_parameter_optinal("async_complete")
				.and_then(|v| v.as_bool()),
			task_model_id: None,
		})
	}
}

pub type Http = Model;
