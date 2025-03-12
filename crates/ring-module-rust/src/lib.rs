mod rust_file;
mod utils;

pub use crate::rust_file::RustFileDetector;
pub use crate::utils::rust_language;
use ring_core_language::DetectLanguage;
use ring_core_modules::Module;
use std::rc::Rc;

pub struct RustModule {
    rust_file_detector: Rc<RustFileDetector>,
}

impl RustModule {
    pub fn new() -> Self {
        RustModule {
            rust_file_detector: Rc::new(RustFileDetector::new()),
        }
    }

    pub fn rust_file_detector(&self) -> Rc<RustFileDetector> {
        self.rust_file_detector.clone()
    }
}

impl Module for RustModule {
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.rust_file_detector()]
    }
}