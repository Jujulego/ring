use crate::toml_language;
use ring_core_file::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct TomlFileDetector {}

impl TomlFileDetector {
    /// Creates a new instance of TomlFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a toml file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_toml::TomlFileDetector;
    ///
    /// let detector = TomlFileDetector::new();
    /// assert!(detector.is_toml_file("Cargo.toml"));
    /// assert!(!detector.is_toml_file("src"));
    /// ```
    #[inline]
    pub fn is_toml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_toml_file(path.as_ref())
    }

    fn _is_toml_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        path.is_file() && path.extension().and_then(OsStr::to_str) == Some("toml")
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

    #[test]
    fn it_should_detect_toml_language() {
        let detector = TomlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test.toml")), Some(toml_language()));
    }

    #[test]
    fn it_should_not_detect_toml_language() {
        let detector = TomlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
