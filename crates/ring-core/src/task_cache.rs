use std::cell::RefCell;
use ring_core_modules::TaskRegistry;
use ring_core_tasks::Task;
use std::collections::HashMap;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::debug;

/// Wrapper of core, caching detected tasks
pub struct TaskCache<'r, R: TaskRegistry> {
    registry: &'r R,
    cache: RefCell<HashMap<Pid, Option<Rc<dyn Task>>>>,
}

impl<'r, R: TaskRegistry> TaskCache<'r, R> {
    /// Creates a new task cache
    pub fn new(registry: &'r R) -> Self {
        TaskCache { registry, cache: RefCell::new(HashMap::new()) }
    }
}

impl<R: TaskRegistry> TaskRegistry for TaskCache<'_, R> {
    /// Uses core & cache to detect task based on given process
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        if let Some(task) = self.cache.borrow().get(&process.pid()) {
            debug!(key = %process.pid(), "task cache hit");
            task.clone()
        } else {
            let task = self.registry.detect_task(process);
            self.cache.borrow_mut()
                .insert(process.pid(), task.clone());

            debug!(key = %process.pid(), "task cached");

            task
        }
    }
}