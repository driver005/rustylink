use super::{
	BuissnessRule, DoWhile, Dynamic, DynamicFork, Event, Fork, GetSignedJwt, Http, Inline, Join,
	JsonTransform, SetVariable, Simple, StartWorkflow, SubWorkflow, Switch, TerminateTask,
	TerminateWorkflow, UpdateSecret, UpdateTask, Wait, WaitForWebhook,
};
use crate::{Context, SqlTask, TaskMapper};
use metadata::{Error, Result, SubWorkflowParams, TaskType};
use sea_orm::FromJsonQueryResult;
use sea_orm::{entity::prelude::*, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

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
	pub loop_over: Option<Vec<Uuid>>,
	pub dynamic_task_name_param: Option<String>,
	pub dynamic_fork_tasks_param: Option<String>,
	pub dynamic_fork_tasks_input_param_name: Option<String>,
	pub fork_tasks: Option<Vec<Uuid>>,
	pub join_on: Option<Vec<String>>,
	pub join_status: Option<String>,
	pub sub_workflow_param: Option<SubWorkflowParams>,
	pub decision_cases: Option<serde_json::Value>,
	pub default_case: Option<Vec<Uuid>>,
	pub evaluator_type: Option<String>,
	pub expression: Option<String>,
	pub sink: Option<String>,
	pub trigger_failure_workflow: Option<bool>,
	pub script_expression: Option<String>,
	pub task_definition_id: Option<Uuid>,
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
pub enum Relation {}

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
			task_definition_id: None,
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
			task_definition_id: None,
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

	async fn save(self, context: &mut Context) -> Result<()> {
		ActiveModel::insert(self.into_active_model(), &context.db)
			.await
			.map_err(|err| Error::DbError(err))?;

		Ok(())
	}

	pub async fn to_task(&self, context: &mut Context) -> Result<Box<dyn TaskMapper>> {
		let task_config = Arc::new(self.clone());

		self.to_owned().save(context).await?;

		match self.task_type {
			TaskType::Simple => {
				let simple_task = Simple::new(Arc::clone(&task_config));
				Ok(Box::new(simple_task))
			}
			TaskType::Dynamic => {
				let dynamic_task: Dynamic = task_config.try_into()?;
				Ok(Box::new(dynamic_task))
			}
			TaskType::ForkJoin => {
				let fork_join_task: Fork = task_config.try_into()?;
				Ok(Box::new(fork_join_task))
			}
			TaskType::ForkJoinDynamic => {
				let dynamic_fork_join_task: DynamicFork = task_config.try_into()?;
				Ok(Box::new(dynamic_fork_join_task))
			}
			TaskType::Switch => {
				let switch_task: Switch = task_config.try_into()?;
				Ok(Box::new(switch_task))
			}
			TaskType::Join => {
				let join_task: Join = task_config.try_into()?;
				Ok(Box::new(join_task))
			}
			TaskType::DoWhile => {
				let do_while_task: DoWhile = task_config.try_into()?;
				Ok(Box::new(do_while_task))
			}
			TaskType::SubWorkflow => {
				let sub_workflow_task: SubWorkflow = task_config.try_into()?;
				Ok(Box::new(sub_workflow_task))
			}
			TaskType::StartWorkflow => {
				let start_workflow_task: StartWorkflow = task_config.try_into()?;
				Ok(Box::new(start_workflow_task))
			}
			TaskType::Event => {
				let event_task: Event = task_config.try_into()?;
				Ok(Box::new(event_task))
			}
			TaskType::Wait => {
				let wait_task: Wait = task_config.try_into()?;
				Ok(Box::new(wait_task))
			}
			TaskType::Human => {
				// let human_task: Human = task_config.try_into()?;
				// Ok(Box::new(human_task))
				Err(Error::illegal_argument("not implmentet"))
			}
			TaskType::UserDefined => {
				// let user_defined_task: UserDefined = task_config.try_into()?;
				// Ok(Box::new(user_defined_task))
				Err(Error::illegal_argument("not implmentet"))
			}
			TaskType::Http => {
				let http_task: Http = task_config.try_into()?;
				Ok(Box::new(http_task))
			}
			TaskType::Inline => {
				let inline_task: Inline = task_config.try_into()?;
				Ok(Box::new(inline_task))
			}
			TaskType::ExclusiveJoin => {
				// let exclusive_join_task: Join = task_config.try_into()?;
				// Ok(Box::new(exclusive_join_task))
				Err(Error::illegal_argument("not implmentet"))
			}
			TaskType::TerminateTask => {
				let terminate_task: TerminateTask = task_config.try_into()?;
				Ok(Box::new(terminate_task))
			}
			TaskType::TerminateWorkflow => {
				let terminate_workflow_task: TerminateWorkflow = task_config.try_into()?;
				Ok(Box::new(terminate_workflow_task))
			}
			TaskType::KafkaPublish => {
				// let kafka_publish_task: KafkaPublish = task_config.try_into()?;
				// Ok(Box::new(kafka_publish_task))
				Err(Error::illegal_argument("not implmentet"))
			}
			TaskType::JsonJqTransform => {
				let json_jq_transform_task: JsonTransform = task_config.try_into()?;
				Ok(Box::new(json_jq_transform_task))
			}
			TaskType::SetVariable => {
				let set_variable_task: SetVariable = SetVariable::new(Arc::clone(&task_config));
				Ok(Box::new(set_variable_task))
			}
			TaskType::UpdateTask => {
				let update_task: UpdateTask = task_config.try_into()?;
				Ok(Box::new(update_task))
			}
			TaskType::WaitForWebhook => {
				let wait_for_webhook_task: WaitForWebhook = task_config.try_into()?;
				Ok(Box::new(wait_for_webhook_task))
			}
			TaskType::BuissnessRule => {
				let buissness_rule_task: BuissnessRule = task_config.try_into()?;
				Ok(Box::new(buissness_rule_task))
			}
			TaskType::GetSignedJwt => {
				let get_signed_jwt_task: GetSignedJwt = task_config.try_into()?;
				Ok(Box::new(get_signed_jwt_task))
			}
			TaskType::UpdateSecret => {
				let update_secret_task: UpdateSecret = task_config.try_into()?;
				Ok(Box::new(update_secret_task))
			}
			TaskType::SqlTask => {
				let sql_task: SqlTask = task_config.try_into()?;
				Ok(Box::new(sql_task))
			}
		}
	}
}

pub type TaskConfig = Model;