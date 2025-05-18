use ring_core_modules::ProcessTaskRegistry;
use ring_core_tasks::{ProcessTask};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::debug;

/// Wrapper of core, caching detected tasks
pub struct TaskCache<'r, R: ProcessTaskRegistry> {
    registry: &'r R,
    cache: RefCell<HashMap<Pid, Option<Rc<dyn ProcessTask>>>>,
}

impl<'r, R: ProcessTaskRegistry> TaskCache<'r, R> {
    /// Creates a new task cache
    pub fn new(registry: &'r R) -> Self {
        TaskCache { registry, cache: RefCell::new(HashMap::new()) }
    }
}

impl<R: ProcessTaskRegistry> ProcessTaskRegistry for TaskCache<'_, R> {
    /// Uses core & cache to detect task based on given process
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn ProcessTask>> {
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