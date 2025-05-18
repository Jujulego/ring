use crate::utils::powershell_language;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use std::ffi::OsStr;
use std::path::Path;
use tracing::instrument;

#[derive(Clone, Debug, Default)]
pub struct PowershellFileDetector;

impl PowershellFileDetector {
    /// Creates a new instance of PowershellFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn _is_script_file(&self, path: &Path) -> bool {
        path.extension().and_then(OsStr::to_str) == Some("ps1")
    }

    /// Checks if given path is a shell file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_shell::ShellFileDetector;
    ///
    /// let detector = ShellFileDetector::new();
    /// assert!(detector.is_shell_file("assets/test.sh"));
    /// ```
    #[inline]
    pub fn is_shell_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_script_file(path.as_ref())
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

    #[test]
    fn it_should_detect_powershell_language() {
        let detector = PowershellFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test.ps1")), Some(powershell_language()));
    }

    #[test]
    fn it_should_not_detect_powershell_language() {
        let detector = PowershellFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
