use ring_core_content::{DetectLanguage, FileContent, Language, QualifyPath};
use ring_module_json::json_language;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct TsconfigFileDetector {}

impl TsconfigFileDetector {
    /// Creates a new instance of TsconfigFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a tsconfig file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_typescript::TsconfigFileDetector;
    ///
    /// let detector = TsconfigFileDetector::new();
    /// assert!(detector.is_tsconfig("assets/tsconfig.json"));
    /// ```
    #[inline]
    pub fn is_tsconfig<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_tsconfig(path.as_ref())
    }

    fn _is_tsconfig(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        if !path.is_file() {
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
    fn qualify_file<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
        if self._is_tsconfig(path) {
            Some((FileContent::Configuration, path))
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
        let detector = TsconfigFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/tsconfig.json")), Some(json_language()));
        assert_eq!(detector.detect_language(Path::new("assets/tsconfig.test.json")), Some(json_language()));
    }

    #[test]
    fn it_should_qualify_as_config_file() {
        let detector = TsconfigFileDetector::new();

        assert_eq!(detector.qualify_file(Path::new("assets/tsconfig.json")), Some((FileContent::Configuration, Path::new("assets/tsconfig.json"))));
        assert_eq!(detector.qualify_file(Path::new("assets/tsconfig.test.json")), Some((FileContent::Configuration, Path::new("assets/tsconfig.test.json"))));
    }
}
