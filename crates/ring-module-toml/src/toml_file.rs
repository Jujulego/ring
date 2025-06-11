use crate::toml_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::Filesystem;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct TomlFileDetector {
    filesystem: Rc<Filesystem>,
}

impl TomlFileDetector {
    /// Creates a new instance of TomlFileDetector
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> TomlFileDetector {
        TomlFileDetector {
            filesystem
        }
    }

    /// Checks if given path is a toml file
    #[inline]
    pub fn is_toml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_toml_file(path.as_ref())
    }

    fn _is_toml_file(&self, path: &Path) -> bool {
        self.filesystem.is_file(path)
            && path.extension().and_then(OsStr::to_str) == Some("toml")
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
    use ring_core_fs::protocols::MemoryProtocol;

    #[test]
    fn it_should_detect_toml_language() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("test.toml", "")
        );

        let detector = TomlFileDetector::new(Rc::new(filesystem));

        assert!(detector.is_toml_file(Path::new("test.toml")));

        assert_eq!(detector.detect_language(Path::new("test.toml")), Some(toml_language()));
    }

    #[test]
    fn it_should_not_detect_toml_language() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("src/lib.rs", "")
        );

        let detector = TomlFileDetector::new(Rc::new(filesystem));

        assert!(!detector.is_toml_file(Path::new("does-not-exists.toml")));
        assert!(!detector.is_toml_file(Path::new("src/lib.rs")));
        assert!(!detector.is_toml_file(Path::new("src")));

        assert_eq!(detector.detect_language(Path::new("does-not-exists.toml")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
