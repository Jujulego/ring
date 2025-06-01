use crate::shell_language;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::traits::AbstractFilesystem;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct ShellFileDetector {
    filesystem: Rc<dyn AbstractFilesystem>,
}

impl ShellFileDetector {
    /// Creates a new instance of ShellFileDetector
    #[inline]
    pub fn new(filesystem: Rc<dyn AbstractFilesystem>) -> Self {
        Self {
            filesystem
        }
    }

    #[inline]
    fn _is_script_file(&self, path: &Path) -> bool {
        // Extensions
        if matches!(
            path.extension().and_then(OsStr::to_str),
            Some("bash") | Some("bsh") | Some("csh") | Some("sh") | Some("zsh")
        ) {
            return true;
        }

        // Shebangs
        let Ok(file) = self.filesystem.open(path) else { return false };

        let reader = BufReader::new(file);
        let shebang = reader.lines()
            .map_while(Result::ok)
            .next();

        shebang.as_ref().is_some_and(|l| l == "#!/bin/sh")
            || shebang.as_ref().is_some_and(|l| l == "#!/bin/bash")
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

    /// Checks if given path is a shell script
    #[inline]
    pub fn is_shell_script<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_shell_script(path.as_ref())
    }

    fn _is_shell_script(&self, path: &Path) -> bool {
        if !self.filesystem.is_file(path) {
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

        // Scripts
        self._is_script_file(path)
    }
}

impl DetectLanguage for ShellFileDetector {
    #[instrument(name = "shell-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_shell_script(path) {
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
    use ring_core_fs::filesystem::VirtualFilesystem;

    #[test]
    fn it_should_detect_shell_script_using_extension() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test.bash", "");
        virtual_fs.add_file("test.bsh", "");
        virtual_fs.add_file("test.csh", "");
        virtual_fs.add_file("test.sh", "");
        virtual_fs.add_file("test.zsh", "");

        let detector = ShellFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_shell_script(Path::new("test.bash")));
        assert!(detector.is_shell_script(Path::new("test.bsh")));
        assert!(detector.is_shell_script(Path::new("test.csh")));
        assert!(detector.is_shell_script(Path::new("test.sh")));
        assert!(detector.is_shell_script(Path::new("test.zsh")));

        assert_eq!(detector.detect_language(Path::new("test.bash")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new("test.bsh")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new("test.csh")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new("test.sh")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new("test.zsh")), Some(shell_language()));

        let script = PathContent::Other("script".to_string(), &PathContent::Source);

        assert_eq!(detector.qualify_path(Path::new("test.bash")), Some((script.clone(), Path::new("test.bash"))));
        assert_eq!(detector.qualify_path(Path::new("test.bsh")), Some((script.clone(), Path::new("test.bsh"))));
        assert_eq!(detector.qualify_path(Path::new("test.csh")), Some((script.clone(), Path::new("test.csh"))));
        assert_eq!(detector.qualify_path(Path::new("test.sh")), Some((script.clone(), Path::new("test.sh"))));
        assert_eq!(detector.qualify_path(Path::new("test.zsh")), Some((script.clone(), Path::new("test.zsh"))));
    }

    #[test]
    fn it_should_detect_shell_script_using_bash_shebang() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test", "#!/bin/bash");

        let detector = ShellFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_shell_script(Path::new("test")));
        assert_eq!(detector.detect_language(Path::new("test")), Some(shell_language()));

        assert_eq!(
            detector.qualify_path(Path::new("test")),
            Some((PathContent::Other("script".to_string(), &PathContent::Source), Path::new("test")))
        );
    }

    #[test]
    fn it_should_detect_shell_script_using_shell_shebang() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test", "#!/bin/sh");

        let detector = ShellFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_shell_script(Path::new("test")));
        assert_eq!(detector.detect_language(Path::new("test")), Some(shell_language()));

        assert_eq!(
            detector.qualify_path(Path::new("test")),
            Some((PathContent::Other("script".to_string(), &PathContent::Source), Path::new("test")))
        );
    }

    #[test]
    fn it_should_detect_shell_config_files() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(".bashrc", "");
        virtual_fs.add_file(".bash_aliases", "");
        virtual_fs.add_file(".bash_profile", "");
        virtual_fs.add_file(".zshrc", "");

        let detector = ShellFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_shell_script(Path::new(".bashrc")));
        assert!(detector.is_shell_script(Path::new(".bash_aliases")));
        assert!(detector.is_shell_script(Path::new(".bash_profile")));
        assert!(detector.is_shell_script(Path::new(".zshrc")));

        assert_eq!(detector.detect_language(Path::new(".bashrc")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new(".bash_aliases")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new(".bash_profile")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new(".zshrc")), Some(shell_language()));

        assert_eq!(detector.qualify_path(Path::new(".bashrc")), Some((PathContent::Configuration, Path::new(".bashrc"))));
        assert_eq!(detector.qualify_path(Path::new(".bash_aliases")), Some((PathContent::Configuration, Path::new(".bash_aliases"))));
        assert_eq!(detector.qualify_path(Path::new(".bash_profile")), Some((PathContent::Configuration, Path::new(".bash_profile"))));
        assert_eq!(detector.qualify_path(Path::new(".zshrc")), Some((PathContent::Configuration, Path::new(".zshrc"))));
    }

    #[test]
    fn it_should_detect_shell_history_files() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(".bash_history", "");
        virtual_fs.add_file(".zsh_history", "");

        let detector = ShellFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_shell_script(Path::new(".bash_history")));
        assert!(!detector.is_shell_script(Path::new(".zsh_history")));

        assert_eq!(detector.detect_language(Path::new(".bash_history")), Some(shell_language()));
        assert_eq!(detector.detect_language(Path::new(".zsh_history")), None);

        let history = PathContent::Other("history".to_string(), &PathContent::Artefact);

        assert_eq!(detector.qualify_path(Path::new(".bash_history")), Some((history.clone(), Path::new(".bash_history"))));
        assert_eq!(detector.qualify_path(Path::new(".zsh_history")), Some((history.clone(), Path::new(".zsh_history"))));
    }

    #[test]
    fn it_should_not_detect_shell_script() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/lib.rs", "");

        let detector = ShellFileDetector::new(Rc::new(virtual_fs));

        assert_eq!(detector.detect_language(Path::new("do-not-exists.bash")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.bsh")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.csh")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.sh")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.zsh")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
