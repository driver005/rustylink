use async_trait::async_trait;
use futures::future::BoxFuture;
use metadata::{EventExecution, SearchResult, TaskExecLog, TaskSummary, WorkflowSummary};
use std::error::Error;

use crate::Message;

#[async_trait]
pub trait IndexDAO {
	/// Setup method in charge of initializing/populating the index.
	async fn setup(&self) -> Result<(), Box<dyn Error>>;

	/// Index a workflow.
	async fn index_workflow(&self, workflow: WorkflowSummary) -> Result<(), Box<dyn Error>>;

	/// Index a workflow asynchronously.
	async fn async_index_workflow(
		&self,
		workflow: WorkflowSummary,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Index a task.
	async fn index_task(&self, task: TaskSummary) -> Result<(), Box<dyn Error>>;

	/// Index a task asynchronously.
	async fn async_index_task(
		&self,
		task: TaskSummary,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Search for workflows.
	async fn search_workflows(
		&self,
		query: String,
		free_text: String,
		start: usize,
		count: usize,
		sort: Vec<String>,
	) -> Result<SearchResult<String>, Box<dyn Error>>;

	/// Search for workflow summaries.
	async fn search_workflow_summary(
		&self,
		query: String,
		free_text: String,
		start: usize,
		count: usize,
		sort: Vec<String>,
	) -> Result<SearchResult<WorkflowSummary>, Box<dyn Error>>;

	/// Search for tasks.
	async fn search_tasks(
		&self,
		query: String,
		free_text: String,
		start: usize,
		count: usize,
		sort: Vec<String>,
	) -> Result<SearchResult<String>, Box<dyn Error>>;

	/// Search for task summaries.
	async fn search_task_summary(
		&self,
		query: String,
		free_text: String,
		start: usize,
		count: usize,
		sort: Vec<String>,
	) -> Result<SearchResult<TaskSummary>, Box<dyn Error>>;

	/// Remove a workflow index.
	async fn remove_workflow(&self, workflow_id: String) -> Result<(), Box<dyn Error>>;

	/// Remove a workflow index asynchronously.
	async fn async_remove_workflow(
		&self,
		workflow_id: String,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Update a workflow index.
	async fn update_workflow(
		&self,
		workflow_instance_id: String,
		keys: Vec<String>,
		values: Vec<serde_json::Value>,
	) -> Result<(), Box<dyn Error>>;

	/// Update a workflow index asynchronously.
	async fn async_update_workflow(
		&self,
		workflow_instance_id: String,
		keys: Vec<String>,
		values: Vec<serde_json::Value>,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Remove a task index.
	async fn remove_task(&self, workflow_id: String, task_id: String)
		-> Result<(), Box<dyn Error>>;

	/// Remove a task index asynchronously.
	async fn async_remove_task(
		&self,
		workflow_id: String,
		task_id: String,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Update a task index.
	async fn update_task(
		&self,
		workflow_id: String,
		task_id: String,
		keys: Vec<String>,
		values: Vec<serde_json::Value>,
	) -> Result<(), Box<dyn Error>>;

	/// Update a task index asynchronously.
	async fn async_update_task(
		&self,
		workflow_id: String,
		task_id: String,
		keys: Vec<String>,
		values: Vec<serde_json::Value>,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Retrieve a specific field from the index.
	async fn get(
		&self,
		workflow_instance_id: String,
		key: String,
	) -> Result<String, Box<dyn Error>>;

	/// Add task execution logs.
	async fn add_task_execution_logs(&self, logs: Vec<TaskExecLog>) -> Result<(), Box<dyn Error>>;

	/// Add task execution logs asynchronously.
	async fn async_add_task_execution_logs(
		&self,
		logs: Vec<TaskExecLog>,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Get task execution logs.
	async fn get_task_execution_logs(
		&self,
		task_id: String,
	) -> Result<Vec<TaskExecLog>, Box<dyn Error>>;

	/// Add an event execution.
	async fn add_event_execution(
		&self,
		event_execution: EventExecution,
	) -> Result<(), Box<dyn Error>>;

	/// Get event executions.
	async fn get_event_executions(
		&self,
		event: String,
	) -> Result<Vec<EventExecution>, Box<dyn Error>>;

	/// Add an event execution asynchronously.
	async fn async_add_event_execution(
		&self,
		event_execution: EventExecution,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Add an external message into the index.
	async fn add_message(&self, queue: String, msg: Message) -> Result<(), Box<dyn Error>>;

	/// Add an external message into the index asynchronously.
	async fn async_add_message(
		&self,
		queue: String,
		message: Message,
	) -> BoxFuture<'static, Result<(), Box<dyn Error>>>;

	/// Get messages from a queue.
	async fn get_messages(&self, queue: String) -> Result<Vec<Message>, Box<dyn Error>>;

	/// Search for workflows that can be archived.
	async fn search_archivable_workflows(
		&self,
		index_name: String,
		archive_ttl_days: u64,
	) -> Result<Vec<String>, Box<dyn Error>>;

	/// Get total workflow counts matching a query.
	async fn get_workflow_count(
		&self,
		query: String,
		free_text: String,
	) -> Result<u64, Box<dyn Error>>;
}
