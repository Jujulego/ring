use crate::SourceFile;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{instrument, trace};
use ring_core::{CodeUnit, CodeUnitDetector};

/// Detector for script files
#[derive(Clone, Debug, Default)]
pub struct SourceFileDetector {
    sources: RefCell<HashMap<PathBuf, Option<Rc<SourceFile>>>>,
}

impl SourceFileDetector {
    pub fn new() -> SourceFileDetector {
        Default::default()
    }

    pub fn _is_source_file(&self, path: &Path) -> bool {
        trace!("touch {}", path.display());
        path.is_file() && path.extension().and_then(OsStr::to_str) == Some("rs")
    }

    #[instrument(name = "rust.detect_source", skip(self, path))]
    pub fn detect_source(&self, path: &Path) -> Option<Rc<SourceFile>> {
        if let Some(source_file) = self.sources.borrow().get(path) {
            return source_file.clone();
        }

        let path = path.to_path_buf();

        let result = if self._is_source_file(&path) {
            Some(Rc::new(SourceFile::new(path.clone())))
        } else {
            None
        };

        self.sources.borrow_mut().insert(path, None);

        result
    }
}

impl CodeUnitDetector for SourceFileDetector {
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        Ok(self.detect_source(path)
            .map(|source| source as Rc<dyn CodeUnit>))
    }
}