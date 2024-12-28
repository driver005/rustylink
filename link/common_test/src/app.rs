// use crate::DbPool;

// pub struct AppState {
// 	pub db: PgPool,
// 	pub dynamic_routes: Mutex<HashMap<String, MethodRouter>>,
// }

// impl AppState {
// 	pub fn new(db: PgPool) -> Self {
// 		AppState {
// 			db,
// 			dynamic_routes: Mutex::new(HashMap::new()),
// 		}
// 	}

// 	pub fn add_route(&self, path: &str, method_router: MethodRouter) {
// 		let mut routes = self.dynamic_routes.lock().unwrap();
// 		routes.insert(path.to_string(), method_router);
// 	}
// }
