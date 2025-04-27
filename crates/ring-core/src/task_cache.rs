use ring_core_modules::Registry;
use ring_core_tasks::Task;
use std::collections::HashMap;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::debug;

/// Wrapper of core, caching detected tasks
pub struct TaskCache<'r, R: Registry> {
    registry: &'r R,
    cache: HashMap<Pid, Option<Rc<dyn Task>>>,
}

impl<'r, R: Registry> TaskCache<'r, R> {
    /// Creates a new task cache
    pub fn new(registry: &'r R) -> Self {
        TaskCache { registry, cache: HashMap::new() }
    }

    /// Uses core & cache to detect task based on given process
    pub fn detect_task(&mut self, process: &Process) -> Option<Rc<dyn Task>> {
        if let Some(task) = self.cache.get(&process.pid()) {
            debug!(key = %process.pid(), "task cache hit");
            task.clone()
        } else {
            let task = self.registry.detect_task(process);
            self.cache.insert(process.pid(), task.clone());
            debug!(key = %process.pid(), "task cached");

            task
        }
    }
}