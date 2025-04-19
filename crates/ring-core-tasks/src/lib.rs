mod task;

use std::rc::Rc;
use sysinfo::Process;
pub use task::Task;

/// Object able to detect a task
pub trait DetectTask {
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>>;
}