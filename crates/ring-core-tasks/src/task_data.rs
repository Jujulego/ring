use serde::{Deserialize, Serialize};

/// Descriptive data on a given task
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TaskData {
    pub kind: String,
}