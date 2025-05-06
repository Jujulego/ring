use crate::Registry;
use ring_core_content::{FileContent, PathContent};
use std::path::Path;

/// Provides calls using path detection module features
pub trait PathRegistry {
    /// Uses all modules to qualify given path
    fn qualify_path<P: AsRef<Path>>(&self, path: P) -> Option<PathContent>;
}

impl<T> PathRegistry for T where T: Registry {
    fn qualify_path<P: AsRef<Path>>(&self, path: P) -> Option<PathContent> {
        self.modules().iter()
            .flat_map(|module| module.file_qualifiers())
            .filter_map(|detector| detector.qualify_path(path.as_ref()))
            .max_by_key(|(_, qualified_path)| qualified_path.components().count())
            .map(|(content, _)| content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Module;
    use ring_core_content::QualifyPath;
    use std::rc::Rc;

    struct TestUtil;

    impl QualifyPath for TestUtil {
        fn qualify_file<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
            Some((FileContent::Tests, path))
        }
        
        fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
            Some((PathContent::Test, path))
        }
    }

    struct TestModule {
        utils: Vec<Rc<TestUtil>>,
    }

    impl Module for TestModule {
        fn file_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
            self.utils.iter()
                .map(|u| u.clone() as Rc<dyn QualifyPath>)
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

        assert_eq!(registry.qualify_path("/test"), Some(PathContent::Test));
    }
}