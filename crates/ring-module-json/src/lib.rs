mod utils;
mod json_file;

pub use crate::utils::json_language;
pub use crate::json_file::JsonFileDetector;
use ring_core_language::DetectLanguage;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct JsonModule {
    json_file_detector: Rc<JsonFileDetector>,
}

impl JsonModule {
    /// Creates a new instance of JsonModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on JsonFileDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_json::JsonModule;
    ///
    /// let module = JsonModule::new();
    /// let detector = module.json_file_detector();
    /// ```
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