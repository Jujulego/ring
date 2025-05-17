use ring_core_units::UnitData;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Descriptive data of a given task
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TaskData {
    pub id: String,
    pub kind: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub exe: PathBuf,
    pub script: Option<PathBuf>,
    pub working_unit: Option<UnitData>,
}