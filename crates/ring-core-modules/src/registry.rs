use crate::Module;
use ring_core_file::Language;
use ring_core_tasks::Task;
use std::path::{absolute, Path};
use std::rc::Rc;
use sysinfo::Process;

/// Object managing a set of modules.
/// Provides calls using features declared by those modules.
pub trait Registry {
    /// Returns list of registered modules
    fn modules(&self) -> &[Box<dyn Module>];

    /// Uses all modules to detect path's language
    #[inline]
    fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language> {
        detect_language(self.modules(), &absolute(path).ok()?)
    }

    /// Uses all modules to detect process's task
    #[inline]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        detect_task(self.modules(), process)
    }
}

/// Uses given modules to detect path's language
fn detect_language(modules: &[Box<dyn Module>], path: &Path) -> Option<Language> {
    modules.iter()
        .flat_map(|module| module.language_detectors())
        .filter_map(|detector| detector.detect_language(path))
        .next()
}

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
    use ring_core_file::{DetectLanguage, Language};
    use ring_core_tasks::{DetectTask, Task};
    use ring_core_units::Unit;
    use std::rc::Rc;
    use sysinfo::System;

    mock! {
        TestUtil {}

        impl DetectLanguage for TestUtil {
            fn detect_language(&self, path: &Path) -> Option<Language>;
        }

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
        fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
            self.utils.iter()
                .map(|u| u.clone() as Rc<dyn DetectLanguage>)
                .collect()
        }

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
    fn it_should_use_test_util_to_detect_language() {
        let language = Language::new("test".to_string());

        let mut util_a = MockTestUtil::new();
        util_a.expect_detect_language()
            .times(1)
            .return_const(None);

        let mut util_b = MockTestUtil::new();
        util_b.expect_detect_language()
            .times(1)
            .return_const(Some(language.clone()));

        let mut util_c = MockTestUtil::new();
        util_c.expect_detect_language()
            .times(0)
            .return_const(None);

        let module = TestModule {
            utils: vec![Rc::new(util_a), Rc::new(util_b), Rc::new(util_c)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        assert_eq!(registry.detect_language("/test"), Some(language));
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