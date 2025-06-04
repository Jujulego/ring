mod json_file;
mod utils;

pub use crate::json_file::JsonFileDetector;
pub use crate::utils::json_language;
use ring_core_content::DetectLanguage;
use ring_core_fs::traits::AbsFilesystem;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Clone)]
pub struct JsonModule {
    json_file_detector: Rc<JsonFileDetector>,
}

impl JsonModule {
    /// Creates a new instance of JsonModule
    #[inline]
    pub fn new(filesystem: Rc<dyn AbsFilesystem>) -> Self {
        JsonModule {
            json_file_detector: Rc::new(JsonFileDetector::new(filesystem))
        }
    }

    /// Returns a pointer on JsonFileDetector
    #[inline]
    pub fn json_file_detector(&self) -> Rc<JsonFileDetector> {
        self.json_file_detector.clone()
    }
}

impl Module for JsonModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.json_file_detector()]
    }
}