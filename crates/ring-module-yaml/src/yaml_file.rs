use crate::yaml_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::Filesystem;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct YamlFileDetector {
    filesystem: Rc<Filesystem>,
}

impl YamlFileDetector {
    /// Creates a new instance of YamlFileDetector
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> YamlFileDetector {
        YamlFileDetector {
            filesystem
        }
    }

    /// Checks if given path is a yaml file
    #[inline]
    pub fn is_yaml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_yaml_file(path.as_ref())
    }

    fn _is_yaml_file(&self, path: &Path) -> bool {
        self.filesystem.is_file(path)
            && matches!(path.extension().and_then(OsStr::to_str), Some("yaml") | Some("yml"))
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
    use ring_core_fs::protocols::MemoryProtocol;

    #[test]
    fn it_should_detect_yaml_language() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("test.yaml", "")
                .with_file("test.yml", "")
        );

        let detector = YamlFileDetector::new(Rc::new(filesystem));

        assert!(detector.is_yaml_file(Path::new("test.yaml")));
        assert!(detector.is_yaml_file(Path::new("test.yml")));

        assert_eq!(detector.detect_language(Path::new("test.yaml")), Some(yaml_language()));
        assert_eq!(detector.detect_language(Path::new("test.yml")), Some(yaml_language()));
    }

    #[test]
    fn it_should_not_detect_yaml_language() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("src/lib.rs", "")
        );

        let detector = YamlFileDetector::new(Rc::new(filesystem));

        assert!(!detector.is_yaml_file(Path::new("does-not-exists.yaml")));
        assert!(!detector.is_yaml_file(Path::new("does-not-exists.yml")));
        assert!(!detector.is_yaml_file(Path::new("src/lib.rs")));
        assert!(!detector.is_yaml_file(Path::new("src")));

        assert_eq!(detector.detect_language(Path::new("does-not-exists.yaml")), None);
        assert_eq!(detector.detect_language(Path::new("does-not-exists.yml")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
