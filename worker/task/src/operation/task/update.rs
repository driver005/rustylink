#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use metadata::{Error, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
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
		&TaskType::UpdateTask
	}

	fn map_task(&self, _context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::UpdateTask;

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.task_update_id = Some(self.id);

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
		})
	}
}

pub type UpdateTask = Model;
