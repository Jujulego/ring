use ring_core_language::{DetectLanguage, Language};
use ring_module_json::json_language;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};
use ring_core_file::{FileContent, QualifyPath};

#[derive(Clone, Debug, Default)]
pub struct NpmPackageDetector {}

impl NpmPackageDetector {
    /// Creates a new instance of NpmPackageDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a npm package manifest
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_manifest("assets/package.json"));
    /// ```
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_manifest(path)
    }

    fn _is_manifest(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("package.json")
    }
}

impl DetectLanguage for NpmPackageDetector {
    #[instrument(name = "npm-package.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        trace!("stat {}", path.display());
        if path.is_file() && self._is_manifest(path) {
            Some(json_language())
        } else {
            None
        }
    }
}

impl QualifyPath for NpmPackageDetector {
    #[instrument(name = "npm-package.qualify-path", skip_all)]
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
        trace!("stat {}", path.display());
        if path.is_file() && self._is_manifest(path) {
            Some((FileContent::Manifest, path))
        } else {
            None
        }
    }
}