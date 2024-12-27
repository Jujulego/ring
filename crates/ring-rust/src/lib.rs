mod language;
mod crates;

pub use crate::crates::{CargoManifest, SourceFile, SourceFileDetector};
use ring_core::{CodeUnitDetector, ProcessUnitDetector, RingModule};
use std::rc::Rc;

pub struct RustModule {
    pub source_file_detector: Rc<SourceFileDetector>,
}

impl RustModule {
    pub fn new() -> Self {
        RustModule {
            source_file_detector: Rc::new(SourceFileDetector::new()),
        }
    }
}

impl Default for RustModule {
    fn default() -> Self {
        RustModule::new()
    }
}

impl RingModule for RustModule {
    fn code_unit_detector(&self) -> Vec<Rc<dyn CodeUnitDetector>> {
        vec![self.source_file_detector.clone()]
    }

    fn process_unit_detector(&self) -> Vec<Rc<dyn ProcessUnitDetector>> {
        vec![]
    }
}