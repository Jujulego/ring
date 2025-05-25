use crate::NpmPackage;
use anyhow::anyhow;
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::PathAdaptator;
use ring_core_units::{DetectUnit, Unit};
use ring_module_json::json_language;
use ring_module_yaml::yaml_language;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, instrument, trace, warn};

/// Detector for npm packages, and related files
#[derive(Clone,)]
pub struct NpmPackageDetector {
    cache: RefCell<HashMap<PathBuf, Option<Rc<NpmPackage>>>>,
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl NpmPackageDetector {
    /// Creates a new instance of NpmPackageDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        NpmPackageDetector {
            cache: RefCell::new(HashMap::new()),
            path_adaptator,
        }
    }

    /// Checks if given path is a npm package manifest
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.path_adaptator.is_file(path).unwrap_or(false)
            && self._is_manifest(path)
    }

    fn _is_manifest(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("package.json")
    }

    /// Checks if given path is a package lockfile
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.path_adaptator.is_file(path).unwrap_or(false)
            && (self._is_npm_lockfile(path) || self._is_pnpm_lockfile(path) || self._is_yarn_lockfile(path))
    }

    fn _is_npm_configuration(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some(".npmrc")
    }

    /// Checks if given path is a npm package lockfile
    pub fn is_npm_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.path_adaptator.is_file(path).unwrap_or(false)
            && self._is_npm_lockfile(path)
    }

    fn _is_npm_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("package-lock.json")
    }

    /// Checks if given path is a npm package
    pub fn is_package<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_dir() && self._is_package(path)
    }

    fn _is_package(&self, path: &Path) -> bool {
        self.is_manifest(path.join("package.json"))
    }

    /// Checks if given path is a pnpm package lockfile
    pub fn is_pnpm_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.path_adaptator.is_file(path).unwrap_or(false)
            && self._is_pnpm_lockfile(path)
    }

    fn _is_pnpm_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("pnpm-lock.yaml")
    }

    /// Checks if given path is a yarn package lockfile
    pub fn is_yarn_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.path_adaptator.is_file(path).unwrap_or(false)
            && self._is_yarn_lockfile(path)
    }

    fn _is_yarn_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("yarn.lock")
    }

    /// Checks if given path is a yarn configuration file
    pub fn is_yarn_configuration<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.path_adaptator.is_file(path).unwrap_or(false)
            && self._is_yarn_configuration(path)
    }

    fn _is_yarn_configuration(&self, path: &Path) -> bool {
        matches!(path.file_name().and_then(OsStr::to_str), Some(".yarnrc") | Some(".yarnrc.yml"))
    }

    /// Load npm package at given path, if any.
    pub fn load_package_at<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<NpmPackage>>> {
        self._load_package_at(path.as_ref())
    }

    /// Load npm package containing given path.
    pub fn load_package_containing<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<NpmPackage>>> {
        let mut path = path.as_ref();

        if self.path_adaptator.is_file(path).unwrap_or(false) {
            path = path.parent().unwrap();
        }

        path.ancestors()
            .find_map(|ancestor| self._load_package_at(ancestor).transpose())
            .transpose()
    }

    fn _load_package_at(&self, path: &Path) -> anyhow::Result<Option<Rc<NpmPackage>>> {
        let manifest_path = path.join("package.json");

        if let Some(crt) = self.cache.borrow().get(&manifest_path) {
            debug!(key = %manifest_path.display(), "npm package cache hit");
            return Ok(crt.clone());
        }

        trace!("read file {}", manifest_path.display());
        match File::open(&manifest_path) {
            Ok(ref mut file) => {
                match serde_json::from_reader(&mut *file) {
                    Ok(manifest) => {
                        let pkg = Some(Rc::new(NpmPackage::new(manifest, path.to_path_buf())));

                        debug!(key = %manifest_path.display(), "npm package cached");
                        self.cache.borrow_mut().insert(manifest_path, pkg.clone());

                        Ok(pkg)
                    }
                    Err(err) => {
                        Err(anyhow!(err).context(format!("Failed to parse {}", manifest_path.display())))
                    }
                }
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                debug!(key = %manifest_path.display(), "npm package miss cached");
                self.cache.borrow_mut().insert(manifest_path, None);

                Ok(None)
            },
            Err(err) => {
                Err(anyhow!(err).context(format!("Unable to access {}", manifest_path.display())))
            }
        }
    }
}

