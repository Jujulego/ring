use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};
use ring_core_language::{DetectLanguage, Language};
use crate::rust_language;

#[derive(Clone, Debug, Default)]
pub struct RustFileDetector {}

impl RustFileDetector {
    pub fn new() -> Self {
        RustFileDetector {}
    }
}

impl DetectLanguage for RustFileDetector {
    #[instrument(name = "rust-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        trace!("touch path {}", path.display());
        if path.is_file() {
            path.extension()
                .and_then(OsStr::to_str)
                .and_then(|ext| if ext == "rs" { Some(rust_language()) } else { None })
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