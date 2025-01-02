use std::collections::{HashMap, VecDeque};

use metadata::{Error, Result};

#[derive(Debug, Clone)]
pub struct MemQueue {
	pub deque: HashMap<String, VecDeque<String>>,
}

impl MemQueue {
	pub fn new() -> MemQueue {
		MemQueue {
			deque: HashMap::new(),
		}
	}

	pub fn print(self) {
		println!("{:?}", self.deque);
	}

	pub fn get_queue_size(&self) -> usize {
		self.deque.len()
	}

	pub fn get_queue_size_by_name(&self, name: &str) -> usize {
		match self.deque.get(name) {
			Some(q) => q.len(),
			None => 0,
		}
	}

	pub fn create_queue(&mut self, name: &str) {
		self.deque.insert(name.to_string(), VecDeque::new());
	}

	pub fn delete_queue(&mut self, name: &str) {
		self.deque.remove(name);
	}

	pub fn push(&mut self, queue_name: &str, value: String) -> Result<()> {
		match self.deque.get_mut(queue_name) {
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

	pub fn push_if_not_exists(&mut self, queue_name: &str, value: String) -> Result<bool> {
		match self.deque.get_mut(queue_name) {
			Some(q) => {
				if !q.contains(&value) {
					q.push_front(value);
					return Ok(true);
				}
			}
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		};

		Ok(false)
	}

	pub fn push_with_priority(
		&mut self,
		queue_name: &str,
		idx: usize,
		value: String,
	) -> Result<()> {
		match self.deque.get_mut(queue_name) {
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

	pub fn pop(&mut self, queue_name: &str) -> Result<String> {
		match self.deque.get_mut(queue_name) {
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

	pub fn batch_poll(&mut self, queue_name: &str, count: usize) -> Result<Vec<String>> {
		match self.deque.get_mut(queue_name) {
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

	pub fn contains(&mut self, queue_name: &str, value: String) -> Result<bool> {
		match self.deque.get_mut(queue_name) {
			Some(q) => Ok(q.contains(&value)),
			None => {
				return Err(Error::Conflict(format!(
					"could not find queue with the name: {}",
					queue_name
				)))
			}
		}
	}

	pub fn find(&mut self, queue_name: &str, value: String) -> Result<usize> {
		match self.deque.get_mut(queue_name) {
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

	pub fn postpone(&mut self, queue_name: &str, value: String) -> Result<()> {
		match self.deque.get_mut(queue_name) {
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

	pub fn postpone_in_all_queues(&mut self, value: String) -> Result<()> {
		for (_, q) in self.deque.iter_mut() {
			if let Some(idx) = q.iter().position(|x| x == &value) {
				q.swap(idx, q.len());
				return Ok(());
			}
		}

		return Err(Error::NotFound(format!("could not find value: {}", value)));
	}

	pub fn get_size(&mut self, queue_name: &str) -> Result<usize> {
		match self.deque.get_mut(queue_name) {
			Some(q) => Ok(q.len()),
			None => {
				Err(Error::Conflict(format!("could not find queue with the name: {}", queue_name)))
			}
		}
	}

	pub fn remove(&mut self, queue_name: &str, idx: usize) -> Result<()> {
		match self.deque.get_mut(queue_name) {
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

	pub fn remove_by_value(&mut self, queue_name: &str, value: String) -> Result<()> {
		match self.deque.get_mut(queue_name) {
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

	pub fn remove_by_value_from_all_queues(&mut self, value: String) -> Result<()> {
		for (_, q) in self.deque.iter_mut() {
			if let Some(idx) = q.iter().position(|x| x == &value) {
				q.remove(idx);
				return Ok(());
			}
		}

		return Err(Error::NotFound(format!("could not find value: {}", value)));
	}

	pub fn flush(&mut self, queue_name: &str) -> Result<()> {
		match self.deque.get_mut(queue_name) {
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
