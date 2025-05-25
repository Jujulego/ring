use crate::yaml_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct YamlFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl YamlFileDetector {
    /// Creates a new instance of YamlFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> YamlFileDetector {
        YamlFileDetector {
            path_adaptator
        }
    }

    /// Checks if given path is a yaml file
    #[inline]
    pub fn is_yaml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_yaml_file(path.as_ref())
    }

    fn _is_yaml_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path).unwrap_or(false)
            && matches!(path.extension().and_then(OsStr::to_str), Some("yaml") | Some("yml"))
    }
}

impl DetectLanguage for YamlFileDetector {
    #[instrument(name = "yaml-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_yaml_file(path) {
            Some(yaml_language())
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
            fn is_file(&self, path: &Path) -> anyhow::Result<bool>;
        }
    }

    #[test]
    fn it_should_detect_yaml_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = YamlFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/test.yaml")), Some(yaml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.yml")), Some(yaml_language()));
    }

    #[test]
    fn it_should_not_detect_yaml_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(false));

        let detector = YamlFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
