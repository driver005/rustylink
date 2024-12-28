#[cfg(feature = "worker")]
use crate::TaskExecutor;
#[cfg(feature = "handler")]
use crate::{Context, TaskConfig, TaskMapper, TaskModel};
use metadata::{Error, OperationType, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sql")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub integration_name: String,
	pub statement: String,
	pub operation_type: OperationType,
	pub parameters: Vec<String>,
	pub expected_output_count: Option<i64>,
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
impl Model {
	pub fn new(
		name: String,
		statement: String,
		operation_type: OperationType,
		parameters: Vec<String>,
		expected_output_count: Option<i64>,
	) -> Self {
		Self {
			task_configuration: Arc::default(),
			id: Uuid::new_v4(),
			integration_name: name,
			statement,
			operation_type,
			parameters,
			expected_output_count,
		}
	}
}

#[cfg(feature = "handler")]
#[async_trait::async_trait]
impl TaskMapper for Model {
	fn get_task_type(&self) -> &TaskType {
		&TaskType::SqlTask
	}

	fn map_task(&self, _context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::SqlTask;
		task.status = TaskStatus::Scheduled;

		Ok(())
	}

	async fn execute(&self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model)?;

		task_model.sql_task_id = Some(self.id);

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
		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			integration_name: owned.get_input_parameter_required("integration_name")?.to_string(),
			statement: owned.get_input_parameter_required("statement")?.to_string(),
			operation_type: serde_json::from_str::<OperationType>(
				&owned.get_input_parameter_required("operation_type")?.to_string(),
			)?,
			parameters: match owned.get_input_parameter_required("parameters")?.as_array() {
				Some(v) => v.iter().map(|v| v.to_string()).collect(),
				None => return Err(Error::IllegalArgument("parameters is missing".to_string())),
			},
			expected_output_count: owned
				.get_input_parameter_optinal("expected_output_count")
				.and_then(|v| v.as_i64()),
		})
	}
}

pub type SqlTask = Model;
