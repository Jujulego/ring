use crate::{Module, Registry};
use ring_core_units::Unit;
use std::path::{absolute, Path};
use std::rc::Rc;

/// Provides calls using unit detection module features
pub trait UnitRegistry: Registry {
    /// Uses all modules to detect units at given path
    #[inline]
    fn detect_units<P: AsRef<Path>>(&self, path: P) -> Vec<Rc<dyn Unit>> {
        absolute(path)
            .map(|path| detect_units(self.modules(), &path))
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
        fn detect_unit(&self, _: &Path) -> Option<Rc<dyn Unit>> {
            Some(Rc::new(TestUnit))
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
    fn it_should_use_test_util_to_detect_language() {
        let module = TestModule {
            utils: vec![Rc::new(TestUtil)]
        };

        let registry = TestRegistry {
            modules: vec![Box::new(module)],
        };

        assert_eq!(registry.detect_units("/test").len(), 1);
    }
}