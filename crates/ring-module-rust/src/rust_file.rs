use crate::rust_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::traits::AbstractFilesystem;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

/// Detector for rust files
#[derive(Clone)]
pub struct RustFileDetector {
    filesystem: Rc<dyn AbstractFilesystem>,
}

impl RustFileDetector {
    /// Creates a new instance of RustFileDetector
    #[inline]
    pub fn new(filesystem: Rc<dyn AbstractFilesystem>) -> Self {
        Self {
            filesystem,
        }
    }

    /// Checks if given path is a rust file
    #[inline]
    pub fn is_rust_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_rust_file(path.as_ref())
    }
    
    fn _is_rust_file(&self, path: &Path) -> bool {
        self.filesystem.is_file(path)
            && path.extension() == Some(OsStr::new("rs"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use ring_core_fs::filesystem::VirtualFilesystem;

    #[test]
    fn it_should_detect_rust_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/main.rs", "");

        let detector = RustFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_rust_file(Path::new("src/main.rs")));
        assert_eq!(detector.detect_language(Path::new("src/main.rs")), Some(rust_language()));
    }

    #[test]
    fn it_should_not_detect_rust_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/main.js", "");

        let detector = RustFileDetector::new(Rc::new(virtual_fs));

        assert!(!detector.is_rust_file(Path::new("do-not-exist.rs")));
        assert!(!detector.is_rust_file(Path::new("src/main.js")));
        assert!(!detector.is_rust_file(Path::new("src")));

        assert_eq!(detector.detect_language(Path::new("do-not-exist.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src/main.js")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}