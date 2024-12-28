use metadata::Error;
use sea_orm::{entity::prelude::*, IntoActiveModel };
use std::sync::Arc;

use crate::TaskConfig;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "get_workflow")]
pub struct Model {
	#[sea_orm(ignore)]
	task_configuration: Arc<TaskConfig>,
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub include_tasks: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "crate::model::Entity")]
	Taskmodel,
}

impl Related<crate::model::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Taskmodel.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Arc<TaskConfig>> for Model {
	type Error = Error;

	fn try_from(value: Arc<TaskConfig>) -> Result<Self, Self::Error> {
		let task_configuration = Arc::clone(&value);
		let owned = match Arc::try_unwrap(value) {
			Ok(val) => val,
			Err(_) => return Err(Error::conflict("could not unwrap workflow task")),
		};
		let id = owned.get_input_parameter_required("id")?.to_string();

		if id.is_empty() {
			return Err(Error::IllegalArgument("id is missing".to_string()));
		}
		Ok(Self {
			task_configuration,
			id: Uuid::new_v4(),
			include_tasks: owned
				.get_input_parameter_required("include_tasks")?
				.as_bool()
				.ok_or_else(|| Error::IllegalArgument("evaluator_type is missing".to_string()))?,
		})
	}
}

pub type GetWorkflow = Model;
