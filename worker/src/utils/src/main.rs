use common::TaskType;
use queue::MemQueue;
use sea_orm::{ActiveEnum, ConnectionTrait, Database, DbConn, EntityTrait, Schema};
use sea_schema::postgres::discovery::SchemaDiscovery;
use task::{
	buissness::Entity as BuissnessRule,
	config::Entity as TaskConfigEntity,
	definition::Entity as TaskDefinitionEntity,
	do_while::Entity as DoWhile,
	dynamic::Entity as Dynamic,
	event::Entity as Event,
	fork::{dynamic::Entity as DynamicFork, fork::Entity as Fork},
	http::Entity as Http,
	human::Entity as Human,
	inline::Entity as Inline,
	join::Entity as Join,
	jwt::Entity as GetSignedJwt,
	log::Entity as TaskExecutionLog,
	model::Entity as TaskModel,
	poll::Entity as PollData,
	secret::Entity as UpdateSecret,
	simple::Entity as Simple,
	sql::Entity as SqlTask,
	switch::Entity as Switch,
	task::{terminate::Entity as TerminateTask, update::Entity as UpdateTask},
	transform::json::Entity as JsonTransform,
	variable::Entity as SetVariable,
	wait::Entity as Wait,
	webhook::wait::Entity as WaitForWebhook,
	workflow::start::Entity as StartWorkflow,
	workflow::sub::Entity as SubWorkflow,
	workflow::{get::Entity as GetWorkflow, terminate::Entity as TerminateWorkflow},
	Context, TaskConfig, TaskDefinition, TaskStorage,
};

async fn create_table<E>(db: &DbConn, entity: E)
where
	E: EntityTrait,
{
	let builder = db.get_database_backend();
	let stmt = builder.build(Schema::new(builder).create_table_from_entity(entity).if_not_exists());

	match db.execute(stmt).await {
		Ok(_) => println!("Migrated: {}", entity.table_name()),
		Err(e) => println!("Error: {}", e),
	}
}

async fn create_enum<E>(db: &DbConn)
where
	E: ActiveEnum,
{
	let builder = db.get_database_backend();
	let stmt = builder.build(&Schema::new(builder).create_enum_from_active_enum::<E>());
	match db.execute(stmt).await {
		Ok(_) => println!("Migrated: {}", E::name().to_string()),
		Err(e) => println!("Error: {}", e),
	}
}

pub async fn create_tables(db: &DbConn) {
	create_enum::<common::EvaluatorType>(db).await;
	create_enum::<common::OperationType>(db).await;
	create_enum::<common::TimeoutPolicy>(db).await;
	create_enum::<common::RetryLogic>(db).await;
	create_enum::<common::TaskType>(db).await;
	create_enum::<common::TaskStatus>(db).await;
	create_enum::<common::TaskTerminationStatus>(db).await;
	create_enum::<common::ForkType>(db).await;
	create_enum::<common::WorkflowStatus>(db).await;
	create_enum::<common::IdempotencyStrategy>(db).await;
	create_table(db, TaskDefinitionEntity).await;
	create_table(db, TaskConfigEntity).await;
	create_table(db, TaskModel).await;
	create_table(db, TaskExecutionLog).await;
	create_table(db, PollData).await;
	create_table(db, BuissnessRule).await;
	create_table(db, DoWhile).await;
	create_table(db, Dynamic).await;
	create_table(db, DynamicFork).await;
	create_table(db, Event).await;
	create_table(db, Fork).await;
	create_table(db, GetSignedJwt).await;
	create_table(db, GetWorkflow).await;
	create_table(db, Http).await;
	create_table(db, Human).await;
	create_table(db, Inline).await;
	create_table(db, Join).await;
	create_table(db, JsonTransform).await;
	create_table(db, SetVariable).await;
	create_table(db, Simple).await;
	create_table(db, SqlTask).await;
	create_table(db, StartWorkflow).await;
	create_table(db, SubWorkflow).await;
	create_table(db, Switch).await;
	create_table(db, TerminateTask).await;
	create_table(db, TerminateWorkflow).await;
	create_table(db, UpdateSecret).await;
	create_table(db, UpdateTask).await;
	create_table(db, Wait).await;
	create_table(db, WaitForWebhook).await;
}

#[actix_web::main]
async fn main() {
	let database = Database::connect("postgresql://postgres:postgres@localhost:5432/medusa-test")
		.await
		.expect("Fail to initialize database connection");

	create_tables(&database).await;

	// let schema_discovery =
	// 	SchemaDiscovery::new(database.get_postgres_connection_pool().clone(), "public");

	// let schema = schema_discovery.discover().await;

	// println!("{:#?}", schema);

	let mut context = Context {
		db: database.clone(),
		queue: Box::new(MemQueue::new()),
	};

	// context.queue.create_queue("SQL_TASK");
	context.queue.create_queue("SIMPLE");

	let task_def = TaskDefinition::new("test_def".to_string());

	task_def.clone().insert(&mut context).await;

	let work_task = TaskConfig::new(
		"test".to_string(),
		task_def.name.clone(),
		TaskType::Simple,
		serde_json::Value::Null,
		true,
		0,
		true,
	);

	match work_task.to_task(&mut context).await {
		Ok(t) => {
			t.add_to_queue(&context).unwrap();
			context.queue.print();
		}
		Err(e) => {
			println!("Error: {}", e);
		}
	};
}
