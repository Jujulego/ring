use crate::Registry;
use ring_core_file::Language;
use std::path::Path;

/// Provides calls using language detection module features
pub trait LanguageRegistry {
    /// Uses all modules to detect path's language
    fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language>;
}

impl<T> LanguageRegistry for T where T: Registry {
    fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language> {
        self.modules().iter()
            .flat_map(|module| module.language_detectors())
            .filter_map(|detector| detector.detect_language(path.as_ref()))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring_core_file::{DetectLanguage, Language};
    use std::rc::Rc;
    use crate::Module;

    struct TestUtil;

    impl DetectLanguage for TestUtil {
        fn detect_language(&self, _: &Path) -> Option<Language> {
            Some(Language::new("test".to_string()))
        }
    }

    struct TestModule {
        utils: Vec<Rc<TestUtil>>,
    }

    impl Module for TestModule {
        fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
            self.utils.iter()
                .map(|u| u.clone() as Rc<dyn DetectLanguage>)
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
        let module = TestModule {
            utils: vec![Rc::new(TestUtil)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        assert!(registry.detect_language("/test").is_some());
    }
}