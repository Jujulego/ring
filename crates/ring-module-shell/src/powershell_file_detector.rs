use crate::powershell_language;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::traits::LocationMetadata;
use ring_core_fs::Filesystem;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct PowershellFileDetector {
    filesystem: Rc<Filesystem>,
}

impl PowershellFileDetector {
    /// Creates a new instance of PowershellFileDetector
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> Self {
        Self {
            filesystem
        }
    }

    /// Checks if given path is a powershell file
    #[inline]
    pub fn is_powershell_script<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_script_file(path.as_ref())
    }

    fn _is_script_file(&self, path: &Path) -> bool {
        self.filesystem.is_file(path) && path.extension().and_then(OsStr::to_str) == Some("ps1")
    }
}

impl DetectLanguage for PowershellFileDetector {
    #[instrument(name = "powershell-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_script_file(path) {
            Some(powershell_language())
        } else {
            None
        }
    }
}

impl QualifyPath for PowershellFileDetector {
    #[instrument(name = "powershell-file.qualify-path", skip_all)]
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        if self._is_script_file(path) {
            Some((PathContent::Other("script".to_string(), &PathContent::Source), path))
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
    fn it_should_detect_powershell_script() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("test.ps1", "")
        );

        let detector = PowershellFileDetector::new(Rc::new(filesystem));

        assert!(detector.is_powershell_script(Path::new("test.ps1")));

        assert_eq!(detector.detect_language(Path::new("test.ps1")), Some(powershell_language()));

        assert_eq!(
            detector.qualify_path(Path::new("test.ps1")),
            Some((PathContent::Other("script".to_string(), &PathContent::Source), Path::new("test.ps1")))
        );
    }

    #[test]
    fn it_should_not_detect_powershell_script() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("src/lib.rs", "")
        );

        let detector = PowershellFileDetector::new(Rc::new(filesystem));
        
        assert!(!detector.is_powershell_script(Path::new("do-not-exists.ps1")));
        assert!(!detector.is_powershell_script(Path::new("src/lib.rs")));
        assert!(!detector.is_powershell_script(Path::new("src")));
        
        assert_eq!(detector.detect_language(Path::new("do-not-exists.ps1")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
        
        assert_eq!(detector.qualify_path(Path::new("do-not-exists.ps1")), None);
        assert_eq!(detector.qualify_path(Path::new("src/lib.rs")), None);
        assert_eq!(detector.qualify_path(Path::new("src")), None);
    }
}
