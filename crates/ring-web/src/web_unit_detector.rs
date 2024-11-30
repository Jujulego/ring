use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{instrument, trace};
use ring_code_unit::{CodeUnit, CodeUnitDetector};
use crate::{Package, ScriptFile, WebLanguage};

////////////////////////////////////////////////////////////////////////////////
// Web Unit Detector
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default)]
pub struct WebUnitDetector {
    packages: RefCell<HashMap<PathBuf, Rc<Package>>>
}

impl WebUnitDetector {
    fn _detect_script_language(&self, path: &Path) -> anyhow::Result<Option<WebLanguage>> {
        if !path.is_file() {
            return Ok(None);
        }

        // Extension based
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            match ext {
                "js" | "jsx" | "cjs" | "mjs" => {
                    return Ok(Some(WebLanguage::JavaScript));
                },
                "ts" | "tsx" | "cts" | "mts" => {
                    return Ok(Some(WebLanguage::TypeScript));
                },
                _ => {}
            }
        }

        // Search shebang header
        if path.extension().is_none() {
            trace!("read file {}", path.display());
            let file = fs::File::open(path)?;
            let reader = BufReader::new(file);
            let shebang = reader.lines()
                .find(|res| res.as_ref().map_or(true, |line| line.starts_with("#!")))
                .transpose()?;
    
            if shebang.map_or(false, |t| t == "#!/usr/bin/env node") {
                return Ok(Some(WebLanguage::JavaScript));
            }
        }

        Ok(None)
    }

    pub fn _detect_package(&self, path: &Path) -> anyhow::Result<Option<Package>> {
        if !path.is_dir() {
            return Ok(None);
        }

        let manifest_path = path.join("package.json");

        trace!("touch file {}", manifest_path.display());
        if manifest_path.try_exists()? {
            Ok(Some(Package::new(path.to_path_buf())))
        } else {
            Ok(None)
        }
    }

    #[instrument(name = "web.detect-script", skip(self, path))]
    pub fn detect_script<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<ScriptFile>>> {
        let path = path.as_ref();

        let script = self._detect_script_language(path)?
            .map(|language| ScriptFile::new(path.to_path_buf(), language));
        
        if let Some(mut script) = script {
            let package = path.parent()
                .and_then(|parent| self.search_package(parent).transpose())
                .transpose()?;
            
            if let Some(package) = package {
                script.set_package(package)
            }

            Ok(Some(Rc::new(script)))
        } else {
            Ok(None)
        }
    }

    #[instrument(name = "web.detect-package", skip(self, path))]
    pub fn detect_package<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<Package>>> {
        let path = path.as_ref();

        if let Some(package) = self.packages.borrow().get(path) {
            return Ok(Some(package.clone()))
        }
        
        let package = self._detect_package(path)?
            .map(Rc::new);
        
        if let Some(package) = &package {
            self.packages.borrow_mut().insert(path.to_path_buf(), package.clone());
        }

        Ok(package)
    }
    
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

impl CodeUnitDetector for WebUnitDetector {
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        let script = self.detect_script(path)?
            .map(|cu| cu as Rc<dyn CodeUnit>);

        let package = self.detect_package(path)?
            .map(|cu| cu as Rc<dyn CodeUnit>);

        Ok(script.or(package))
    }
}