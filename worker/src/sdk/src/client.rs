use http_body_util::{Empty, Full};
use hyper::body::Bytes;
use hyper::client::HttpConnector;
use hyper::Error;
use hyper::{body::Body, Method, Request, Uri};
use hyper_util::rt::TokioIo;
use task::{PollData, TaskExecutionLog, TaskModel};
use tokio::net::TcpStream;

pub struct TaskClient {
	client: Client<HttpConnector>,
	base_url: String,
}

impl TaskClient {
	pub fn new(base_url: String) -> Self {
		TaskClient {
			client: Client::new(),
			base_url,
		}
	}

	async fn request(address: String) -> Result<(), Error> {
		// Open a TCP connection to the remote host
		let stream = TcpStream::connect(address).await?;

		// Use an adapter to access something implementing `tokio::io` traits as if they implement
		// `hyper::rt` IO traits.
		let io = TokioIo::new(stream);

		// Create the Hyper client
		let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

		// Spawn a task to poll the connection, driving the HTTP state
		tokio::task::spawn(async move {
			if let Err(err) = conn.await {
				println!("Connection failed: {:?}", err);
			}
		});

		Ok(())
	}

	pub async fn get_task(
		&self,
		task_type: &str,
		task_id: String,
	) -> Result<TaskModel, Box<dyn std::error::Error>> {
		let url = format!("{}/task/{}/{}", self.base_url, task_type, task_id);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let task: TaskModel = serde_json::from_slice(&body)?;
		Ok(task)
	}

	pub async fn update_task(
		&self,
		task: TaskModel,
	) -> Result<TaskModel, Box<dyn std::error::Error>> {
		let url = format!("{}/task", self.base_url);
		let uri: Uri = url.parse()?;
		let body = serde_json::to_string(&task)?;
		let req = Request::builder()
			.method(Method::PATCH)
			.uri(uri)
			.header("Content-Type", "application/json")
			.body(Full::new(Bytes::from(body)))?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let updated_task: TaskModel = serde_json::from_slice(&body)?;
		Ok(updated_task)
	}

	pub async fn poll_task(&self, task_type: &str) -> Result<String, Box<dyn std::error::Error>> {
		let url = format!("{}/task/poll/{}", self.base_url, task_type);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let task: String = serde_json::from_slice(&body)?;
		Ok(task)
	}

	pub async fn batch_poll_tasks(
		&self,
		task_type: &str,
		count: usize,
	) -> Result<Vec<String>, Box<dyn std::error::Error>> {
		let url = format!("{}/task/poll/batch/{}?count={}", self.base_url, task_type, count);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let tasks: Vec<String> = serde_json::from_slice(&body)?;
		Ok(tasks)
	}

	pub async fn get_task_log(
		&self,
		task_id: String,
	) -> Result<Vec<TaskExecutionLog>, Box<dyn std::error::Error>> {
		let url = format!("{}/task/{}/log", self.base_url, task_id);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let logs: Vec<TaskExecutionLog> = serde_json::from_slice(&body)?;
		Ok(logs)
	}

	pub async fn add_task_log(
		&self,
		task_id: String,
		log_data: String,
	) -> Result<TaskExecutionLog, Box<dyn std::error::Error>> {
		let url = format!("{}/task/{}/log", self.base_url, task_id);
		let uri: Uri = url.parse()?;
		let req = Request::builder()
			.method(Method::POST)
			.uri(uri)
			.header("Content-Type", "application/json")
			.body(Full::new(Bytes::from(log_data)))?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let log: TaskExecutionLog = serde_json::from_slice(&body)?;
		Ok(log)
	}

	pub async fn queue_poll_data(
		&self,
		task_type: &str,
	) -> Result<Vec<PollData>, Box<dyn std::error::Error>> {
		let url = format!("{}/task/queue/polldata?task_type={}", self.base_url, task_type);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let poll_data: Vec<PollData> = serde_json::from_slice(&body)?;
		Ok(poll_data)
	}

	pub async fn queues_poll_data(&self) -> Result<Vec<PollData>, Box<dyn std::error::Error>> {
		let url = format!("{}/task/queue/polldata/all", self.base_url);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let poll_data: Vec<PollData> = serde_json::from_slice(&body)?;
		Ok(poll_data)
	}

	pub async fn queues_size(&self, task_type: &str) -> Result<usize, Box<dyn std::error::Error>> {
		let url = format!("{}/task/queue/sizes?task_type={}", self.base_url, task_type);
		let uri: Uri = url.parse()?;
		let req = Request::builder().method(Method::GET).uri(uri).body(Empty::<Bytes>::new())?;
		let res = self.client.request(req).await?;
		let body = res.collect().await?.aggregate();
		let size: usize = serde_json::from_slice(&body)?;
		Ok(size)
	}
}
