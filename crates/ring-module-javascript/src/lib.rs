mod javascript_file;
mod utils;

pub use crate::javascript_file::JavascriptFileDetector;
pub use crate::utils::javascript_language;
use ring_core_language::DetectLanguage;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct JavascriptModule {
    javascript_file_detector: Rc<JavascriptFileDetector>,
}

impl JavascriptModule {
    /// Creates a new instance of JavascriptModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on JavascriptFileDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::JavascriptModule;
    ///
    /// let module = JavascriptModule::new();
    /// let detector = module.javascript_file_detector();
    /// ```
    #[inline]
    pub fn javascript_file_detector(&self) -> Rc<JavascriptFileDetector> {
        self.javascript_file_detector.clone()
    }
}

impl Module for JavascriptModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.javascript_file_detector()]
    }
}
