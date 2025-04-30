use crate::typescript_language;
use ring_core_content::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct TypescriptFileDetector {}

impl TypescriptFileDetector {
    /// Creates a new instance of TypescriptFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a typescript file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_typescript::TypescriptFileDetector;
    ///
    /// let detector = TypescriptFileDetector::new();
    /// assert!(detector.is_typescript_file("assets/test.ts"));
    /// ```
    #[inline]
    pub fn is_typescript_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_typescript_file(path.as_ref())
    }

    fn _is_typescript_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        path.is_file() && matches!(path.extension().and_then(OsStr::to_str), Some("ts") | Some("cts") | Some("mts") | Some("tsx"))
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

    #[test]
    fn it_should_detect_typescript_language() {
        let detector = TypescriptFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test.ts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.cts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.mts")), Some(typescript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.tsx")), Some(typescript_language()));
    }

    #[test]
    fn it_should_not_detect_typescript_language() {
        let detector = TypescriptFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
