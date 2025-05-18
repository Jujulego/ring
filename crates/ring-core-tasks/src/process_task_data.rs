use crate::{ProcessTask, Task};
use ring_core_units::{Unit, UnitData};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Descriptive data of a given task
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProcessTaskData {
    pub id: String,
    pub kind: String,
    pub args: Vec<String>,
    pub executable: PathBuf,
    pub script: Option<PathBuf>,
    pub working_directory: PathBuf,
    pub working_unit: Option<UnitData>,
}

impl Task for ProcessTaskData {
    #[inline]
    fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    fn kind(&self) -> &str {
        &self.kind
    }

    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.as_ref()
            .map(|unit| Rc::new(unit.clone()) as Rc<dyn Unit>)
    }
}

impl ProcessTask for ProcessTaskData {
    #[inline]
    fn executable(&self) -> &Path {
        &self.executable
    }

    #[inline]
    fn working_directory(&self) -> &Path {
        &self.working_directory
    }

    #[inline]
    fn args(&self) -> &[String] {
        &self.args
    }

    #[inline]
    fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    #[inline]
    fn inspect(&self) -> ProcessTaskData {
        self.clone()
    }
}