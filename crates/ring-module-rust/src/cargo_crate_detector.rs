use crate::cargo_crate::CargoCrate;
use anyhow::{anyhow, Context};
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::traits::AbstractFilesystem;
use ring_core_fs::Error;
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
#[derive(Clone)]
pub struct CargoCrateDetector {
    cache: RefCell<HashMap<PathBuf, Option<Rc<CargoCrate>>>>,
    filesystem: Rc<dyn AbstractFilesystem>,
}

impl CargoCrateDetector {
    /// Creates a new instance of CargoCrateDetector
    #[inline]
    pub fn new(filesystem: Rc<dyn AbstractFilesystem>) -> Self {
        CargoCrateDetector {
            cache: RefCell::new(HashMap::new()),
            filesystem,
        }
    }

    /// Checks if given path is a Cargo manifest
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_file(path) && self._is_manifest(path)
    }

    fn _is_manifest(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("Cargo.toml")
    }

    /// Checks if given path is a Cargo lockfile
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        path.parent().is_some_and(|parent| self.is_crate(parent))
            && self.filesystem.is_file(path)
            && self._is_lockfile(path)
    }

    fn _is_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("Cargo.lock")
    }

    /// Checks if given path is a Cargo config file
    pub fn is_cargo_config<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_file(path) && self._is_cargo_config(path)
    }

    fn _is_cargo_config(&self, path: &Path) -> bool {
        path.parent().and_then(|p| p.file_name()) == Some(OsStr::new(".cargo"))
            && path.file_stem() == Some(OsStr::new("config"))
            && path.extension().is_none_or(|ext| ext == OsStr::new("toml"))
    }

    /// Checks if given path is a Cargo crate
    pub fn is_crate<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_dir(path) && self._is_crate(path)
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
        match self.filesystem.open(&manifest_path) {
            Ok(file) => {
                let content = io::read_to_string(file)
                    .context(format!("Failed to read {}", manifest_path.display()))?;
                
                let manifest = toml::from_str(&content)
                    .context(format!("Failed to parse {}", manifest_path.display()))?;

                let crt = Some(Rc::new(CargoCrate::new(manifest, path.to_path_buf())));

                debug!(key = %manifest_path.display(), "cargo crate cached");
                self.cache.borrow_mut().insert(manifest_path, crt.clone());

                Ok(crt)
            }
            Err(Error::NotFound(_)) => {
                debug!(key = %manifest_path.display(), "cargo crate miss cached");
                self.cache.borrow_mut().insert(manifest_path, None);

                Ok(None)
            }
            Err(err) => {
                Err(anyhow!(err).context(format!("Failed to load {}", manifest_path.display())))
            }
        }
    }
}

impl DetectLanguage for CargoCrateDetector {
    #[instrument(name = "cargo-crate.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        let is_file = self.filesystem.is_file(path);
        
        if is_file && (self._is_manifest(path) || self._is_lockfile(path) || self._is_cargo_config(path)) {
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
        if self.filesystem.is_file(path) {
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

            if self.filesystem.is_file(ancestor) {
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
    use indoc::indoc;
    use ring_core_fs::filesystem::VirtualFilesystem;

    #[test]
    fn it_should_detect_cargo_manifest() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(Path::new("Cargo.toml"), "");

        let detector = CargoCrateDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_manifest(Path::new("Cargo.toml")));
        assert_eq!(detector.detect_language(Path::new("Cargo.toml")), Some(toml_language()));
        assert_eq!(detector.qualify_path(Path::new("Cargo.toml")), Some((PathContent::Manifest, Path::new("Cargo.toml"))));
    }

    #[test]
    fn it_should_detect_cargo_lockfile() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(Path::new("/Cargo.toml"), "");
        virtual_fs.add_file(Path::new("/Cargo.lock"), "");

        let detector = CargoCrateDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_lockfile(Path::new("/Cargo.lock")));
        assert_eq!(detector.detect_language(Path::new("/Cargo.lock")), Some(toml_language()));
        assert_eq!(
            detector.qualify_path(Path::new("/Cargo.lock")),
            Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("/Cargo.lock")))
        );
    }

    #[test]
    fn it_should_detect_cargo_configuration() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(Path::new("/.cargo/config"), "");
        virtual_fs.add_file(Path::new("/.cargo/config.toml"), "");

        let detector = CargoCrateDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_cargo_config("/.cargo/config"));
        assert!(detector.is_cargo_config("/.cargo/config.toml"));

        assert_eq!(detector.detect_language(Path::new("/.cargo/config")), Some(toml_language()));
        assert_eq!(detector.detect_language(Path::new("/.cargo/config.toml")), Some(toml_language()));

        assert_eq!(detector.qualify_path(Path::new("/.cargo/config")), Some((PathContent::Configuration, Path::new("/.cargo/config"))));
        assert_eq!(detector.qualify_path(Path::new("/.cargo/config.toml")), Some((PathContent::Configuration, Path::new("/.cargo/config.toml"))));
    }

    #[test]
    fn it_should_qualify_rust_files() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(Path::new("/Cargo.toml"), "");
        virtual_fs.add_file(Path::new("/build.rs"), "");
        virtual_fs.add_file(Path::new("/src/lib.rs"), "");
        virtual_fs.add_file(Path::new("/tests/test.rs"), "");

        let detector = CargoCrateDetector::new(Rc::new(virtual_fs));

        assert_eq!(
            detector.qualify_path(Path::new("/build.rs")),
            Some((PathContent::Other("build".into(), &PathContent::Configuration), Path::new("/build.rs")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("/src/lib.rs")),
            Some((PathContent::Source, Path::new("/src")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("/tests/test.rs")),
            Some((PathContent::Test, Path::new("/tests")))
        );
    }

    #[test]
    fn it_should_load_cargo_crate() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(Path::new("/Cargo.toml"), indoc! {r#"
            [package]
            name = "test"
        "#});
        virtual_fs.add_file(Path::new("/src/lib.rs"), "");

        let detector = CargoCrateDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_crate("/"));

        assert_eq!(detector.load_crate_at("/").unwrap().unwrap().name(), Some("test"));

        assert_eq!(detector.load_crate_containing("/").unwrap().unwrap().name(), Some("test"));
        assert_eq!(detector.load_crate_containing("/src").unwrap().unwrap().name(), Some("test"));
        assert_eq!(detector.load_crate_containing("/src/lib.rs").unwrap().unwrap().name(), Some("test"));

        assert_eq!(detector.detect_unit(Path::new("/")).unwrap().name(), Some("test"));
    }
}
