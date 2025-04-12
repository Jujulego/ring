use ring_core_file::{DetectLanguage, FileContent, Language, QualifyPath};
use ring_module_json::json_language;
use ring_module_yaml::yaml_language;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use serde::Deserialize;
use tracing::{instrument, trace, warn};
use ring_core_units::{DetectUnit, Unit};
use crate::javascript_language;

/// Parsed content of package.json files
#[derive(Clone, Debug, Deserialize)]
pub struct PackageManifest {
    pub name: Option<String>,
    #[serde(default)]
    pub workspaces: Vec<String>,
}

/// Represents a npm package unit
#[derive(Clone, Debug)]
pub struct NpmPackage {
    manifest: PackageManifest,
    root: PathBuf,
}

impl NpmPackage {
    /// Returns loaded package.json manifest
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
}

impl Unit for NpmPackage {
    /// Returns the detected kind of unit.
    /// Either `"npm:package"` or `"npm:workspace"`
    fn kind(&self) -> &str {
        if self.manifest.workspaces.is_empty() {
            "npm:package"
        } else {
            "npm:workspace"
        }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn language(&self) -> Option<Language> {
        Some(javascript_language())
    }

    fn name(&self) -> Option<&str> {
        self.manifest.name.as_deref()
    }
}

/// Detector of npm packages, and related files (manifest and lockfiles)
#[derive(Clone, Debug, Default)]
pub struct NpmPackageDetector {}

impl NpmPackageDetector {
    /// Creates a new instance of NpmPackageDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a npm package manifest
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_manifest("assets/package.json"));
    /// ```
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_manifest(path)
    }

    fn _is_manifest(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("package.json")
    }

    /// Checks if given path is a package lockfile
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_lockfile("assets/package-lock.json"));
    /// assert!(detector.is_lockfile("assets/pnpm-lock.yaml"));
    /// assert!(detector.is_lockfile("assets/yarn.lock"));
    /// ```
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && (self._is_npm_lockfile(path) || self._is_pnpm_lockfile(path) || self._is_yarn_lockfile(path))
    }

    /// Checks if given path is a npm package lockfile
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_npm_lockfile("assets/package-lock.json"));
    /// ```
    pub fn is_npm_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_npm_lockfile(path)
    }

    fn _is_npm_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("package-lock.json")
    }

    /// Checks if given path is a npm package
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_package("assets"));
    /// ```
    pub fn is_package<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_dir() && self._is_package(path)
    }

    fn _is_package(&self, path: &Path) -> bool {
        self.is_manifest(path.join("package.json"))
    }

    /// Checks if given path is a pnpm package lockfile
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_pnpm_lockfile("assets/pnpm-lock.yaml"));
    /// ```
    pub fn is_pnpm_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_pnpm_lockfile(path)
    }

    fn _is_pnpm_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("pnpm-lock.yaml")
    }

    /// Checks if given path is a yarn package lockfile
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_yarn_lockfile("assets/yarn.lock"));
    /// ```
    pub fn is_yarn_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_yarn_lockfile(path)
    }

    fn _is_yarn_lockfile(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some("yarn.lock")
    }

    /// Checks if given path is a yarn configuration file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::NpmPackageDetector;
    ///
    /// let detector = NpmPackageDetector::new();
    /// assert!(detector.is_yarn_configuration("assets/.yarnrc.yml"));
    /// ```
    pub fn is_yarn_configuration<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();

        trace!("stat {}", path.display());
        path.is_file() && self._is_yarn_configuration(path)
    }

    fn _is_yarn_configuration(&self, path: &Path) -> bool {
        path.file_name().and_then(OsStr::to_str) == Some(".yarnrc.yml")
    }
}

impl DetectLanguage for NpmPackageDetector {
    #[instrument(name = "npm-package.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        trace!("stat {}", path.display());
        if path.is_file() {
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
        let manifest_path = path.join("package.json");
        
        trace!("read file {}", manifest_path.display());
        match File::open(&manifest_path) {
            Ok(ref mut file) => {
                match serde_json::from_reader(&mut *file) {
                    Ok(manifest) => {
                        Some(Rc::new(NpmPackage { manifest, root: path.to_path_buf() }))
                    }
                    Err(err) => {
                        warn!("Failed to parse {}: {}", manifest_path.display(), err);
                        None
                    }
                }
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => None,
            Err(err) => {
                warn!("Unable to access {}: {err}", manifest_path.display());
                None
            }
        }
    }
}

impl QualifyPath for NpmPackageDetector {
    #[instrument(name = "npm-package.qualify-path", skip_all)]
    fn qualify_file<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
        // Out of package cases
        trace!("stat {}", path.display());
        if path.is_file() {
            if self._is_manifest(path) {
                return Some((FileContent::Manifest, path));
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

            trace!("stat {}", ancestor.display());
            if ancestor.is_file() {
                if self._is_yarn_configuration(ancestor) {
                    return Some((FileContent::Configuration, ancestor));
                }

                if self._is_npm_lockfile(ancestor) || self._is_pnpm_lockfile(ancestor) || self._is_yarn_lockfile(ancestor) {
                    return Some((FileContent::Lockfile, ancestor));
                }

                match ancestor.file_name().and_then(OsStr::to_str) {
                    Some(".pnp.cjs") => return Some((FileContent::Other("pnp commonjs".into()), ancestor)),
                    Some(".pnp.loader.mjs") => return Some((FileContent::Other("pnp esm".into()), ancestor)),
                    _ => {}
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
    fn it_should_detect_manifest_language() {
        let detector = NpmPackageDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/package.json")), Some(json_language()));
    }

    #[test]
    fn it_should_detect_lockfile_language() {
        let detector = NpmPackageDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/package-lock.json")), Some(json_language()));
        assert_eq!(detector.detect_language(Path::new("assets/pnpm-lock.yaml")), Some(yaml_language()));
        assert_eq!(detector.detect_language(Path::new("assets/yarn.lock")), Some(yaml_language()));
    }

    #[test]
    fn it_should_qualify_npm_files() {
        let detector = NpmPackageDetector::new();

        assert_eq!(detector.qualify_file(Path::new("assets/package.json")), Some((FileContent::Manifest, Path::new("assets/package.json"))));
        assert_eq!(detector.qualify_file(Path::new("assets/package-lock.json")), Some((FileContent::Lockfile, Path::new("assets/package-lock.json"))));
    }

    #[test]
    fn it_should_qualify_pnpm_files() {
        let detector = NpmPackageDetector::new();

        assert_eq!(detector.qualify_file(Path::new("assets/pnpm-lock.yaml")), Some((FileContent::Lockfile, Path::new("assets/pnpm-lock.yaml"))));
    }

    #[test]
    fn it_should_qualify_yarn_files() {
        let detector = NpmPackageDetector::new();

        assert_eq!(detector.qualify_file(Path::new("assets/.pnp.cjs")), Some((FileContent::Other("pnp commonjs".into()), Path::new("assets/.pnp.cjs"))));
        assert_eq!(detector.qualify_file(Path::new("assets/.pnp.loader.mjs")), Some((FileContent::Other("pnp esm".into()), Path::new("assets/.pnp.loader.mjs"))));
        assert_eq!(detector.qualify_file(Path::new("assets/.yarnrc.yml")), Some((FileContent::Configuration, Path::new("assets/.yarnrc.yml"))));
        assert_eq!(detector.qualify_file(Path::new("assets/yarn.lock")), Some((FileContent::Lockfile, Path::new("assets/yarn.lock"))));
    }
}