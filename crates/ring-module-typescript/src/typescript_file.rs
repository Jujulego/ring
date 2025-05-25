use crate::typescript_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct TypescriptFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl TypescriptFileDetector {
    /// Creates a new instance of TypescriptFileDetector
    #[inline]
    pub fn new(path_tools: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator: path_tools
        }
    }

    /// Checks if given path is a typescript file
    #[inline]
    pub fn is_typescript_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_typescript_file(path.as_ref())
    }

    fn _is_typescript_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path).unwrap_or(false)
            && matches!(path.extension().and_then(OsStr::to_str), Some("ts") | Some("cts") | Some("mts") | Some("tsx"))
    }
}

impl DetectLanguage for TypescriptFileDetector {
    #[instrument(name = "typescript-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_typescript_file(path) {
            Some(typescript_language())
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
    fn it_should_detect_typescript_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = TypescriptFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/test.ts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.cts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.mts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.tsx")), Some(typescript_language()));
    }

    #[test]
    fn it_should_not_detect_typescript_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(false));

        let detector = TypescriptFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
