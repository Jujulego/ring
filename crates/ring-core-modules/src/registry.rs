use std::path::{absolute, Path};
use std::rc::Rc;
use ring_core_file::Language;
use crate::Module;

/// Object managing a set of modules.
/// Provides call using features declared by those modules.
pub trait Registry {
    /// Returns list of registered modules
    fn modules(&self) -> &[Rc<dyn Module>];

    /// Uses all modules to detect path's language
    #[inline]
    fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language> {
        detect_language(self.modules(), &absolute(path).ok()?)
    }
}

/// Uses given modules to detect path's language
fn detect_language(modules: &[Rc<dyn Module>], path: &Path) -> Option<Language> {
    modules.iter()
        .flat_map(|module| module.language_detectors())
        .filter_map(|detector| detector.detect_language(path))
        .next()
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use ring_core_file::{DetectLanguage, Language};
    use super::*;

    mock! {
        TestUtil {}

        impl DetectLanguage for TestUtil {
            fn detect_language(&self, path: &Path) -> Option<Language>;
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
    }

    struct TestRegistry {
        modules: Vec<Rc<dyn Module>>,
    }
    
    impl Registry for TestRegistry {
        fn modules(&self) -> &[Rc<dyn Module>] {
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
            modules: vec![Rc::new(module)],
        };
        
        assert_eq!(registry.detect_language("/test"), Some(language));
    }
}