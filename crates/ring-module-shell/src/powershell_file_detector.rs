use crate::powershell_language;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct PowershellFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl PowershellFileDetector {
    /// Creates a new instance of PowershellFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator
        }
    }

    /// Checks if given path is a powershell file
    #[inline]
    pub fn is_shell_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_script_file(path.as_ref())
    }

    fn _is_script_file(&self, path: &Path) -> bool {
        self.path_adaptator.is_file(path).unwrap_or(false)
            && path.extension().and_then(OsStr::to_str) == Some("ps1")
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
    use mockall::mock;
    use ring_core_fs::PathAdaptator;

    mock! {
        TestAdaptator {}

        impl PathAdaptator for TestAdaptator {
            fn is_supported(&self, path: &Path) -> bool;
            fn is_file(&self, path: &Path) -> anyhow::Result<bool>;
        }
    }

    #[test]
    fn it_should_detect_powershell_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = PowershellFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/test.ps1")), Some(powershell_language()));
    }

    #[test]
    fn it_should_qualify_file_as_script() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = PowershellFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(
            detector.qualify_path(Path::new("assets/test.ps1")),
            Some((PathContent::Other("script".to_string(), &PathContent::Source), Path::new("assets/test.ps1")))
        );
    }

    #[test]
    fn it_should_not_detect_powershell_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = PowershellFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
