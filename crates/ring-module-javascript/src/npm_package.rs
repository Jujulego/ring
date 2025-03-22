use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use ring_module_json::json_language;
use ring_module_yaml::yaml_language;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{instrument, trace};

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
            if self._is_manifest(path) || self.is_npm_lockfile(path) {
                return Some(json_language());
            }

            if self._is_pnpm_lockfile(path) || self.is_yarn_lockfile(path) {
                return Some(yaml_language());
            }
        }

        None
    }
}

impl QualifyPath for NpmPackageDetector {
    #[instrument(name = "npm-package.qualify-path", skip_all)]
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)> {
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