impl DetectLanguage for NpmPackageDetector {
    #[instrument(name = "npm-package.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self.path_adaptator.is_file(path).unwrap_or(false)  {
            if self._is_manifest(path) || self._is_npm_lockfile(path) {
                return Some(json_language());
            }

            if self._is_pnpm_lockfile(path) || self._is_yarn_lockfile(path) {
                return Some(yaml_language());
            }
        }

        None
    }
}

impl DetectUnit for NpmPackageDetector {
    #[instrument(name = "npm-package.detect-unit", skip_all)]
    fn detect_unit(&self, path: &Path) -> Option<Rc<dyn Unit>> {
        match self._load_package_at(path) {
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

impl QualifyPath for NpmPackageDetector {
    #[instrument(name = "npm-package.qualify-path", skip_all)]
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        // Out of package cases
        if self.path_adaptator.is_file(path).unwrap_or(false)  {
            if self._is_manifest(path) {
                return Some((PathContent::Manifest, path));
            }

            if self._is_npm_configuration(path) || self._is_yarn_configuration(path) {
                return Some((PathContent::Configuration, path));
            }
        } else {
            return None;
        }

        // In package cases
        for ancestor in path.ancestors() {
            if let Some(parent) = ancestor.parent() {
                if !self._is_package(parent) {
                    continue;
                }
            } else {
                break;
            }

            if self.path_adaptator.is_file(ancestor).unwrap_or(false)  {
                if self._is_npm_lockfile(ancestor) || self._is_pnpm_lockfile(ancestor) || self._is_yarn_lockfile(ancestor) {
                    return Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), ancestor));
                }

                match ancestor.file_name().and_then(OsStr::to_str) {
                    Some(".pnp.cjs") => return Some((PathContent::Other("pnp-cjs".into(), &PathContent::Dependency), ancestor)),
                    Some(".pnp.loader.mjs") => return Some((PathContent::Other("pnp-esm".into(), &PathContent::Dependency), ancestor)),
                    _ => {}
                }
            } else if ancestor.file_name().and_then(OsStr::to_str) == Some("node_modules") {
                return Some((PathContent::Dependency, ancestor));
            }
        }

        None
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
            fn is_file(&self, path: &Path) -> anyhow::Result<bool>;
        }
    }

    #[test]
    fn it_should_detect_manifest_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = NpmPackageDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/package.json")), Some(json_language()));
    }

    #[test]
    fn it_should_detect_lockfile_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = NpmPackageDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/package-lock.json")), Some(json_language()));
        assert_eq!(detector.detect_language(Path::new("assets/pnpm-lock.yaml")), Some(yaml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/yarn.lock")), Some(yaml_language()));
    }

    #[test]
    fn it_should_detect_package_unit() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = NpmPackageDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_unit(Path::new("assets")).unwrap().name(), Some("test-assets"));
    }

    #[test]
    fn it_should_qualify_npm_files() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = NpmPackageDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.qualify_path(Path::new("assets/package.json")), Some((PathContent::Manifest, Path::new("assets/package.json"))));
        assert_eq!(detector.qualify_path(Path::new("assets/package-lock.json")), Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("assets/package-lock.json"))));
    }

    #[test]
    fn it_should_qualify_pnpm_files() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = NpmPackageDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.qualify_path(Path::new("assets/pnpm-lock.yaml")), Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("assets/pnpm-lock.yaml"))));
    }

    #[test]
    fn it_should_qualify_yarn_files() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .returning(|_| Ok(true));

        let detector = NpmPackageDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.qualify_path(Path::new("assets/.pnp.cjs")), Some((PathContent::Other("pnp-cjs".into(), &PathContent::Dependency), Path::new("assets/.pnp.cjs"))));
        assert_eq!(detector.qualify_path(Path::new("assets/.pnp.loader.mjs")), Some((PathContent::Other("pnp-esm".into(), &PathContent::Dependency), Path::new("assets/.pnp.loader.mjs"))));
        assert_eq!(detector.qualify_path(Path::new("assets/.yarnrc.yml")), Some((PathContent::Configuration, Path::new("assets/.yarnrc.yml"))));
        assert_eq!(detector.qualify_path(Path::new("assets/yarn.lock")), Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("assets/yarn.lock"))));
    }
}