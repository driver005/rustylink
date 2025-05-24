use std::sync::Arc;

use crate::Value;

/// A validator for scalar
pub type ScalarValidatorFn = Arc<dyn Fn(&Value) -> bool + Send + Sync>;
