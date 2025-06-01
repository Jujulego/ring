use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::PathAdaptator;
use ring_module_json::json_language;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct TsconfigFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl TsconfigFileDetector {
    /// Creates a new instance of TsconfigFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator
        }
    }

    /// Checks if given path is a tsconfig file
    #[inline]
    pub fn is_tsconfig<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_tsconfig(path.as_ref())
    }

    fn _is_tsconfig(&self, path: &Path) -> bool {
        if !self.path_adaptator.is_file(path) {
            return false;
        }

        path.file_name().and_then(OsStr::to_str).is_some_and(|file_name| {
            file_name.starts_with("tsconfig.") && file_name.ends_with(".json")
        })
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
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        if self._is_tsconfig(path) {
            Some((PathContent::Configuration, path))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use ring_core_fs::VirtualFilesystem;
    use super::*;
    
    #[test]
    fn it_should_detect_tsconfig_file() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("tsconfig.json", "");
        virtual_fs.add_file("tsconfig.suffix.json", "");

        let detector = TsconfigFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_tsconfig(Path::new("tsconfig.json")));
        assert!(detector.is_tsconfig(Path::new("tsconfig.suffix.json")));

        assert_eq!(detector.detect_language(Path::new("tsconfig.json")), Some(json_language()));
        assert_eq!(detector.detect_language(Path::new("tsconfig.suffix.json")), Some(json_language()));

        assert_eq!(detector.qualify_path(Path::new("tsconfig.json")), Some((PathContent::Configuration, Path::new("tsconfig.json"))));
        assert_eq!(detector.qualify_path(Path::new("tsconfig.suffix.json")), Some((PathContent::Configuration, Path::new("tsconfig.suffix.json"))));
    }
    
    #[test]
    fn it_should_not_detect_tsconfig_file() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/lib.rs", "");

        let detector = TsconfigFileDetector::new(Rc::new(virtual_fs));

        assert!(!detector.is_tsconfig(Path::new("tsconfig.json")));
        assert!(!detector.is_tsconfig(Path::new("tsconfig.do-not-exists.json")));
        assert!(!detector.is_tsconfig(Path::new("src/lib.rs")));
        assert!(!detector.is_tsconfig(Path::new("src")));
        
        assert_eq!(detector.detect_language(Path::new("tsconfig.json")), None);
        assert_eq!(detector.detect_language(Path::new("tsconfig.do-not-exists.json")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);

        assert_eq!(detector.qualify_path(Path::new("tsconfig.json")), None);
        assert_eq!(detector.qualify_path(Path::new("tsconfig.do-not-exists.json")), None);
        assert_eq!(detector.qualify_path(Path::new("src/lib.rs")), None);
        assert_eq!(detector.qualify_path(Path::new("src")), None);
    }
}
