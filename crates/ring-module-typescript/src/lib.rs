mod utils;
mod typescript_file;

pub use crate::utils::typescript_language;
pub use crate::typescript_file::TypescriptFileDetector;
use ring_core_language::DetectLanguage;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct TypescriptModule {
    typescript_file_detector: Rc<TypescriptFileDetector>,
}

impl TypescriptModule {
    /// Creates a new instance of TypescriptModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on TypescriptFileDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_typescript::TypescriptModule;
    ///
    /// let module = TypescriptModule::new();
    /// let detector = module.typescript_file_detector();
    /// ```
    #[inline]
    pub fn typescript_file_detector(&self) -> Rc<TypescriptFileDetector> {
        self.typescript_file_detector.clone()
    }
}

impl Module for TypescriptModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.typescript_file_detector()]
    }
}