use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};
use ring_core_language::{DetectLanguage, Language};
use crate::rust_language;

#[derive(Clone, Debug, Default)]
pub struct RustFileDetector {}

impl RustFileDetector {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
    
    #[inline]
    pub fn is_rust_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_rust_file(path.as_ref())
    }
    
    fn _is_rust_file(&self, path: &Path) -> bool {
        trace!("touch path {}", path.display());
        path.is_file() && path.extension()
            .and_then(OsStr::to_str)
            .map_or(false, |ext| ext == "rs")
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
}