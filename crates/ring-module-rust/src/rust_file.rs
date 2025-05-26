use crate::rust_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct RustFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl RustFileDetector {
    /// Creates a new instance of RustFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator,
        }
    }

    /// Checks if given path is a rust file
    #[inline]
    pub fn is_rust_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_rust_file(path.as_ref())
    }
    
    fn _is_rust_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path) && path.extension().and_then(OsStr::to_str) == Some("rs")
    }
}

impl DetectLanguage for RustFileDetector {
    #[instrument(name = "rust-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_rust_file(path) {
            Some(rust_language())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use ring_core_fs::{FileWrapper, PathAdaptator};

    mock! {
        TestAdaptator {}

        impl PathAdaptator for TestAdaptator {
            fn is_dir(&self, path: &Path) -> bool;
            fn is_file(&self, path: &Path) -> bool;
            fn is_supported(&self, path: &Path) -> bool;
            fn open(&self, path: &Path) -> anyhow::Result<Box<dyn FileWrapper>>;
        }
    }

    #[test]
    fn it_should_detect_rust_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(true);

        let detector = RustFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), Some(rust_language()));
    }

    #[test]
    fn it_should_not_detect_rust_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(false);

        let detector = RustFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("Cargo.toml")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}