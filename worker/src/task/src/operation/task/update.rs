#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use common::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "update_task")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub task_status: TaskStatus,
	pub workflow_id: Option<String>,
	pub task_ref_name: Option<String>,
	pub task_id: Option<String>,
	pub merge_output: Option<bool>,
	pub task_output: Option<serde_json::Value>,
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
		TaskType::UpdateTask
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.queue.push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::UpdateTask;

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

#[cfg(feature = "handler")]
impl TryFrom<Arc<TaskConfig>> for Model {
	type Error = Error;

	fn try_from(value: Arc<TaskConfig>) -> Result<Self, Self::Error> {
		let task_configuration = Arc::clone(&value);
		let owned = match Arc::try_unwrap(value) {
			Ok(val) => val,
			Err(_) => return Err(Error::conflict("could not unwrap workflow task")),
		};

		let workflow_id = owned.get_input_parameter_optinal("workflow_id").map(|v| v.to_string());
		let task_ref_name =
			owned.get_input_parameter_optinal("task_ref_name").map(|v| v.to_string());
		let task_id = owned.get_input_parameter_optinal("task_id").map(|v| v.to_string());

		if task_id.is_none() {
			if workflow_id.is_none() && task_ref_name.is_none() {
				return Err(Error::IllegalArgument(
					"workflow_id and task_ref_name is missing".to_string(),
				));
			} else if workflow_id.is_none() {
				return Err(Error::IllegalArgument("workflow_id is missing".to_string()));
			} else if task_ref_name.is_none() {
				return Err(Error::IllegalArgument("task_ref_name is missing".to_string()));
			} else {
				return Err(Error::IllegalArgument(
					"workflow_id, task_ref_name or task_id is missing".to_string(),
				));
			}
		}

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			task_status: match owned.get_input_parameter_required("task_status")?.as_str() {
				Some(val) => serde_json::from_str::<TaskStatus>(val)?,
				None => return Err(Error::IllegalArgument("task_status is missing".to_string())),
			},
			workflow_id: owned.get_input_parameter_optinal("workflow_id").map(|v| v.to_string()),
			task_ref_name: owned
				.get_input_parameter_optinal("task_ref_name")
				.map(|v| v.to_string()),
			task_id: owned.get_input_parameter_optinal("task_id").map(|v| v.to_string()),
			merge_output: owned
				.get_input_parameter_optinal("merge_output")
				.and_then(|v| v.as_bool()),
			task_output: owned
				.get_input_parameter_optinal("task_output")
				.and_then(|v| v.as_object())
				.map(|v| v.clone().into_iter().collect()),
			task_model_id: None,
		})
	}
}

pub type UpdateTask = Model;
