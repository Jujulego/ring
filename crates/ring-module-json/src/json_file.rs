use crate::json_language;
use ring_core_content::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct JsonFileDetector {}

impl JsonFileDetector {
    /// Creates a new instance of JsonFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a json file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_json::JsonFileDetector;
    ///
    /// let detector = JsonFileDetector::new();
    /// assert!(detector.is_json_file("assets/test.json"));
    /// ```
    #[inline]
    pub fn is_json_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_json_file(path.as_ref())
    }

    fn _is_json_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        path.is_file() && path.extension().and_then(OsStr::to_str) == Some("json")
    }
}

impl DetectLanguage for JsonFileDetector {
    #[instrument(name = "json-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_json_file(path) {
            Some(json_language())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_json_language() {
        let detector = JsonFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test.json")), Some(json_language()));
    }

    #[test]
    fn it_should_not_detect_json_language() {
        let detector = JsonFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
