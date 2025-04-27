use crate::{Module, Registry};
use ring_core_tasks::Task;
use std::rc::Rc;
use sysinfo::Process;

/// Provides calls using task detection module features
pub trait TaskRegistry: Registry {
    /// Uses all modules to detect process's task
    #[inline]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        detect_task(self.modules(), process)
    }
}

impl<T> TaskRegistry for T where T: Registry {}

/// Uses given modules to detect process's task
fn detect_task(modules: &[Box<dyn Module>], process: &Process) -> Option<Rc<dyn Task>> {
    modules.iter()
        .flat_map(|module| module.task_detectors())
        .filter_map(|detector| detector.detect_task(process))
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use ring_core_tasks::{DetectTask, Task};
    use ring_core_units::Unit;
    use std::path::Path;
    use std::rc::Rc;
    use sysinfo::System;

    mock! {
        TestUtil {}

        impl DetectTask for TestUtil {
            fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>>;
        }
    }

    mock! {
        TestTask {}

        impl Task for TestTask {
            fn exe(&self) -> &Path;
            fn kind(&self) -> &str;
            fn working_unit(&self) -> Option<Rc<dyn Unit>>;
        }
    }

    struct TestModule {
        utils: Vec<Rc<MockTestUtil>>,
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
        let mut util_a = MockTestUtil::new();
        util_a.expect_detect_task()
            .times(1)
            .returning(|_| None);

        let mut util_b = MockTestUtil::new();
        util_b.expect_detect_task()
            .times(1)
            .returning(|_| Some(Rc::new(MockTestTask::new())));

        let mut util_c = MockTestUtil::new();
        util_c.expect_detect_task()
            .times(0)
            .returning(|_| None);

        let module = TestModule {
            utils: vec![Rc::new(util_a), Rc::new(util_b), Rc::new(util_c)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        let sys = System::new_all();
        let process = sys.processes().values().next().unwrap();

        assert!(registry.detect_task(process).is_some());
    }
}