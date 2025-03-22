use crate::yaml_language;
use ring_core_language::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct YamlFileDetector {}

impl YamlFileDetector {
    /// Creates a new instance of YamlFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a yaml file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_yaml::YamlFileDetector;
    ///
    /// let detector = YamlFileDetector::new();
    /// assert!(detector.is_yaml_file("assets/test.yaml"));
    /// assert!(detector.is_yaml_file("assets/test.yml"));
    /// ```
    #[inline]
    pub fn is_yaml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_yaml_file(path.as_ref())
    }

    fn _is_yaml_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        path.is_file() && matches!(path.extension().and_then(OsStr::to_str), Some("yaml") | Some("yml"))
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

    #[test]
    fn it_should_detect_yaml_language() {
        let detector = YamlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test.yaml")), Some(yaml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.yml")), Some(yaml_language()));
    }

    #[test]
    fn it_should_not_detect_yaml_language() {
        let detector = YamlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
