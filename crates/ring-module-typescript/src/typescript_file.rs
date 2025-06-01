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
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator
        }
    }

    /// Checks if given path is a typescript file
    #[inline]
    pub fn is_typescript_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_typescript_file(path.as_ref())
    }

    fn _is_typescript_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path)
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
    use ring_core_fs::VirtualFilesystem;

    #[test]
    fn it_should_detect_typescript_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test.ts", "");
        virtual_fs.add_file("test.tsx", "");
        virtual_fs.add_file("test.cts", "");
        virtual_fs.add_file("test.mts", "");

        let detector = TypescriptFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_typescript_file(Path::new("test.ts")));
        assert!(detector.is_typescript_file(Path::new("test.cts")));
        assert!(detector.is_typescript_file(Path::new("test.mts")));
        assert!(detector.is_typescript_file(Path::new("test.tsx")));

        assert_eq!(detector.detect_language(Path::new("test.ts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("test.cts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("test.mts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("test.tsx")), Some(typescript_language()));
    }

    #[test]
    fn it_should_not_detect_typescript_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/lib.rs", "");

        let detector = TypescriptFileDetector::new(Rc::new(virtual_fs));

        assert!(!detector.is_typescript_file(Path::new("do-not-exists.ts")));
        assert!(!detector.is_typescript_file(Path::new("do-not-exists.tsx")));
        assert!(!detector.is_typescript_file(Path::new("do-not-exists.cts")));
        assert!(!detector.is_typescript_file(Path::new("do-not-exists.mts")));
        assert!(!detector.is_typescript_file(Path::new("src/lib.rs")));
        assert!(!detector.is_typescript_file(Path::new("src")));

        assert_eq!(detector.detect_language(Path::new("do-not-exists.ts")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.tsx")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.cts")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.mts")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
