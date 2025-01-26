use crate::{
	BuissnessRule, DoWhile, Dynamic, DynamicFork, Event, Fork, GetSignedJwt, Http, Inline, Join,
	JsonTransform, SetVariable, Simple, StartWorkflow, SubWorkflow, Switch, TerminateTask,
	TerminateWorkflow, UpdateSecret, UpdateTask, Wait, WaitForWebhook,
};
use crate::{Context, SqlTask, TaskMapper, TaskStorage};
use common::{Error, EvaluatorType, Result, SubWorkflowParams, TaskType};
use sea_orm::{entity::prelude::*, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

use super::TaskDefinition;

/// Represents a task definition as part of a workflow
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "task_config")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	#[serde(skip_serializing)]
	pub id: Uuid,
	pub name: String,
	pub task_reference_name: String,
	pub task_type: TaskType,
	pub description: Option<String>,
	#[serde(default)]
	pub optional: bool,
	pub input_parameters: serde_json::Value,
	#[serde(default)]
	pub async_complete: bool,
	#[serde(default)]
	pub start_delay: i64,
	#[serde(default)]
	pub permissive: bool,
	pub loop_condition: Option<String>,
	pub loop_over: Option<serde_json::Value>,
	pub dynamic_task_name_param: Option<String>,
	pub dynamic_fork_tasks_param: Option<String>,
	pub dynamic_fork_tasks_input_param_name: Option<String>,
	pub fork_tasks: Option<serde_json::Value>,
	pub join_on: Option<Vec<String>>,
	pub join_status: Option<String>,
	pub sub_workflow_param: Option<SubWorkflowParams>,
	pub decision_cases: Option<serde_json::Value>,
	pub default_case: Option<serde_json::Value>,
	pub evaluator_type: Option<EvaluatorType>,
	pub expression: Option<String>,
	pub sink: Option<String>,
	pub trigger_failure_workflow: Option<bool>,
	pub script_expression: Option<String>,
	pub task_definition: Option<String>,
	// pub task_definition: Option<TaskDef>,
	pub rate_limited: Option<bool>,
	pub default_exclusive_join_task: Option<Vec<String>>,
	pub retry_count: Option<i32>,
	pub on_state_change: Option<serde_json::Value>,
	pub cache_config: Option<CacheConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct CacheConfig {
	pub key: Option<String>,
	pub ttl_in_second: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_one = "crate::data::model::Entity")]
	Taskmodel,
}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
	fn default() -> Self {
		Model {
			id: Uuid::new_v4(),
			name: String::new(),
			task_reference_name: String::new(),
			description: None,
			input_parameters: serde_json::Value::Null,
			task_type: TaskType::Simple,
			dynamic_task_name_param: None,
			script_expression: None,
			decision_cases: None,
			dynamic_fork_tasks_param: None,
			dynamic_fork_tasks_input_param_name: None,
			default_case: None,
			fork_tasks: None,
			start_delay: 0,
			sub_workflow_param: None,
			join_on: None,
			sink: None,
			optional: false,
			task_definition: None,
			rate_limited: None,
			default_exclusive_join_task: None,
			async_complete: false,
			loop_condition: None,
			loop_over: None,
			retry_count: None,
			evaluator_type: None,
			expression: None,
			on_state_change: None,
			join_status: None,
			cache_config: None,
			permissive: false,
			trigger_failure_workflow: None,
		}
	}
}

impl fmt::Display for Model {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.name, self.task_reference_name)
	}
}

impl Model {
	pub fn new(
		name: String,
		task_reference_name: String,
		task_type: TaskType,
		input_parameters: serde_json::Value,
		async_complete: bool,
		start_delay: i64,
		permissive: bool,
	) -> Self {
		Model {
			id: Uuid::new_v4(),
			name,
			task_reference_name,
			input_parameters,
			task_type,
			async_complete,
			start_delay,
			permissive,
			description: None,
			dynamic_task_name_param: None,
			script_expression: None,
			decision_cases: None,
			dynamic_fork_tasks_param: None,
			dynamic_fork_tasks_input_param_name: None,
			default_case: None,
			fork_tasks: None,
			sub_workflow_param: None,
			join_on: None,
			sink: None,
			optional: false,
			task_definition: None,
			rate_limited: None,
			default_exclusive_join_task: None,
			loop_condition: None,
			loop_over: None,
			retry_count: None,
			evaluator_type: None,
			expression: None,
			on_state_change: None,
			join_status: None,
			cache_config: None,
			trigger_failure_workflow: None,
		}
	}

	pub fn get_input_parameter_required(&self, name: &str) -> Result<&serde_json::Value> {
		self.input_parameters
			.get(name)
			.ok_or(Error::IllegalArgument(format!("Missing input parameter: {name}")))
	}

	pub fn get_input_parameter_optinal(&self, name: &str) -> Option<&serde_json::Value> {
		self.input_parameters.get(name)
	}

