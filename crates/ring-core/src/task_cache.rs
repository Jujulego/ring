use crate::Core;
use ring_core_tasks::Task;
use std::collections::HashMap;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::debug;

/// Wrapper of core, caching detected tasks
pub struct TaskCache<'c> {
    core: &'c Core,
    cache: HashMap<Pid, Option<Rc<dyn Task>>>,
}

impl<'c> TaskCache<'c> {
    /// Creates a new task cache
    pub fn new(core: &'c Core) -> Self {
        TaskCache { core, cache: HashMap::new() }
    }

    /// Uses core & cache to detect task based on given process
    pub fn detect_task(&mut self, process: &Process) -> Option<Rc<dyn Task>> {
        if let Some(task) = self.cache.get(&process.pid()) {
            debug!(key = %process.pid(), "task cache hit");
            task.clone()
        } else {
            let task = self.core.detect_task(process);
            self.cache.insert(process.pid(), task.clone());
            debug!(key = %process.pid(), "task cached");

            task
        }
    }
}