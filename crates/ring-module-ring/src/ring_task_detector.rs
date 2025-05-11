use std::ffi::OsStr;
use std::rc::Rc;
use sysinfo::Process;
use tracing::instrument;
use ring_core_modules::{RegistryRef, UnitRegistry};
use ring_core_tasks::{DetectTask, Task};
use crate::RingTask;

#[derive(Clone)]
pub struct RingTaskDetector {
    registry: Rc<RegistryRef>,
}

impl RingTaskDetector {
    pub fn new(registry: Rc<RegistryRef>) -> RingTaskDetector {
        RingTaskDetector {
            registry
        }
    }
}

impl DetectTask for RingTaskDetector {
    #[instrument(name = "ring-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        let exe = process.exe()?;
        
        if matches!(exe.file_stem().and_then(OsStr::to_str), Some("ring") | Some("ring-cli")) {
            let task = Rc::new(RingTask::new(
                exe.to_path_buf(),
                process.pid().as_u32() == std::process::id(),
                process.cwd()
                    .and_then(|cwd| self.registry.detect_units_containing(cwd).first().cloned())
            ));

            Some(task)
        } else {
            None
        }
    }
}
