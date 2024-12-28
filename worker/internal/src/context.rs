use crate::{
	ConcurrentExecutionLimitDao, EventHandlerDao, ExecutionDao, IndexDAO, MetadataDao, PollDataDao,
	QueueDao, RateLimitingDao,
};

pub struct Context {
	pub metadata: Box<dyn MetadataDao>,
	pub queue: Box<dyn QueueDao>,
	// pub concurent_execution_limit: Box<dyn ConcurrentExecutionLimitDao>,
	// pub event_handler: Box<dyn EventHandlerDao>,
	// pub execution: Box<dyn ExecutionDao>,
	// pub index: Box<dyn IndexDAO>,
	// pub poll_data: Box<dyn PollDataDao>,
	// pub rate_limiting: Box<dyn RateLimitingDao>,
}
