use crate::NpmPackage;
use anyhow::{anyhow, Context};
use ring_core_content::{DetectLanguage, Language, PathContent, QualifyPath};
use ring_core_fs::traits::AbstractFilesystem;
use ring_core_fs::Error;
use ring_core_units::{DetectUnit, Unit};
use ring_module_json::json_language;
use ring_module_yaml::yaml_language;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, instrument, warn};

/// Detector for npm packages, and related files
#[derive(Clone)]
pub struct NpmPackageDetector {
    cache: RefCell<HashMap<PathBuf, Option<Rc<NpmPackage>>>>,
    filesystem: Rc<dyn AbstractFilesystem>,
}

impl NpmPackageDetector {
    /// Creates a new instance of NpmPackageDetector
    #[inline]
    pub fn new(filesystem: Rc<dyn AbstractFilesystem>) -> Self {
        NpmPackageDetector {
            cache: RefCell::new(HashMap::new()),
            filesystem,
        }
    }

    /// Checks if given path is a npm package manifest
    #[inline]
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_file(path) && self._is_manifest(path)
    }

    #[inline]
    fn _is_manifest(&self, path: &Path) -> bool {
        path.file_name() == Some(OsStr::new("package.json"))
    }

    /// Checks if given path is a package lockfile
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_file(path)
            && (self._is_npm_lockfile(path) || self._is_pnpm_lockfile(path) || self._is_yarn_lockfile(path))
    }

    /// Checks if given path is a npm config file
    #[inline]
    pub fn is_npm_configuration<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_file(path) && self._is_npm_configuration(path)
    }

    #[inline]
    fn _is_npm_configuration(&self, path: &Path) -> bool {
        path.file_name() == Some(OsStr::new(".npmrc"))
    }

    /// Checks if given path is a npm package lockfile
    #[inline]
    pub fn is_npm_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        path.parent().is_some_and(|parent| self.is_package(parent))
            && self.filesystem.is_file(path)
            && self._is_npm_lockfile(path)
    }

    #[inline]
    fn _is_npm_lockfile(&self, path: &Path) -> bool {
        path.file_name() == Some(OsStr::new("package-lock.json"))
    }

    /// Checks if given path is a npm package
    #[inline]
    pub fn is_package<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_dir(path) && self._is_package(path)
    }

    #[inline]
    fn _is_package(&self, path: &Path) -> bool {
        self.is_manifest(path.join("package.json"))
    }

    /// Checks if given path is a pnpm package lockfile
    #[inline]
    pub fn is_pnpm_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        path.parent().is_some_and(|parent| self.is_package(parent))
            && self.filesystem.is_file(path)
            && self._is_pnpm_lockfile(path)
    }

    #[inline]
    fn _is_pnpm_lockfile(&self, path: &Path) -> bool {
        path.file_name() == Some(OsStr::new("pnpm-lock.yaml"))
    }

    /// Checks if given path is a yarn package lockfile
    #[inline]
    pub fn is_yarn_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        path.parent().is_some_and(|parent| self.is_package(parent))
            && self.filesystem.is_file(path)
            && self._is_yarn_lockfile(path)
    }

    #[inline]
    fn _is_yarn_lockfile(&self, path: &Path) -> bool {
        path.file_name() == Some(OsStr::new("yarn.lock"))
    }

    /// Checks if given path is a yarn configuration file
    #[inline]
    pub fn is_yarn_configuration<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        self.filesystem.is_file(path) && self._is_yarn_configuration(path)
    }

    #[inline]
    fn _is_yarn_configuration(&self, path: &Path) -> bool {
        matches!(path.file_name().and_then(OsStr::to_str), Some(".yarnrc") | Some(".yarnrc.yml"))
    }

    /// Load npm package at given path, if any.
    #[inline]
    pub fn load_package_at<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<NpmPackage>>> {
        self._load_package_at(path.as_ref())
    }

    /// Load npm package containing given path.
    pub fn load_package_containing<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<NpmPackage>>> {
        let mut path = path.as_ref();

        if self.filesystem.is_file(path) {
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

        match self.filesystem.open(&manifest_path) {
            Ok(mut file) => {
                let manifest = serde_json::from_reader(file.as_reader())
                    .context(format!("Failed to parse {}", manifest_path.display()))?;

                let pkg = Some(Rc::new(NpmPackage::new(manifest, path.to_path_buf())));

                debug!(key = %manifest_path.display(), "npm package cached");
                self.cache.borrow_mut().insert(manifest_path, pkg.clone());

                Ok(pkg)
            },
            Err(Error::NotFound(_)) => {
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
        if self.filesystem.is_file(path)  {
            if self._is_manifest(path) || self._is_npm_lockfile(path) {
                return Some(json_language());
            }

            if self._is_pnpm_lockfile(path) || self._is_yarn_configuration(path) || self._is_yarn_lockfile(path) {
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
        if self.filesystem.is_file(path)  {
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

            if self.filesystem.is_file(ancestor)  {
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
    use ring_core_fs::filesystem::VirtualFilesystem;

    #[test]
    fn it_should_detect_package_json() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("package.json", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_manifest(Path::new("package.json")));
        assert_eq!(detector.detect_language(Path::new("package.json")), Some(json_language()));
        assert_eq!(detector.qualify_path(Path::new("package.json")), Some((PathContent::Manifest, Path::new("package.json"))));
    }

    #[test]
    fn it_should_detect_npm_configuration() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(".npmrc", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_npm_configuration(Path::new(".npmrc")));
        assert_eq!(detector.detect_language(Path::new(".npmrc")), None);
        assert_eq!(
            detector.qualify_path(Path::new(".npmrc")),
            Some((PathContent::Configuration, Path::new(".npmrc")))
        );
    }

    #[test]
    fn it_should_detect_npm_lockfile() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("/package.json", "{}");
        virtual_fs.add_file("/package-lock.json", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_lockfile(Path::new("/package-lock.json")));
        assert!(detector.is_npm_lockfile(Path::new("/package-lock.json")));
        assert_eq!(detector.detect_language(Path::new("/package-lock.json")), Some(json_language()));
        assert_eq!(
            detector.qualify_path(Path::new("/package-lock.json")),
            Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("/package-lock.json")))
        );
    }

    #[test]
    fn it_should_detect_pnpm_lockfile() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("/package.json", "{}");
        virtual_fs.add_file("/pnpm-lock.yaml", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_lockfile(Path::new("/pnpm-lock.yaml")));
        assert!(detector.is_pnpm_lockfile(Path::new("/pnpm-lock.yaml")));
        assert_eq!(detector.detect_language(Path::new("/pnpm-lock.yaml")), Some(yaml_language()));
        assert_eq!(
            detector.qualify_path(Path::new("/pnpm-lock.yaml")),
            Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("/pnpm-lock.yaml")))
        );
    }

    #[test]
    fn it_should_detect_yarn_configuration() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file(".yarnrc.yml", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_yarn_configuration(Path::new(".yarnrc.yml")));
        assert_eq!(detector.detect_language(Path::new(".yarnrc.yml")), Some(yaml_language()));
        assert_eq!(
            detector.qualify_path(Path::new(".yarnrc.yml")),
            Some((PathContent::Configuration, Path::new(".yarnrc.yml")))
        );
    }

    #[test]
    fn it_should_detect_yarn_lockfile() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("/package.json", "{}");
        virtual_fs.add_file("/yarn.lock", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_lockfile(Path::new("/yarn.lock")));
        assert!(detector.is_yarn_lockfile(Path::new("/yarn.lock")));
        assert_eq!(detector.detect_language(Path::new("/yarn.lock")), Some(yaml_language()));
        assert_eq!(
            detector.qualify_path(Path::new("/yarn.lock")),
            Some((PathContent::Other("lockfile".to_string(), &PathContent::Dependency), Path::new("/yarn.lock")))
        );
    }

    #[test]
    fn it_should_load_package_unit() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("/package.json", r#"{ "name": "test" }"#);
        virtual_fs.add_file("/src/main.js", "");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_package(Path::new("/")));

        assert_eq!(detector.load_package_at(Path::new("/")).unwrap().unwrap().name(), Some("test"));

        assert_eq!(detector.load_package_containing(Path::new("/")).unwrap().unwrap().name(), Some("test"));
        assert_eq!(detector.load_package_containing(Path::new("/src")).unwrap().unwrap().name(), Some("test"));
        assert_eq!(detector.load_package_containing(Path::new("/src/main.js")).unwrap().unwrap().name(), Some("test"));

        assert_eq!(detector.detect_unit(Path::new("/")).unwrap().name(), Some("test"));
    }

    #[test]
    fn it_should_qualify_yarn_pnp_files() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("/package.json", "{}");
        virtual_fs.add_file("/.pnp.cjs", "{}");
        virtual_fs.add_file("/.pnp.loader.mjs", "{}");

        let detector = NpmPackageDetector::new(Rc::new(virtual_fs));

        assert_eq!(
            detector.qualify_path(Path::new("/.pnp.cjs")),
            Some((PathContent::Other("pnp-cjs".into(), &PathContent::Dependency), Path::new("/.pnp.cjs")))
        );
        assert_eq!(
            detector.qualify_path(Path::new("/.pnp.loader.mjs")),
            Some((PathContent::Other("pnp-esm".into(), &PathContent::Dependency), Path::new("/.pnp.loader.mjs")))
        );
    }
}