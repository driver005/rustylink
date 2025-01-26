use common::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub struct MemQueue {
	pub deque: Mutex<HashMap<String, VecDeque<String>>>,
}

impl MemQueue {
	pub fn new() -> MemQueue {
		MemQueue {
			deque: Mutex::new(HashMap::new()),
		}
	}

	fn lock(&self) -> Result<MutexGuard<HashMap<String, VecDeque<String>>>> {
		self.deque.lock().map_err(|_| Error::Conflict("Failed to acquire lock".to_string()))
	}

	pub fn print(self) {
		println!("{:?}", self.deque);
	}

	pub fn get_queue_size(&self) -> Result<usize> {
		self.deque
			.lock()
			.map(|guard| guard.len())
			.map_err(|_| Error::Conflict("Failed to acquire lock".to_string()))
	}

	pub fn get_queue_size_by_name(&self, name: &str) -> Result<usize> {
		let guard = self.lock()?;
		Ok(guard.get(name).map_or(0, |q| q.len()))
	}

	pub fn create_queue(&self, name: &str) -> Result<()> {
		self.lock()?.insert(name.to_string(), VecDeque::new());
		Ok(())
	}

	pub fn delete_queue(&self, name: &str) -> Result<()> {
		self.deque
			.lock()
			.map(|mut guard| guard.remove(name))
			.map_err(|_| Error::Conflict("Failed to acquire lock".to_string()))?;
		Ok(())
	}

	pub fn push(&self, queue_name: &str, value: String) -> Result<()> {
		let mut guard = self.lock()?;

		match guard.get_mut(queue_name) {
			Some(q) => q.push_front(value),
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		};
		Ok(())
	}

	pub fn push_if_not_exists(&self, queue_name: &str, value: String) -> Result<bool> {
		let mut guard = self.lock()?;

		match guard.get_mut(queue_name) {
			Some(q) => {
				if !q.contains(&value) {
					q.push_front(value);
					return Ok(true);
				}
				Ok(false)
			}
			None => {
				Err(Error::Conflict(format!("could not find queue with the name: {}", queue_name)))
			}
		}
	}

	pub fn push_with_priority(&self, queue_name: &str, idx: usize, value: String) -> Result<()> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => q.insert(idx, value),
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		};

		Ok(())
	}

	pub fn pop(&self, queue_name: &str) -> Result<String> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => match q.pop_back() {
				Some(v) => Ok(v),
				None => {
					Err(Error::NotFound(format!("could remove item from queue: {}", queue_name)))
				}
			},
			None => {
				Err(Error::Conflict(format!("could not find queue with the name: {}", queue_name)))
			}
		}
	}

	pub fn batch_poll(&self, queue_name: &str, count: usize) -> Result<Vec<String>> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => {
				let mut result = Vec::new();
				for _ in 0..count {
					if let Some(v) = q.pop_back() {
						result.push(v);
					}
				}
				Ok(result)
			}
			None => {
				Err(Error::Conflict(format!("could not find queue with the name: {}", queue_name)))
			}
		}
	}

	pub fn contains(&self, queue_name: &str, value: String) -> Result<bool> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => Ok(q.contains(&value)),
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		}
	}

	pub fn find(&self, queue_name: &str, value: String) -> Result<usize> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => match q.binary_search(&value) {
				Ok(idx) => Ok(idx),
				Err(_) => {
					return Err(Error::NotFound(format!(
						"could not find value: {} in queue: {}",
						value, queue_name
					)))
				}
			},
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		}
	}

	pub fn postpone(&self, queue_name: &str, value: String) -> Result<()> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => match q.binary_search(&value) {
				Ok(idx) => q.swap(idx, q.len()),
				Err(_) => {
					return Err(Error::NotFound(format!(
						"could not find value: {} in queue: {}",
						value, queue_name
					)))
				}
			},
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		};

		Ok(())
	}

	pub fn postpone_in_all_queues(&self, value: String) -> Result<()> {
		for (_, q) in self.lock()?.iter_mut() {
			if let Some(idx) = q.iter().position(|x| x == &value) {
				q.swap(idx, q.len());
				return Ok(());
			}
		}

		return Err(Error::NotFound(format!("could not find value: {}", value)));
	}

	pub fn get_size(&self, queue_name: &str) -> Result<usize> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => Ok(q.len()),
			None => {
				Err(Error::Conflict(format!("could not find queue with the name: {}", queue_name)))
			}
		}
	}

	pub fn remove(&self, queue_name: &str, idx: usize) -> Result<()> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => q.remove(idx),
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		};

		Ok(())
	}

	pub fn remove_by_value(&self, queue_name: &str, value: String) -> Result<()> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => {
				if let Some(idx) = q.iter().position(|x| x == &value) {
					q.remove(idx);
				}
			}
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)));
			}
		};

		Ok(())
	}

	pub fn remove_by_value_from_all_queues(&self, value: String) -> Result<()> {
		for (_, q) in self.lock()?.iter_mut() {
			if let Some(idx) = q.iter().position(|x| x == &value) {
				q.remove(idx);
				return Ok(());
			}
		}

		return Err(Error::NotFound(format!("could not find value: {}", value)));
	}

	pub fn flush(&self, queue_name: &str) -> Result<()> {
		match self.lock()?.get_mut(queue_name) {
			Some(q) => q.clear(),
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		};

		Ok(())
	}
}
