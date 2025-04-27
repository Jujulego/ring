use crate::{Module, Registry};
use ring_core_units::Unit;
use std::path::{absolute, Path};
use std::rc::Rc;

/// Provides calls using unit detection module features
pub trait UnitRegistry: Registry {
    /// Uses all modules to detect units at given path
    #[inline]
    fn detect_units_at<P: AsRef<Path>>(&self, path: P) -> Vec<Rc<dyn Unit>> {
        absolute(path)
            .map(|path| detect_units(self.modules(), &path))
            .unwrap_or_default()
    }

    /// Uses all modules to detect units containing given path
    #[inline]
    fn detect_units_containing<P: AsRef<Path>>(&self, path: P) -> Vec<Rc<dyn Unit>> {
        absolute(path).iter()
            .flat_map(|path| path.ancestors())
            .map(|path| detect_units(self.modules(), path))
            .find(|units| !units.is_empty())
            .unwrap_or_default()
    }
}

impl<T> UnitRegistry for T where T: Registry {}

/// Uses given modules to detect units at given path
fn detect_units(modules: &[Box<dyn Module>], path: &Path) -> Vec<Rc<dyn Unit>> {
    modules.iter()
        .flat_map(|module| module.unit_detectors())
        .filter_map(|detector| detector.detect_unit(path))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use super::*;
    use ring_core_units::DetectUnit;
    use std::rc::Rc;

    struct TestUnit;

    impl Unit for TestUnit {
        fn kind(&self) -> &str {
            "test"
        }

        fn root(&self) -> &Path {
            unimplemented!()
        }
    }

    struct TestUtil;

    impl DetectUnit for TestUtil {
        fn detect_unit(&self, path: &Path) -> Option<Rc<dyn Unit>> {
            if path.file_name().and_then(OsStr::to_str) == Some("test") {
                Some(Rc::new(TestUnit))
            } else {
                None
            }
        }
    }

    struct TestModule {
        utils: Vec<Rc<TestUtil>>,
    }

    impl Module for TestModule {
        fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
            self.utils.iter()
                .map(|u| u.clone() as Rc<dyn DetectUnit>)
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
    fn it_should_use_test_util_to_detect_language_at_path() {
        let module = TestModule {
            utils: vec![Rc::new(TestUtil)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        assert_eq!(registry.detect_units_at("/test").len(), 1);
        assert_eq!(registry.detect_units_at("/toto").len(), 0);
    }

    #[test]
    fn it_should_use_test_util_to_detect_language_containing_path() {
        let module = TestModule {
            utils: vec![Rc::new(TestUtil)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        assert_eq!(registry.detect_units_containing("/test/toto").len(), 1);
        assert_eq!(registry.detect_units_containing("/toto/foo").len(), 0);
    }
}