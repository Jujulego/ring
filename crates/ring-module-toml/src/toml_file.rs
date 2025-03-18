use crate::toml_language;
use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct TomlFileDetector {}

impl TomlFileDetector {
    /// Creates a new instance of TomlFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a toml file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_toml::TomlFileDetector;
    ///
    /// let detector = TomlFileDetector::new();
    /// assert!(detector.is_toml_file("Cargo.toml"));
    /// assert!(!detector.is_toml_file("src"));
    /// ```
    #[inline]
    pub fn is_toml_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_toml_file(path.as_ref())
    }

    fn _is_toml_file(&self, path: &Path) -> bool {
        trace!("touch path {}", path.display());
        if !path.is_file() {
            return false;
        }

        path.extension().and_then(OsStr::to_str) == Some("toml")
            || path.file_name().and_then(OsStr::to_str) == Some("Cargo.lock")
    }
}

impl DetectLanguage for TomlFileDetector {
    #[instrument(name = "toml-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_toml_file(path) {
            Some(toml_language())
        } else {
            None
        }
    }
}

impl QualifyPath for TomlFileDetector {
    #[instrument(name = "toml-file.qualify-content", skip_all)]
    fn qualify_content(&self, path: &Path) -> Option<FileContent> {
        match path.file_name().and_then(OsStr::to_str) {
            Some("Cargo.lock") => Some(FileContent::Lockfile),
            Some("Cargo.toml") => Some(FileContent::Manifest),
            _ => {
                match path.parent().and_then(Path::file_name).and_then(OsStr::to_str) {
                    Some(".cargo") | Some(".config") | Some(".github") => Some(FileContent::Configuration),
                    _ => None
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_toml_language() {
        let detector = TomlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("Cargo.toml")), Some(toml_language()));
    }

    #[test]
    fn it_should_detect_cargo_lockfile_as_toml() {
        let detector = TomlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("../../Cargo.lock")), Some(toml_language()));
    }

    #[test]
    fn it_should_not_detect_toml_language() {
        let detector = TomlFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }

    #[test]
    fn it_should_qualify_path_content() {
        let detector = TomlFileDetector::new();

        assert_eq!(detector.qualify_content(Path::new("Cargo.toml")), Some(FileContent::Manifest));
        assert_eq!(detector.qualify_content(Path::new("../../Cargo.lock")), Some(FileContent::Lockfile));
        assert_eq!(detector.qualify_content(Path::new(".cargo/config.toml")), Some(FileContent::Configuration));
        assert_eq!(detector.qualify_content(Path::new(".config/config.toml")), Some(FileContent::Configuration));
        assert_eq!(detector.qualify_content(Path::new(".github/config.toml")), Some(FileContent::Configuration));
        assert_eq!(detector.qualify_content(Path::new("test.toml")), None);
    }
}
