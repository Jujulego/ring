use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::PathAdaptator;
use ring_module_json::json_language;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct TsconfigFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl TsconfigFileDetector {
    /// Creates a new instance of TsconfigFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator
        }
    }

    /// Checks if given path is a tsconfig file
    #[inline]
    pub fn is_tsconfig<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_tsconfig(path.as_ref())
    }

    fn _is_tsconfig(&self, path: &Path) -> bool {
        if !self.path_adaptator.is_file(path).unwrap_or(false) {
            return false;
        }

        if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
            file_name.starts_with("tsconfig.") && file_name.ends_with(".json")
        } else {
            false
        }
    }
}

impl DetectLanguage for TsconfigFileDetector {
    #[instrument(name = "tsconfig-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_tsconfig(path) {
            Some(json_language())
        } else {
            None
        }
    }
}

impl QualifyPath for TsconfigFileDetector {
    #[instrument(name = "tsconfig-file.qualify-path", skip_all)]
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        if self._is_tsconfig(path) {
            Some((PathContent::Configuration, path))
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
    fn it_should_detect_json_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = TsconfigFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/tsconfig.json")), Some(json_language()));
        assert_eq!(detector.detect_language(Path::new("assets/tsconfig.test.json")), Some(json_language()));
    }

    #[test]
    fn it_should_qualify_as_config_file() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = TsconfigFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.qualify_path(Path::new("assets/tsconfig.json")), Some((PathContent::Configuration, Path::new("assets/tsconfig.json"))));
        assert_eq!(detector.qualify_path(Path::new("assets/tsconfig.test.json")), Some((PathContent::Configuration, Path::new("assets/tsconfig.test.json"))));
    }
}
