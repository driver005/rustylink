#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use chrono::{DateTime, Utc};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "wait")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub until: Option<DateTime<Utc>>,
	pub duration: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "crate::model::Entity")]
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
	fn get_task_type(&self) -> &TaskType {
		&TaskType::Wait
	}

	fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let task_def = self.get_task_def(context, &self.task_configuration.name)?;

		if let Some(dur) = &self.duration {
			let v = dur.parse::< i64>()?;

			task.callback_after_seconds = v;
			task.wait_timeout = Some(v as i64);
		}

		if let Some(utl) = self.until {
			let now = Utc::now();
			let dif = utl - now;

			task.callback_after_seconds = dif.num_seconds() as  i64;
			task.wait_timeout = Some(now.timestamp());
		}

		task.task_type = TaskType::Wait;
		task.status = TaskStatus::InProgress;
		task.start_time = Utc::now();
		task.isolation_group_id = task_def.isolation_group_id.clone();

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.wait_id = Some(self.id);

		self.to_owned().save(context).await?;

		context.queue.push(&self.get_task_type().to_string(), self.id.to_string())?;

		Ok(task_model)
	}

	async fn save(self, context: &mut Context) -> Result<()> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}
}

#[cfg(feature = "worker")]
impl TaskExecutor for Model {
	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
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
		})
	}
}

pub type Wait = Model;
