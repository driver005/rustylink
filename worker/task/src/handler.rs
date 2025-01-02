use crate::{
	log, poll, BuissnessRule, Context, DoWhile, Dynamic, DynamicFork, Event, Fork, GetSignedJwt,
	Http, Human, Inline, Join, JsonTransform, PollData, SetVariable, Simple, SqlTask,
	StartWorkflow, SubWorkflow, Switch, TaskExecutionLog, TaskModel, TaskStorage, TerminateTask,
	TerminateWorkflow, UpdateSecret, UpdateTask, Wait, WaitForWebhook,
};
use actix_web::{web, HttpResponse, Scope};
use metadata::TaskType;
use sea_orm::{ColumnTrait, Order, QueryFilter, QueryOrder};
use uuid::Uuid;

struct Handler {}

impl Handler {
	pub fn router() -> Scope {
		//TODO: update task by ref name nomal and sync
		web::scope("/task")
			.route("", web::patch().to(Self::update_task))
			.route(":task_type/:task_id", web::get().to(Self::get_task))
			.route("model/:task_id", web::get().to(Self::get_task_model))
			.route("/:task_id/log", web::get().to(Self::get_task_log))
			.route("/:task_id/log", web::post().to(Self::add_task_log))
			.route("/poll/:task_type", web::get().to(Self::poll_task))
			.route("/poll/batch/:task_type", web::get().to(Self::batch_poll_tasks))
			.route("/queue/all", web::get().to(Self::queue_details))
			.route("/queue/polldata", web::get().to(Self::queue_poll_data))
			.route("/queue/polldata/all", web::get().to(Self::queues_poll_data))
			.route("/queue/sizes", web::get().to(Self::queues_size))
			.route("/search", web::get().to(Self::search_task))
	}

	pub async fn get_task(
		data: web::Data<Context>,
		task_type: web::Path<TaskType>,
		task_id: web::Path<Uuid>,
	) -> HttpResponse {
		match task_type.into_inner() {
			TaskType::Simple => match Simple::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::Dynamic => match Dynamic::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::ForkJoin => match Fork::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::ForkJoinDynamic => {
				match DynamicFork::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::Switch => match Switch::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::Join => match Join::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::DoWhile => match DoWhile::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::SubWorkflow => {
				match SubWorkflow::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::StartWorkflow => {
				match StartWorkflow::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::Event => match Event::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::Wait => match Wait::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::Human => match Human::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::UserDefined => {
				// match UserDefined::find_by_id(&data, task_id.into_inner()).await {
				// 	Ok(task) => HttpResponse::Ok().json(task),
				// 	Err(_) => HttpResponse::NotFound().finish(),
				// }
				todo!()
			}
			TaskType::Http => match Http::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::Inline => match Inline::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::ExclusiveJoin => match Join::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
			TaskType::TerminateTask => {
				match TerminateTask::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::TerminateWorkflow => {
				match TerminateWorkflow::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::KafkaPublish => {
				// match KafkaPublish::find_by_id(&data, task_id.into_inner()).await {
				// 	Ok(task) => HttpResponse::Ok().json(task),
				// 	Err(_) => HttpResponse::NotFound().finish(),
				// }
				todo!()
			}
			TaskType::JsonJqTransform => {
				match JsonTransform::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::SetVariable => {
				match SetVariable::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::UpdateTask => {
				match UpdateTask::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::WaitForWebhook => {
				match WaitForWebhook::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::BuissnessRule => {
				match BuissnessRule::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::GetSignedJwt => {
				match GetSignedJwt::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::UpdateSecret => {
				match UpdateSecret::find_by_id(&data, task_id.into_inner()).await {
					Ok(task) => HttpResponse::Ok().json(task),
					Err(_) => HttpResponse::NotFound().finish(),
				}
			}
			TaskType::SqlTask => match SqlTask::find_by_id(&data, task_id.into_inner()).await {
				Ok(task) => HttpResponse::Ok().json(task),
				Err(_) => HttpResponse::NotFound().finish(),
			},
		}
	}

	pub async fn get_task_model(
		data: web::Data<Context>,
		task_id: web::Path<Uuid>,
	) -> HttpResponse {
		match TaskModel::find_by_id(&data, task_id.into_inner()).await {
			Ok(task) => HttpResponse::Ok().json(task),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn update_task(data: web::Data<Context>, task: web::Json<TaskModel>) -> HttpResponse {
		match task.into_inner().update(&data).await {
			Ok(_) => HttpResponse::Ok().finish(),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn search_task(data: web::Data<Context>, task_id: web::Path<Uuid>) -> HttpResponse {
		HttpResponse::NotFound().finish()
	}

	pub async fn poll_task(data: web::Data<Context>, task_type: web::Path<String>) -> HttpResponse {
		match data.get_queue().pop(&task_type.into_inner()) {
			Ok(task) => HttpResponse::Ok().json(task),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn batch_poll_tasks(
		data: web::Data<Context>,
		task_type: web::Path<String>,
		count: web::Query<usize>,
	) -> HttpResponse {
		match data.get_queue().batch_poll(&task_type.into_inner(), count.into_inner()) {
			Ok(tasks) => HttpResponse::Ok().json(tasks),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn get_task_log(data: web::Data<Context>, task_id: web::Path<Uuid>) -> HttpResponse {
		match TaskExecutionLog::find()
			.filter(log::Column::TaskId.eq(task_id.into_inner()))
			.order_by(log::Column::CreatedTime, Order::Asc)
			.all(&data.db)
			.await
		{
			Ok(logs) => HttpResponse::Ok().json(logs),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn add_task_log(
		data: web::Data<Context>,
		task_id: web::Path<Uuid>,
		log_data: web::Json<String>,
	) -> HttpResponse {
		match TaskExecutionLog::new(task_id.into_inner(), log_data.into_inner()).insert(&data).await
		{
			Ok(task) => HttpResponse::Ok().json(task),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn queue_details(
		data: web::Data<Context>,
		task_type: web::Path<String>,
	) -> HttpResponse {
		HttpResponse::NotFound().finish()
	}

	pub async fn queue_poll_data(
		data: web::Data<Context>,
		task_type: web::Query<String>,
	) -> HttpResponse {
		match PollData::find()
			.filter(poll::Column::QueueName.eq(task_type.into_inner()))
			.order_by(poll::Column::ModifiedOn, Order::Asc)
			.all(&data.db)
			.await
		{
			Ok(task) => HttpResponse::Ok().json(task),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn queues_poll_data(data: web::Data<Context>) -> HttpResponse {
		match PollData::find().order_by(poll::Column::ModifiedOn, Order::Asc).all(&data.db).await {
			Ok(task) => HttpResponse::Ok().json(task),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}

	pub async fn queues_size(
		data: web::Data<Context>,
		task_type: web::Query<String>,
	) -> HttpResponse {
		match data.get_queue().get_size(&task_type.into_inner()) {
			Ok(tasks) => HttpResponse::Ok().json(tasks),
			Err(_) => HttpResponse::NotFound().finish(),
		}
	}
}
