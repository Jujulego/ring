mod process_data;
mod process_task;
mod process_task_data;
mod task;

pub use crate::process_data::ProcessData;
pub use crate::process_task::ProcessTask;
pub use crate::task::Task;
use std::rc::Rc;
use sysinfo::Process;

/// Object able to detect a task
pub trait DetectProcessTask {
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn ProcessTask>>;
}