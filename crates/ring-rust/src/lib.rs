mod language;

use std::rc::Rc;
use ring_core::{CodeUnitDetector, ProcessUnitDetector, RingModule};

pub struct RustModule {}

impl RustModule {
    pub fn new() -> Self {
        RustModule {}
    }
}

impl Default for RustModule {
    fn default() -> Self {
        RustModule::new()
    }
}

impl RingModule for RustModule {
    fn code_unit_detector(&self) -> Vec<Rc<dyn CodeUnitDetector>> {
        vec![]
    }

    fn process_unit_detector(&self) -> Vec<Rc<dyn ProcessUnitDetector>> {
        vec![]
    }
}