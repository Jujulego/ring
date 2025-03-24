use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};
use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use ring_module_json::json_language;

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
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
        if self._is_tsconfig(path) {
            Some((FileContent::Configuration, path))
        } else {
            None
        }
    }
}