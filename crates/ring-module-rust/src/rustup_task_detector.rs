use crate::rustup_task::RustupTask;
use ring_core_tasks::{DetectTask, Task};
use std::ffi::OsStr;
use std::rc::Rc;
use sysinfo::Process;
use tracing::instrument;

#[derive(Clone, Debug)]
pub struct RustupTaskDetector;

impl RustupTaskDetector {
    /// Create a new instance of RustupTaskDetector
    pub fn new() -> Self {
        RustupTaskDetector
    }

    /// Test if given process is a rustup task
    pub fn is_rustup_task(&self, process: &Process) -> bool {
        process.exe()
            .is_some_and(|exe| exe.file_stem().and_then(OsStr::to_str) == Some("rustup"))
    }

    /// Builds a `RustupTask` object from given process.
    pub fn load_rustup_task(&self, process: &Process) -> Option<Rc<RustupTask>> {
        if self.is_rustup_task(process) {
            let task = RustupTask::new(process.into());
            
            Some(Rc::new(task))
        } else {
            None
        }
    }
}

impl Default for RustupTaskDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DetectTask for RustupTaskDetector {
    #[instrument(name = "rustup-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        self.load_rustup_task(process)
            .map(|t| t as Rc<dyn Task>)
    }
}