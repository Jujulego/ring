use crate::Registry;
use ring_core_tasks::Task;
use std::rc::Rc;
use sysinfo::Process;

/// Provides calls using task detection module features
pub trait TaskRegistry {
    /// Uses all modules to detect process's task
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>>;
}

impl<T> TaskRegistry for T where T: Registry {
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        self.modules().iter()
            .flat_map(|module| module.task_detectors())
            .filter_map(|detector| detector.detect_task(process))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Module;
    use ring_core_tasks::{DetectTask, Task};
    use ring_core_units::Unit;
    use std::path::Path;
    use std::rc::Rc;
    use sysinfo::System;

    struct TestTask;

    impl Task for TestTask {
        fn id(&self) -> &str {
            "test"
        }

        fn executable(&self) -> &Path {
            unimplemented!()
        }

        fn kind(&self) -> &str {
            "test"
        }

        fn working_unit(&self) -> Option<Rc<dyn Unit>> {
            None
        }
    }

    struct TestUtil;

    impl DetectTask for TestUtil {
        fn detect_task(&self, _: &Process) -> Option<Rc<dyn Task>> {
            Some(Rc::new(TestTask))
        }
    }

    struct TestModule {
        utils: Vec<Rc<TestUtil>>,
    }

    impl Module for TestModule {
        fn task_detectors(&self) -> Vec<Rc<dyn DetectTask>> {
            self.utils.iter()
                .map(|u| u.clone() as Rc<dyn DetectTask>)
                .collect()
        }
    }

    struct TestRegistry {
        modules: Vec<Box<dyn Module>>,
    }

    impl Registry for TestRegistry {
        fn modules(&self) -> &[Box<dyn Module>] {
            &self.modules
        }
    }

    #[test]
    fn it_should_use_test_util_to_detect_task() {
        let module = TestModule {
            utils: vec![Rc::new(TestUtil)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        let sys = System::new_all();
        let process = sys.processes().values().next().unwrap();

        assert!(registry.detect_task(process).is_some());
    }
}