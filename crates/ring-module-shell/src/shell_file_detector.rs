use crate::shell_language;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use tracing::{instrument, trace};

#[derive(Clone)]
pub struct ShellFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl ShellFileDetector {
    /// Creates a new instance of ShellFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator
        }
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
    #[inline]
    pub fn is_shell_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_shell_file(path.as_ref())
    }

    fn _is_shell_file(&self, path: &Path) -> bool {
        if !self.path_adaptator.is_file(path) {
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
        } else if self._is_script_file(path) {
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
            fn is_dir(&self, path: &Path) -> bool;
            fn is_file(&self, path: &Path) -> bool;
        }
    }

    #[test]
    fn it_should_detect_shell_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(true);

        let detector = ShellFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/test")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.sh")), Some(shell_language()));
    }

    #[test]
    fn it_should_qualify_file_as_script() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(true);

        let detector = ShellFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(
            detector.qualify_path(Path::new("assets/test.sh")),
            Some((PathContent::Other("script".to_string(), &PathContent::Source), Path::new("assets/test.sh")))
        );
    }

    #[test]
    fn it_should_not_detect_shell_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file().return_const(false);

        let detector = ShellFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
