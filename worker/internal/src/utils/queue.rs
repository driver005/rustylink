use metadata::Task;
use model::TaskModel;
use std::string::String;

pub struct QueueUtils;

impl QueueUtils {
	const DOMAIN_SEPARATOR: &'static str = ":";
	const ISOLATION_SEPARATOR: &'static str = "-";
	const EXECUTION_NAME_SPACE_SEPARATOR: &'static str = "@";

	pub fn get_queue_name_by_task_model(task_model: &TaskModel) -> String {
		Self::get_queue_name(
			&task_model.task_type.to_string(),
			task_model.domain.as_deref(),
			task_model.isolation_group_id.as_deref(),
			task_model.execution_name_space.as_deref(),
		)
	}

	pub fn get_queue_name_by_task(task: &Task) -> String {
		Self::get_queue_name(
			&task.task_type.to_string(),
			task.domain.as_deref(),
			task.isolation_group_id.as_deref(),
			task.execution_name_space.as_deref(),
		)
	}

	pub fn get_queue_name(
		task_type: &str,
		domain: Option<&str>,
		isolation_group_id: Option<&str>,
		execution_name_space: Option<&str>,
	) -> String {
		let mut queue_name = match domain {
			Some(d) => format!("{}{}{}", d, Self::DOMAIN_SEPARATOR, task_type),
			None => task_type.to_string(),
		};

		if let Some(namespace) = execution_name_space {
			queue_name.push_str(&format!("{}{}", Self::EXECUTION_NAME_SPACE_SEPARATOR, namespace));
		}

		if let Some(group_id) = isolation_group_id {
			queue_name.push_str(&format!("{}{}", Self::ISOLATION_SEPARATOR, group_id));
		}

		queue_name
	}

	pub fn get_queue_name_without_domain(queue_name: &str) -> String {
		queue_name.splitn(2, Self::DOMAIN_SEPARATOR).nth(1).unwrap_or("").to_string()
	}

	pub fn get_execution_name_space(queue_name: &str) -> String {
		if queue_name.contains(Self::ISOLATION_SEPARATOR)
			&& queue_name.contains(Self::EXECUTION_NAME_SPACE_SEPARATOR)
		{
			queue_name
				.split(Self::EXECUTION_NAME_SPACE_SEPARATOR)
				.nth(1)
				.unwrap_or("")
				.split(Self::ISOLATION_SEPARATOR)
				.next()
				.unwrap_or("")
				.to_string()
		} else if queue_name.contains(Self::EXECUTION_NAME_SPACE_SEPARATOR) {
			queue_name.split(Self::EXECUTION_NAME_SPACE_SEPARATOR).nth(1).unwrap_or("").to_string()
		} else {
			String::new()
		}
	}

	pub fn is_isolated_queue(queue: &str) -> bool {
		!Self::get_isolation_group(queue).is_empty()
	}

	fn get_isolation_group(queue: &str) -> String {
		queue.split(Self::ISOLATION_SEPARATOR).nth(1).unwrap_or("").to_string()
	}

	pub fn get_task_type(queue: &str) -> String {
		if queue.is_empty() {
			return String::new();
		}

		let start_index = queue.find(Self::DOMAIN_SEPARATOR).map(|i| i + 1).unwrap_or(0);
		let end_index = queue
			.find(Self::EXECUTION_NAME_SPACE_SEPARATOR)
			.or_else(|| queue.rfind(Self::ISOLATION_SEPARATOR))
			.unwrap_or(queue.len());

		queue[start_index..end_index].to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_queue_name() {
		assert_eq!(
			QueueUtils::get_queue_name("task", Some("domain"), Some("iso"), Some("exec")),
			"domain:task@exec-iso"
		);
		assert_eq!(QueueUtils::get_queue_name("task", None, None, None), "task");
	}

	#[test]
	fn test_get_queue_name_without_domain() {
		assert_eq!(QueueUtils::get_queue_name_without_domain("domain:task"), "task");
	}

	#[test]
	fn test_get_execution_name_space() {
		assert_eq!(QueueUtils::get_execution_name_space("task@exec-iso"), "exec");
		assert_eq!(QueueUtils::get_execution_name_space("task@exec"), "exec");
		assert_eq!(QueueUtils::get_execution_name_space("task"), "");
	}

	#[test]
	fn test_is_isolated_queue() {
		assert!(QueueUtils::is_isolated_queue("task-iso"));
		assert!(!QueueUtils::is_isolated_queue("task"));
	}

	#[test]
	fn test_get_task_type() {
		assert_eq!(QueueUtils::get_task_type("domain:task@exec-iso"), "task");
		assert_eq!(QueueUtils::get_task_type("task"), "task");
		assert_eq!(QueueUtils::get_task_type(""), "");
	}
}
