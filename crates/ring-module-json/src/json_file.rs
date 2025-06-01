use crate::json_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct JsonFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl JsonFileDetector {
    /// Creates a new instance of JsonFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator
        }
    }

    /// Checks if given path is a json file
    #[inline]
    pub fn is_json_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_json_file(path.as_ref())
    }

    fn _is_json_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path) 
            && path.extension().and_then(OsStr::to_str) == Some("json")
    }
}

impl DetectLanguage for JsonFileDetector {
    #[instrument(name = "json-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_json_file(path) {
            Some(json_language())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring_core_fs::VirtualFilesystem;

    #[test]
    fn it_should_detect_json_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test.json", "{}");

        let detector = JsonFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_json_file(Path::new("test.json")));

        assert_eq!(detector.detect_language(Path::new("test.json")), Some(json_language()));
    }

    #[test]
    fn it_should_not_detect_json_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/lib.rs", "");

        let detector = JsonFileDetector::new(Rc::new(virtual_fs));

        assert!(!detector.is_json_file(Path::new("do-not-exists.json")));
        assert!(!detector.is_json_file(Path::new("src/lib.rs")));
        assert!(!detector.is_json_file(Path::new("src")));

        assert_eq!(detector.detect_language(Path::new("do-not-exists.json")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
