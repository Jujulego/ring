use crate::packages::package_manager::PACKAGE_MANAGERS;
use crate::{Package, PackageManager, PackageManifest};
use anyhow::anyhow;
use ring_core::{CodeUnit, CodeUnitDetector};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{info, instrument, trace};

/// Detector for packages
#[derive(Clone, Debug, Default)]
pub struct PackageDetector {
    packages: RefCell<HashMap<PathBuf, Option<Rc<Package>>>>,
}

impl PackageDetector {
    pub fn new() -> PackageDetector {
        Default::default()
    }

    /// Detect package manager in given directory (based on lockfile)
    pub fn detect_package_manager(&self, path: &Path) -> anyhow::Result<Option<PackageManager>> {
        for package_manager in PACKAGE_MANAGERS {
            let lockfile_path = path.join(package_manager.lockfile());

            trace!("touch {}", lockfile_path.display());
            if lockfile_path.try_exists()? {
                return Ok(Some(package_manager));
            }
        }

        Ok(None)
    }

    /// Load manifest within given path (if it exists)
    pub fn load_manifest(&self, path: &Path) -> anyhow::Result<Option<PackageManifest>> {
        let manifest_path = path.join("package.json");

        trace!("read file {}", manifest_path.display());
        match File::open(&manifest_path) {
            Ok(ref mut file) => Some(PackageManifest::from_reader(file)).transpose(),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => {
                Err(anyhow!(err).context(format!("Unable to access {}", manifest_path.display())))
            }
        }
    }

    fn _detect_package(&self, path: &Path) -> anyhow::Result<Option<Package>> {
        trace!("touch {}", path.display());

        match path.metadata() {
            Ok(metadata) if metadata.is_dir() => {
                if let Some(manifest) = self.load_manifest(path)? {
                    let mut package = Package::new(path.to_path_buf(), manifest);

                    if let Some(manager) = self.detect_package_manager(path)? {
                        *package.package_manager_mut() = Some(manager);
                    }

                    Ok(Some(package))
                } else {
                    Ok(None)
                }
            },
            Ok(_) => Ok(None),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => {
                Err(anyhow!(err).context(format!("Unable to access {}", path.display())))
            }
        }
    }

    /// Detect if there is a package at given path
    #[instrument(name = "web.detect-package", skip(self, path))]
    pub fn detect_package<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<Package>>> {
        let path = path.as_ref();

        if let Some(package) = self.packages.borrow().get(path) {
            return Ok(package.clone())
        }

        let package = self._detect_package(path)?
            .map(Rc::new);

        if let Some(package) = &package {
            if let Some(name) = package.name() {
                info!("recognized {} as a web package '{}'", path.display(), name);
            } else {
                info!("recognized {} as a web package", path.display());
            }

            self.packages.borrow_mut().insert(path.to_path_buf(), Some(package.clone()));
        } else {
            self.packages.borrow_mut().insert(path.to_path_buf(), None);
        }

        Ok(package)
    }

    /// Detect if given path is within a package, and return it
    pub fn search_package(&self, mut path: &Path) -> anyhow::Result<Option<Rc<Package>>> {
        loop {
            if let Some(package) = self.detect_package(path)? {
                break Ok(Some(package));
            }

            if let Some(parent) = path.parent() {
                path = parent;
            } else {
                break Ok(None);
            }
        }
    }
}

impl CodeUnitDetector for PackageDetector {
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        self.detect_package(path)
            .map(|opt| opt.map(|pkg| pkg as Rc<dyn CodeUnit>))
    }
}