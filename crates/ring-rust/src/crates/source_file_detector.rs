use crate::{CargoCrateDetector, SourceFile};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{info, instrument, trace};
use ring_core::{CodeUnit, CodeUnitDetector};

/// Detector for script files
#[derive(Clone, Debug)]
pub struct SourceFileDetector {
    crate_detector: Rc<CargoCrateDetector>,
    sources: RefCell<HashMap<PathBuf, Option<Rc<SourceFile>>>>,
}

impl SourceFileDetector {
    pub fn new(crate_detector: Rc<CargoCrateDetector>) -> SourceFileDetector {
        SourceFileDetector {
            crate_detector,
            sources: RefCell::new(HashMap::new()),
        }
    }

    pub fn _is_source_file(&self, path: &Path) -> bool {
        trace!("touch {}", path.display());
        path.is_file() && path.extension().and_then(OsStr::to_str) == Some("rs")
    }

    #[instrument(name = "rust.detect_source", skip(self, path))]
    pub fn detect_source(&self, path: &Path) -> anyhow::Result<Option<Rc<SourceFile>>> {
        if let Some(source_file) = self.sources.borrow().get(path) {
            return Ok(source_file.clone());
        }

        let path = path.to_path_buf();

        let result = if self._is_source_file(&path) {
            let mut source = SourceFile::new(path.clone());
            
            *source.cargo_crate_mut() = path.parent()
                .and_then(|parent| self.crate_detector.search_crate(parent).transpose())
                .transpose()?;
            
            info!("recognized {} as a rust source file", path.display());
            Some(Rc::new(source))
        } else {
            None
        };

        self.sources.borrow_mut().insert(path, None);

        Ok(result)
    }
}

impl CodeUnitDetector for SourceFileDetector {
    fn name(&self) -> &str {
        "rust:source-file"
    }
    
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        self.detect_source(path)
            .map(|opt| opt.map(|src| src as Rc<dyn CodeUnit>))
    }
}