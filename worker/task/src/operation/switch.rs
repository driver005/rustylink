use crate::{Context, TaskConfig, TaskMapper, TaskModel, TaskStorage};
use chrono::Utc;
use metadata::{Error, EvaluatorType, Result, TaskStatus, TaskType};
use sea_orm::{entity::prelude::*, sea_query::index, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "switch")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	// The type of evaluator used. Supported types:
	//  -  value-param—Evaluates a specific input parameter in the Switch task.
	//  -  graaljs—Evaluates JavaScript expressions and computes the value. Allows you to use ES6-compatible JavaScript.
	pub evaluator_type: EvaluatorType,
	// The expression that is evaluated by the Switch task. The expression format depends on the evaluator type:
	//  -  For the value-param evaluator, the expression is the input parameter key.
	//  -  For the graaljs evaluator, the expression is the JavaScript expression.
	pub expression: String,
	// A map of the possible switch cases. The keys are the possible outputs of the evaluated expression, and the values are the list of tasks to be executed in each case.
	pub decision_cases: serde_json::Value,
	// The default branch. Contains the list of tasks to be executed when no matching value is found in the decision cases.
	pub default_case: Option<serde_json::Value>,
	// The list of tasks to be executed in the Switch task.
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
		TaskType::Switch
	}

	fn get_primary_key(&self) -> Uuid {
		self.id
	}

	fn add_to_queue(&self, context: &Context) -> Result<()> {
		context.get_queue().push(&Self::get_task_type().to_string(), self.id.to_string())
	}

	async fn map_task(&self, context: &Context, task: &mut TaskModel) -> Result<()> {
		task.task_type = TaskType::Switch;
		task.status = TaskStatus::InProgress;
		task.task_def_name = TaskType::TASK_TYPE_SWITCH.to_string();
		task.start_time = Utc::now();
		task.reason_for_incompletion = None;

		task.to_owned().insert(context).await?;

		Ok(())
	}

	async fn execute(&mut self, context: &mut Context) -> Result<TaskModel> {
		let mut task_model = self.new_task(&self.task_configuration)?;

		self.map_task(context, &mut task_model).await?;

		self.task_model_id = Some(task_model.task_id);

		let case_name = match self.evaluator_type {
			EvaluatorType::ValueParam => self.expression.to_owned(),
			EvaluatorType::Graaljs => {
				todo!();
			}
		};

		match self.decision_cases.get(case_name) {
			Some(value) => {
				if let Some(decision_cases) = value.as_array() {
					for decision_case in decision_cases {
						let task_config =
							serde_json::from_value::<TaskConfig>(decision_case.to_owned())?;
						let task = task_config.to_task(context).await?;

						self.task_ids.push(task.get_primary_key());

						context.get_queue().push(
							&Self::get_task_type().to_string(),
							task.get_primary_key().to_string(),
						)?;
					}
				} else {
					return Err(Error::NotFound(format!(
						"decision_case has to be a array of one or more tasks for task with id: {}",
						self.id.to_string()
					)));
				}
			}
			None => {
				if let Some(case) = &self.default_case {
					if let Some(decision_cases) = case.as_array() {
						for (index, decision_case) in decision_cases.iter().enumerate() {
							let task_config =
								serde_json::from_value::<TaskConfig>(decision_case.to_owned())?;
							let task = task_config.to_task(context).await?;

							if index == 0 {
								task.add_to_queue(context)?;
							}

							context.get_queue().push(
								&Self::get_task_type().to_string(),
								task.get_primary_key().to_string(),
							)?;
						}
					} else {
						return Err(Error::NotFound(format!(
                            "default_case has to be a array of one or more tasks for task with id: {}",
                            self.id.to_string()
                        )));
					}
				} else {
					return Err(Error::NotFound(format!(
						"default_case is missing for task with id: {}",
						self.id.to_string()
					)));
				}
			}
		};

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
			expression: owned
				.expression
				.ok_or_else(|| Error::IllegalArgument("expression is missing".to_string()))?,
			decision_cases: owned
				.decision_cases
				.ok_or_else(|| Error::IllegalArgument("decision_cases is missing".to_string()))?,
			default_case: owned.default_case,
			task_ids: vec![],
			task_model_id: None,
		})
	}
}

pub type Switch = Model;
