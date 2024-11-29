use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use tracing::{instrument, trace};
use ring_code_unit::{CodeUnit, CodeUnitDetector};
use crate::{Package, ScriptFile, WebLanguage};

////////////////////////////////////////////////////////////////////////////////
// Web Unit Detector
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub struct WebUnitDetector {}

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

    pub fn _detect_package(&self, path: &Path) -> anyhow::Result<Option<WebLanguage>> {
        if !path.is_dir() {
            return Ok(None);
        }

        let manifest_path = path.join("package.json");
        trace!("touch file {}", manifest_path.display());
        
        if manifest_path.try_exists()? {
            Ok(Some(WebLanguage::JavaScript))
        } else {
            Ok(None)
        }
    }

    #[instrument(name = "web.detect_script", skip(self, path))]
    pub fn detect_script<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<ScriptFile>> {
        let path = path.as_ref();

        let script = self._detect_script_language(path)?
            .map(|language| ScriptFile::new(path.to_path_buf(), language));

        Ok(script)
    }

    #[instrument(name = "web.detect_package", skip(self, path))]
    pub fn detect_package<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Package>> {
        let path = path.as_ref();

        let package = self._detect_package(path)?
            .map(|language| Package::new(path.to_path_buf(), language));

        Ok(package)
    }
}

impl CodeUnitDetector for WebUnitDetector {
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        let script = self.detect_script(path)?
            .map(|cu| Rc::new(cu) as Rc<dyn CodeUnit>);

        let package = self.detect_package(path)?
            .map(|cu| Rc::new(cu) as Rc<dyn CodeUnit>);

        Ok(script.or(package))
    }
}