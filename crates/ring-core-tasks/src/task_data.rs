use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Descriptive data on a given task
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TaskData {
    pub id: String,
    pub kind: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub exe: PathBuf,
    pub script: Option<PathBuf>,
}