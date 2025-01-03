use crate::packages::PackageDetector;
use crate::{ScriptFile, WebLanguage};
use ring_core::{CodeUnit, CodeUnitDetector};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{info, instrument, trace};

/// Detector for script files
#[derive(Clone, Debug)]
pub struct ScriptFileDetector {
    package_detector: Rc<PackageDetector>,
    scripts: RefCell<HashMap<PathBuf, Option<Rc<ScriptFile>>>>,
}

impl ScriptFileDetector {
    pub fn new(package_detector: Rc<PackageDetector>) -> ScriptFileDetector {
        ScriptFileDetector {
            package_detector,
            scripts: RefCell::new(HashMap::new()),
        }
    }

    pub fn detect_script_language(&self, path: &Path) -> anyhow::Result<Option<WebLanguage>> {
        trace!("touch {}", path.display());
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
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let shebang = reader.lines()
                .find(|res| res.as_ref().map_or(true, |line| line.starts_with("#!")))
                .transpose()?;

            if shebang.is_some_and(|t| t == "#!/usr/bin/env node") {
                return Ok(Some(WebLanguage::JavaScript));
            }
        }

        Ok(None)
    }

    fn _detect_script(&self, path: &Path) -> anyhow::Result<Option<Rc<ScriptFile>>> {
        if let Some(script) = self.scripts.borrow().get(path) {
            return Ok(script.clone());
        }

        let script = self.detect_script_language(path)?
            .map(|language| ScriptFile::new(path.to_path_buf(), language));

        if let Some(mut script) = script {
            let package = path.parent()
                .and_then(|parent| self.package_detector.search_package(parent).transpose())
                .transpose()?;

            if let Some(package) = package {
                *script.package_mut() = Some(package);
            }

            info!("recognized {} as a web script", path.display());
            let script = Rc::new(script);
            self.scripts.borrow_mut().insert(path.to_path_buf(), Some(script.clone()));

            Ok(Some(script))
        } else {
            self.scripts.borrow_mut().insert(path.to_path_buf(), None);

            Ok(None)
        }
    }

    #[instrument(name = "web.detect-script", skip(self, path))]
    pub fn detect_script<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<ScriptFile>>> {
        let path = path.as_ref();

        if let Some(script) = self._detect_script(path)? {
            Ok(Some(script))
        } else {
            for ext in [".js", ".cjs", ".mjs"] {
                let mut extended = path.as_os_str().to_os_string();
                extended.push(ext);

                if let Some(script) = self._detect_script(Path::new(&extended))? {
                    return Ok(Some(script));
                }
            }

            Ok(None)
        }
    }
}

impl CodeUnitDetector for ScriptFileDetector {
    fn name(&self) -> &str {
        "web:script"
    }
    
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        self.detect_script(path).map(|opt| opt.map(|pkg| pkg as Rc<dyn CodeUnit>))
    }
}