use crate::Registry;
use ring_core_tasks::ProcessTask;
use std::rc::Rc;
use sysinfo::Process;

/// Provides calls using task detection module features
pub trait ProcessTaskRegistry {
    /// Uses all modules to detect process's task
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn ProcessTask>>;
}

impl<T> ProcessTaskRegistry for T where T: Registry {
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn ProcessTask>> {
        self.modules().iter()
            .flat_map(|module| module.task_detectors())
            .filter_map(|detector| detector.detect_task(process))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use crate::Module;
    use ring_core_tasks::{DetectProcessTask, Task};
    use std::rc::Rc;
    use sysinfo::{Pid, System};

    struct TestTask;

    impl Task for TestTask {
        fn id(&self) -> &str {
            "id"
        }

        fn kind(&self) -> &str {
            "kind"
        }
    }

    impl ProcessTask for TestTask {
        fn executable(&self) -> &Path {
            "/test/exe".as_ref()
        }

        fn working_directory(&self) -> &Path {
            "/test".as_ref()
        }
    }

    struct TestUtil;

    impl DetectProcessTask for TestUtil {
        fn detect_task(&self, _: &Process) -> Option<Rc<dyn ProcessTask>> {
            Some(Rc::new(TestTask))
        }
    }

    struct TestModule {
        utils: Vec<Rc<TestUtil>>,
    }

    impl Module for TestModule {
        fn task_detectors(&self) -> Vec<Rc<dyn DetectProcessTask>> {
            self.utils.iter()
                .map(|u| u.clone() as Rc<dyn DetectProcessTask>)
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
        let process = sys.process(Pid::from_u32(std::process::id())).unwrap();

        assert!(registry.detect_task(process).is_some());
    }
}