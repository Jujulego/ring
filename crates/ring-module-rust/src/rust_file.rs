use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};
use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use crate::rust_language;

#[derive(Clone, Debug, Default)]
pub struct RustFileDetector {}

impl RustFileDetector {
    /// Creates a new instance of RustFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a rust file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::RustFileDetector;
    ///
    /// let detector = RustFileDetector::new();
    /// assert!(detector.is_rust_file("src/lib.rs"));
    /// assert!(!detector.is_rust_file("src"));
    /// ```
    #[inline]
    pub fn is_rust_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_rust_file(path.as_ref())
    }
    
    fn _is_rust_file(&self, path: &Path) -> bool {
        trace!("touch path {}", path.display());
        path.is_file() && path.extension().and_then(OsStr::to_str) == Some("rs")
    }
}

impl DetectLanguage for RustFileDetector {
    #[instrument(name = "rust-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_rust_file(path) {
            Some(rust_language())
        } else {
            None
        }
    }
}

impl QualifyPath for RustFileDetector {
    #[instrument(name = "rust-file.qualify-content", skip_all)]
    fn qualify_content(&self, path: &Path) -> Option<FileContent> {
        if !self.is_rust_file(path) {
            return None;
        }
        
        Some(FileContent::Source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_rust_language() {
        let detector = RustFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), Some(rust_language()));
    }

    #[test]
    fn it_should_not_detect_rust_language() {
        let detector = RustFileDetector {};

        assert_eq!(detector.detect_language(Path::new("Cargo.toml")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
    
    #[test]
    fn it_should_qualify_path_as_source() {
        let detector = RustFileDetector {};
        
        assert_eq!(detector.qualify_content(Path::new("do-not-exists.rs")), None);
        assert_eq!(detector.qualify_content(Path::new("src/lib.rs")), Some(FileContent::Source));
    }
}