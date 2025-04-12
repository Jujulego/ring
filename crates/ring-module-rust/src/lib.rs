mod cargo_crate_detector;
mod rust_file;
mod utils;
mod cargo_crate;

pub use crate::cargo_crate_detector::CargoCrateDetector;
pub use crate::rust_file::RustFileDetector;
pub use crate::utils::rust_language;
use ring_core_file::{DetectLanguage, QualifyPath};
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct RustModule {
    cargo_project_detector: Rc<CargoCrateDetector>,
    rust_file_detector: Rc<RustFileDetector>,
}

impl RustModule {
    /// Creates a new instance of RustModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on CargoManifestDetector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ring_module_rust::RustModule;
    /// 
    /// let module = RustModule::new();
    /// let detector = module.rust_file_detector();
    /// ```
    #[inline]
    pub fn cargo_project_detector(&self) -> Rc<CargoCrateDetector> {
        self.cargo_project_detector.clone()
    }

    /// Returns a pointer on RustFileDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::RustModule;
    ///
    /// let module = RustModule::new();
    /// let detector = module.rust_file_detector();
    /// ```
    #[inline]
    pub fn rust_file_detector(&self) -> Rc<RustFileDetector> {
        self.rust_file_detector.clone()
    }
}

impl Module for RustModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![
            self.cargo_project_detector(),
            self.rust_file_detector()
        ]
    }

    #[inline]
    fn path_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![
            self.cargo_project_detector(), 
        ]
    }
}