use crate::shell_language;
use ring_core_file::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct ShellFileDetector;

impl ShellFileDetector {
    /// Creates a new instance of ShellFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
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
        self._is_shell_file(path.as_ref())
    }

    fn _is_shell_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        if !path.is_file() {
            return false;
        }
        
        if path.extension().and_then(OsStr::to_str) == Some("sh") {
            return true;
        }
        
        trace!("read {}", path.display());
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let shebang = reader.lines()
                .map_while(Result::ok)
                .find(|line| line.starts_with("#!"));
            
            if shebang.is_some_and(|l| l == "#!/bin/sh") {
                return true;
            }
        }
        
        false
    }
}

impl DetectLanguage for ShellFileDetector {
    #[instrument(name = "shell-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_shell_file(path) {
            Some(shell_language())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_shell_language() {
        let detector = ShellFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.sh")), Some(shell_language()));
    }

    #[test]
    fn it_should_not_detect_shell_language() {
        let detector = ShellFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
