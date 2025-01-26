#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use chrono::{DateTime, Utc};
use common::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "wait")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	// The datetime and timezone in one of the following formats:
	//  -  yyyy-MM-dd HH:mm z
	//  -  yyyy-MM-dd HH:mm
	//  -  yyyy-MM-dd
	pub until: Option<DateTime<Utc>>,
	// The wait duration in the format x days y hours z minutes aa seconds. The accepted units in this field are:
	//  -  days, or d for days
	//  -  hours, hrs, or h for hours
	//  -  minutes, mins, or m for minutes
	//  -  seconds, secs, or s for seconds
	pub duration: Option<String>,
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
		TaskType::Wait
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

		if let Some(dur) = &self.duration {
			let v = dur.parse::<i64>()?;

			task.callback_after_seconds = v;
			task.wait_timeout = Some(v as i64);
		}

		if let Some(utl) = self.until {
			let now = Utc::now();
			let dif = utl - now;

			task.callback_after_seconds = dif.num_seconds() as i64;
			task.wait_timeout = Some(now.timestamp());
		}

		task.task_type = TaskType::Wait;
		task.status = TaskStatus::InProgress;
		task.start_time = Utc::now();
		task.isolation_group_id = task_def.isolation_group_id.clone();

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
		todo!()
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

		let until = match owned.get_input_parameter_optinal("until").and_then(|v| v.as_str()) {
			Some(val) => Some(DateTime::from_str(val)?),
			None => None,
		};
		let duration = owned.get_input_parameter_optinal("duration").map(|v| v.to_string());

		if until.is_none() && duration.is_none() {
			return Err(Error::IllegalArgument(
				"Either 'until' or 'duration' must be specified".to_string(),
			));
		}

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			until,
			duration,
			task_model_id: None,
		})
	}
}

pub type Wait = Model;
