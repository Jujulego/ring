use crate::shell_language;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
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

    #[inline]
    fn _is_script_file(&self, path: &Path) -> bool {
        matches!(
            path.extension().and_then(OsStr::to_str),
            Some("bash") | Some("bsh") | Some("csh") | Some("sh") | Some("zsh")
        )
    }

    #[inline]
    fn _is_bash_config_file(&self, path: &Path) -> bool {
        matches!(
            path.file_name().and_then(OsStr::to_str),
            Some(".bashrc") | Some(".bash_aliases") | Some(".bash_profile")
        )
    }

    #[inline]
    fn _is_bash_history_file(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str).is_some_and(|name| name == ".bash_history")
    }

    #[inline]
    fn _is_zsh_config_file(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str).is_some_and(|name| name == ".zshrc")
    }

    #[inline]
    fn _is_zsh_history_file(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str).is_some_and(|name| name == ".zsh_history")
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

        // Config files
        if self._is_bash_config_file(path) || self._is_zsh_config_file(path) {
            return true;
        }

        // History files
        if self._is_bash_history_file(path) {
            return true;
        }

        // Extensions
        if self._is_script_file(path) {
            return true;
        }

        // Shebangs
        trace!("read {}", path.display());
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let shebang = reader.lines()
                .map_while(Result::ok)
                .find(|line| line.starts_with("#!"));

            if shebang.as_ref().is_some_and(|l| l == "#!/bin/sh") {
                return true;
            }

            if shebang.as_ref().is_some_and(|l| l == "#!/bin/bash") {
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

impl QualifyPath for ShellFileDetector {
    #[instrument(name = "shell-file.qualify-path", skip_all)]
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        if self._is_bash_config_file(path) || self._is_zsh_config_file(path) {
            Some((PathContent::Configuration, path))
        } else if self._is_bash_history_file(path) || self._is_zsh_history_file(path) {
            Some((PathContent::Other("history".to_string(), &PathContent::Artefact), path))
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
