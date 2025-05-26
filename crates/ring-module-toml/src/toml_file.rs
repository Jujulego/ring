use crate::toml_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct TomlFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl TomlFileDetector {
    /// Creates a new instance of TomlFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> TomlFileDetector {
        TomlFileDetector {
            path_adaptator
        }
    }

    /// Checks if given path is a toml file
    #[inline]
    pub fn is_toml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_toml_file(path.as_ref())
    }

    fn _is_toml_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path)
            && path.extension().and_then(OsStr::to_str) == Some("toml")
    }
}

impl DetectLanguage for TomlFileDetector {
    #[instrument(name = "toml-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_toml_file(path) {
            Some(toml_language())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use ring_core_fs::PathAdaptator;

    mock! {
        TestAdaptator {}

        impl PathAdaptator for TestAdaptator {
            fn is_supported(&self, path: &Path) -> bool;
            fn is_dir(&self, path: &Path) -> bool;
            fn is_file(&self, path: &Path) -> bool;
        }
    }

    #[test]
    fn it_should_detect_toml_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(true);

        let detector = TomlFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/test.toml")), Some(toml_language()));
    }

    #[test]
    fn it_should_not_detect_toml_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(false);

        let detector = TomlFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