	pub async fn to_task(&self, context: &mut Context) -> Result<Box<dyn TaskMapper>> {
		let task_config = Arc::new(self.clone());

		self.to_owned().insert(context).await?;

		let mut task: Box<dyn TaskMapper + Send> = match self.task_type {
			TaskType::Simple => {
				let simple_task = Simple::new(Arc::clone(&task_config));
				Box::new(simple_task)
			}
			TaskType::Dynamic => {
				let dynamic_task: Dynamic = task_config.try_into()?;
				Box::new(dynamic_task)
			}
			TaskType::ForkJoin => {
				let fork_join_task: Fork = task_config.try_into()?;
				Box::new(fork_join_task)
			}
			TaskType::ForkJoinDynamic => {
				let dynamic_fork_join_task: DynamicFork = task_config.try_into()?;
				Box::new(dynamic_fork_join_task)
			}
			TaskType::Switch => {
				let switch_task: Switch = task_config.try_into()?;
				Box::new(switch_task)
			}
			TaskType::Join => {
				let join_task: Join = task_config.try_into()?;
				Box::new(join_task)
			}
			TaskType::DoWhile => {
				let do_while_task: DoWhile = task_config.try_into()?;
				Box::new(do_while_task)
			}
			TaskType::SubWorkflow => {
				let sub_workflow_task: SubWorkflow = task_config.try_into()?;
				Box::new(sub_workflow_task)
			}
			TaskType::StartWorkflow => {
				let start_workflow_task: StartWorkflow = task_config.try_into()?;
				Box::new(start_workflow_task)
			}
			TaskType::Event => {
				let event_task: Event = task_config.try_into()?;
				Box::new(event_task)
			}
			TaskType::Wait => {
				let wait_task: Wait = task_config.try_into()?;
				Box::new(wait_task)
			}
			TaskType::Human => {
				// let human_task: Human = task_config.try_into()?;
				// Box::new(human_task)
				return Err(Error::illegal_argument("not implmentet"));
			}
			TaskType::UserDefined => {
				// let user_defined_task: UserDefined = task_config.try_into()?;
				// Box::new(user_defined_task)
				return Err(Error::illegal_argument("not implmentet"));
			}
			TaskType::Http => {
				let http_task: Http = task_config.try_into()?;
				Box::new(http_task)
			}
			TaskType::Inline => {
				let inline_task: Inline = task_config.try_into()?;
				Box::new(inline_task)
			}
			TaskType::ExclusiveJoin => {
				// let exclusive_join_task: Join = task_config.try_into()?;
				// Box::new(exclusive_join_task)
				return Err(Error::illegal_argument("not implmentet"));
			}
			TaskType::TerminateTask => {
				let terminate_task: TerminateTask = task_config.try_into()?;
				Box::new(terminate_task)
			}
			TaskType::TerminateWorkflow => {
				let terminate_workflow_task: TerminateWorkflow = task_config.try_into()?;
				Box::new(terminate_workflow_task)
			}
			TaskType::KafkaPublish => {
				// let kafka_publish_task: KafkaPublish = task_config.try_into()?;
				// Box::new(kafka_publish_task)
				return Err(Error::illegal_argument("not implmentet"));
			}
			TaskType::JsonJqTransform => {
				let json_jq_transform_task: JsonTransform = task_config.try_into()?;
				Box::new(json_jq_transform_task)
			}
			TaskType::SetVariable => {
				let set_variable_task: SetVariable = SetVariable::new(Arc::clone(&task_config));
				Box::new(set_variable_task)
			}
			TaskType::UpdateTask => {
				let update_task: UpdateTask = task_config.try_into()?;
				Box::new(update_task)
			}
			TaskType::WaitForWebhook => {
				let wait_for_webhook_task: WaitForWebhook = task_config.try_into()?;
				Box::new(wait_for_webhook_task)
			}
			TaskType::BuissnessRule => {
				let buissness_rule_task: BuissnessRule = task_config.try_into()?;
				Box::new(buissness_rule_task)
			}
			TaskType::GetSignedJwt => {
				let get_signed_jwt_task: GetSignedJwt = task_config.try_into()?;
				Box::new(get_signed_jwt_task)
			}
			TaskType::UpdateSecret => {
				let update_secret_task: UpdateSecret = task_config.try_into()?;
				Box::new(update_secret_task)
			}
			TaskType::SqlTask => {
				let sql_task: SqlTask = task_config.try_into()?;
				Box::new(sql_task)
			}
		};

		task.execute(context).await?;

		Ok(task)
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
			return Err(Error::NotFound(format!(
				"Could not find task config with id: {}",
				task_id
			)));
		}
	}
}

impl From<TaskDefinition> for Model {
	fn from(task_def: TaskDefinition) -> Self {
		Model {
			id: Uuid::new_v4(),
			name: task_def.name.clone(),
			task_reference_name: format!("{}_ref", task_def.name),
			task_type: TaskType::Simple,
			description: task_def.description.clone(),
			optional: false,                           // Default value
			input_parameters: serde_json::Value::Null, // Initialize as null
			async_complete: false,                     // Default value
			start_delay: 0,                            // Default value
			permissive: task_def.enforce_schema,       // Use enforce_schema as permissive
			loop_condition: None,                      // Default value
			loop_over: None,                           // Default value
			dynamic_task_name_param: None,
			dynamic_fork_tasks_param: None,
			dynamic_fork_tasks_input_param_name: None,
			fork_tasks: None,
			join_on: None,
			join_status: None,
			sub_workflow_param: None,
			decision_cases: None,
			default_case: None,
			evaluator_type: None,
			expression: None,
			sink: None,
			trigger_failure_workflow: None,
			script_expression: None,
			task_definition: Some(task_def.name), // Link to the task definition
			rate_limited: Some(task_def.rate_limit_per_frequency.is_some()),
			default_exclusive_join_task: None,
			retry_count: Some(task_def.retry_count),
			on_state_change: None,
			cache_config: None,
		}
	}
}

pub type TaskConfig = Model;
