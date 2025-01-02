use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use chrono::Utc;
use metadata::{Error, EvaluatorType, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "do_while")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	//The type of evaluator used. Supported types:
	//  -  value-param—Evaluates a specific input parameter in the Do While task.
	//  -  graaljs—Evaluates JavaScript expressions and computes the value. Allows you to use ES6-compatible JavaScript.
	pub evaluator_type: EvaluatorType,
	// The condition that is evaluated by the Do While task after every iteration. The expression format depends on the evaluator type:
	//  -  For the value-param evaluator, the expression is the input parameter key.
	//  -  For the javascript and graaljs evaluators, the expression is the JavaScript expression.
	pub loop_condition: String,
	// The list of tasks to be executed as long as the condition is true.
	pub loop_over: serde_json::Value,
	// The list of tasks to be executed in the Do While task.
	pub task_ids: Vec<Uuid>,
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
		TaskType::DoWhile
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.get_queue().push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		let task_def = self
			.get_task_def(context, self.task_configuration.task_reference_name.to_owned())
			.await?;

		task.task_type = TaskType::DoWhile;
		task.status = TaskStatus::InProgress;
		task.start_time = Utc::now();
		task.retry_count = self.task_configuration.retry_count;
		task.rate_limit_per_frequency = task_def.rate_limit_per_frequency;
		task.rate_limit_frequency_in_seconds = task_def.rate_limit_frequency_in_seconds;

		task.to_owned().insert(context).await?;

		Ok(())
	}

	async fn execute(&mut self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model).await?;

		self.task_model_id = Some(task_model.task_id);

		if let Some(loob_task_config) = self.loop_over.as_array() {
			for decision_case in loob_task_config.iter() {
				let task_config = serde_json::from_value::<TaskConfig>(decision_case.to_owned())?;
				let task = task_config.to_task(context).await?;

				self.task_ids.push(task.get_primary_key());

				context
					.get_queue()
					.push(&Self::get_task_type().to_string(), task.get_primary_key().to_string())?;
			}
		} else {
			return Err(Error::NotFound(
				"decision cases has needs to be a array of one or more tasks".to_string(),
			));
		}

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

		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			evaluator_type: owned
				.evaluator_type
				.ok_or_else(|| Error::IllegalArgument("evaluator_type is missing".to_string()))?,
			loop_condition: owned
				.loop_condition
				.ok_or_else(|| Error::IllegalArgument("loop_condition is missing".to_string()))?,
			loop_over: owned
				.loop_over
				.ok_or_else(|| Error::IllegalArgument("loop_over is missing".to_string()))?,
			task_ids: vec![],
			task_model_id: None,
		})
	}
}

pub type DoWhile = Model;
