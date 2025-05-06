use crate::cargo_crate::CargoCrate;
use anyhow::anyhow;
use cargo_toml::Manifest;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_units::{DetectUnit, Unit};
use ring_module_toml::toml_language;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, instrument, trace, warn};

/// Detector for cargo crates, and related files
#[derive(Clone, Debug)]
pub struct CargoCrateDetector {
    cache: RefCell<HashMap<PathBuf, Option<Rc<CargoCrate>>>>,
}

impl CargoCrateDetector {
    /// Creates a new instance of CargoCrateDetector
    #[inline]
    pub fn new() -> Self {
        CargoCrateDetector {
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Checks if given path is a Cargo manifest
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoCrateDetector;
    ///
    /// let detector = CargoCrateDetector::new();
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
    /// use ring_module_rust::CargoCrateDetector;
    ///
    /// let detector = CargoCrateDetector::new();
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
    /// use ring_module_rust::CargoCrateDetector;
    ///
    /// let detector = CargoCrateDetector::new();
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
    /// use ring_module_rust::CargoCrateDetector;
    ///
    /// let detector = CargoCrateDetector::new();
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

    /// Load crate data at given path
    pub fn load_crate_at<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<CargoCrate>>> {
        self._load_crate_at(path.as_ref())
    }

    /// Load crate data containing given path
    pub fn load_crate_containing<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<CargoCrate>>> {
        let path = path.as_ref();
        
        path.ancestors()
            .find_map(|ancestor| self._load_crate_at(ancestor).transpose())
            .transpose()
    }

    fn _load_crate_at(&self, path: &Path) -> anyhow::Result<Option<Rc<CargoCrate>>> {
        let manifest_path = path.join("Cargo.toml");

        if let Some(crt) = self.cache.borrow().get(&manifest_path) {
            debug!(key = %manifest_path.display(), "cargo crate cache hit");
            return Ok(crt.clone());
        }

        trace!("read file {}", manifest_path.display());
        match Manifest::from_path(&manifest_path) {
            Ok(manifest) => {
                let crt = Some(Rc::new(CargoCrate::new(manifest, path.to_path_buf())));

                debug!(key = %manifest_path.display(), "cargo crate cached");
                self.cache.borrow_mut().insert(manifest_path, crt.clone());

                Ok(crt)
            },
            Err(cargo_toml::Error::Io(err)) if err.kind() == io::ErrorKind::NotFound => {
                debug!(key = %manifest_path.display(), "cargo crate miss cached");
                self.cache.borrow_mut().insert(manifest_path, None);

                Ok(None)
            },
            Err(err) => {
                Err(anyhow!(err).context(format!("Failed to load {}", manifest_path.display())))
            }
        }
    }
}

impl Default for CargoCrateDetector {
    fn default() -> Self {
        CargoCrateDetector::new()
    }
}

impl DetectLanguage for CargoCrateDetector {
    #[instrument(name = "cargo-crate.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        trace!("stat {}", path.display());
        if path.is_file() && (self._is_manifest(path) || self._is_lockfile(path) || self._is_cargo_config(path)) {
            Some(toml_language())
        } else {
            None
        }
    }
}

impl DetectUnit for CargoCrateDetector {
    #[instrument(name = "cargo-crate.detect-unit", skip_all)]
    fn detect_unit(&self, path: &Path) -> Option<Rc<dyn Unit>> {
        match self._load_crate_at(path) {
            Ok(result) => result.map(|unit| unit as Rc<dyn Unit>),
            Err(err) => {
                warn!("{}", err);
                if let Some(source) = err.source() {
                    debug!("Error caused by: {}", source);
                }

                None
            }
        }
    }
}

impl QualifyPath for CargoCrateDetector {
    #[instrument(name = "cargo-crate.qualify-path", skip_all)]
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        // Out of crate cases
        trace!("stat {}", path.display());
        if path.is_file() {
            if self._is_manifest(path) { // manifest defines the folder as a crate
                return Some((PathContent::Manifest, path));
            } else if self._is_cargo_config(path) {
                return Some((PathContent::Configuration, path));
            }
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
                    return Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), ancestor));
                }

                if ancestor.file_name().and_then(OsStr::to_str) == Some("build.rs") {
                    return Some((PathContent::Other("build".into(), &PathContent::Configuration), ancestor))
                }
            } else {
                match ancestor.file_name().and_then(OsStr::to_str) {
                    Some("src") => return Some((PathContent::Source, ancestor)),
                    Some("target") => return Some((PathContent::Artefact, ancestor)),
                    Some("tests") => return Some((PathContent::Test, ancestor)),
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
        let detector = CargoCrateDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/.cargo/config")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/.cargo/config.toml")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/Cargo.toml")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/Cargo.lock")), Some(toml_language()));
    }

    #[test]
    fn it_should_qualify_path_content() {
        let detector = CargoCrateDetector::new();

        assert_eq!(
            detector.qualify_path(Path::new("assets/.cargo/config")),
            Some((PathContent::Configuration, Path::new("assets/.cargo/config")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("assets/.cargo/config.toml")),
            Some((PathContent::Configuration, Path::new("assets/.cargo/config.toml")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("assets/build.rs")),
            Some((PathContent::Other("build".into(), &PathContent::Configuration), Path::new("assets/build.rs")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("assets/Cargo.toml")),
            Some((PathContent::Manifest, Path::new("assets/Cargo.toml")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("assets/Cargo.lock")),
            Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("assets/Cargo.lock")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("assets/src/lib.rs")),
            Some((PathContent::Source, Path::new("assets/src")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("assets/tests/test.rs")),
            Some((PathContent::Test, Path::new("assets/tests")))
        );
        assert_eq!(detector.qualify_path(Path::new("do-not-exists.toml")), None);
    }
}
