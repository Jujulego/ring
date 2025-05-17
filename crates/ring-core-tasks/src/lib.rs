mod process_data;
mod task;
mod task_data;

pub use crate::process_data::ProcessData;
pub use crate::task::Task;
use std::rc::Rc;
use sysinfo::Process;

/// Object able to detect a task
pub trait DetectTask {
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>>;
}