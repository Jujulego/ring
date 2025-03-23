use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use ring_module_toml::toml_language;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct CargoProjectDetector {}

impl CargoProjectDetector {
    /// Creates a new instance of CargoProjectDetector
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
    /// assert!(detector.is_manifest("assets/Cargo.toml"));
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
    /// assert!(detector.is_lockfile("assets/Cargo.lock"));
    /// ```
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_lockfile(path)
    }

    fn _is_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("Cargo.lock")
    }

    /// Checks if given path is a Cargo config file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_cargo_config("assets/.cargo/config"));
    /// assert!(detector.is_cargo_config("assets/.cargo/config.toml"));
    /// ```
    pub fn is_cargo_config<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_cargo_config(path)
    }

    fn _is_cargo_config(&self, path: &Path) -> bool {
        path.parent().and_then(|p| p.file_name()).and_then(OsStr::to_str) == Some(".cargo")
            && path.file_stem().and_then(OsStr::to_str) == Some("config")
            && path.extension().and_then(OsStr::to_str).is_none_or(|ext| ext == "toml")
    }

    /// Checks if given path is a Cargo crate
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_crate("assets"));
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
    #[instrument(name = "cargo-project.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        trace!("stat {}", path.display());
        if path.is_file() && (self._is_manifest(path) || self._is_lockfile(path) || self._is_cargo_config(path)) {
            Some(toml_language())
        } else {
            None
        }
    }
}

impl QualifyPath for CargoProjectDetector {
    #[instrument(name = "cargo-project.qualify-path", skip_all)]
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
        // Out of crate cases
        trace!("stat {}", path.display());
        if path.is_file() {
            if self._is_manifest(path) { // manifest defines the folder as a crate
                return Some((FileContent::Manifest, path));
            } else if self._is_cargo_config(path) {
                return Some((FileContent::Configuration, path));
            }
        } else {
            return None;
        }

        // In crate cases
        for ancestor in path.ancestors() {
            if let Some(parent) = ancestor.parent() {
                if !self._is_crate(parent) {
                    continue;
                }
            } else {
                break;
            }

            trace!("stat {}", ancestor.display());
            if ancestor.is_file() {
                if self._is_lockfile(ancestor) {
                    return Some((FileContent::Lockfile, ancestor));
                }

                if ancestor.file_name().and_then(OsStr::to_str) == Some("build.rs") {
                    return Some((FileContent::Other("build".into()), ancestor))
                }
            } else {
                match ancestor.file_name().and_then(OsStr::to_str) {
                    Some("src") => return Some((FileContent::Source, ancestor)),
                    Some("tests") => return Some((FileContent::Tests, ancestor)),
                    _ => {},
                }
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

        assert_eq!(detector.detect_language(Path::new("assets/.cargo/config")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/.cargo/config.toml")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/Cargo.toml")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/Cargo.lock")), Some(toml_language()));
    }

    #[test]
    fn it_should_qualify_path_content() {
        let detector = CargoProjectDetector::new();

        assert_eq!(
            detector.qualify_content(Path::new("assets/.cargo/config")),
            Some((FileContent::Configuration, Path::new("assets/.cargo/config")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/.cargo/config.toml")),
            Some((FileContent::Configuration, Path::new("assets/.cargo/config.toml")))
        );
        assert_eq!(
            detector.qualify_content(Path::new("assets/build.rs")),
            Some((FileContent::Other("build".into()), Path::new("assets/build.rs")))
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
