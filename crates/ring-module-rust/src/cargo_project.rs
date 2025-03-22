use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use ring_module_toml::toml_language;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct CargoProjectDetector {}

impl CargoProjectDetector {
    /// Creates a new instance of CargoManifestDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a Cargo manifest
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_manifest("Cargo.toml"));
    /// assert!(!detector.is_manifest("src"));
    /// ```
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_manifest(path)
    }

    fn _is_manifest(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("Cargo.toml")
    }

    /// Checks if given path is a Cargo lockfile
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_lockfile("../../Cargo.lock"));
    /// assert!(!detector.is_lockfile("src"));
    /// ```
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_lockfile(path)
    }

    fn _is_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("Cargo.lock")
    }

    /// Checks if given path is a Cargo crate
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_crate("."));
    /// assert!(!detector.is_crate("src"));
    /// ```
    pub fn is_crate<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_dir() && self._is_crate(path)
    }

    fn _is_crate(&self, path: &Path) -> bool {
        self.is_manifest(path.join("Cargo.toml"))
    }
}

impl DetectLanguage for CargoProjectDetector {
    #[instrument(name = "cargo-manifest.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_manifest(path) || self._is_lockfile(path) {
            Some(toml_language())
        } else {
            None
        }
    }
}

impl QualifyPath for CargoProjectDetector {
    #[instrument(name = "cargo-manifest.qualify-path", skip_all)]
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
        trace!("stat {}", path.display());
        let path_is_file = path.is_file();
        
        for ancestor in path.ancestors() {
            if let Some(parent) = ancestor.parent() {
                if !self._is_crate(parent) {
                    continue;
                }
            } else {
                break;
            }

            trace!("stat {}", ancestor.display());
            match (ancestor.file_name().and_then(OsStr::to_str), path_is_file, ancestor.is_file()) {
                (Some(".cargo"), true, false) => return Some((FileContent::Configuration, ancestor)),
                (Some("build.rs"), true, true) => return Some((FileContent::Other("build".into()), ancestor)),
                (Some("Cargo.toml"), true, true) => return Some((FileContent::Manifest, ancestor)),
                (Some("Cargo.lock"), true, true) => return Some((FileContent::Lockfile, ancestor)),
                (Some("src"), true, false) => return Some((FileContent::Source, ancestor)),
                (Some("tests"), true, false) => return Some((FileContent::Tests, ancestor)),
                _ => continue,
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_toml_language() {
        let detector = CargoProjectDetector::new();

        assert_eq!(detector.detect_language(Path::new("Cargo.toml")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("../../Cargo.lock")), Some(toml_language()));
    }

    #[test]
    fn it_should_qualify_path_content() {
        let detector = CargoProjectDetector::new();

        assert_eq!(
            detector.qualify_content(Path::new("assets/.cargo/config")),
            Some((FileContent::Configuration, Path::new("assets/.cargo")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/build.rs")),
            Some((FileContent::Source, Path::new("assets/build.rs")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/Cargo.toml")),
            Some((FileContent::Manifest, Path::new("assets/Cargo.toml")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/Cargo.lock")),
            Some((FileContent::Lockfile, Path::new("assets/Cargo.lock")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/src/lib.rs")),
            Some((FileContent::Source, Path::new("assets/src")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/tests/test.rs")),
            Some((FileContent::Tests, Path::new("assets/tests")))
        );
        assert_eq!(detector.qualify_content(Path::new("do-not-exists.toml")), None);
    }
}
