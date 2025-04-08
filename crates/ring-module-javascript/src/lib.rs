mod javascript_file;
mod utils;
mod npm_package;

pub use crate::javascript_file::JavascriptFileDetector;
pub use crate::npm_package::{PackageManifest, NpmPackage, NpmPackageDetector};
pub use crate::utils::javascript_language;
use ring_core_file::{DetectLanguage, QualifyPath};
use ring_core_modules::Module;
use std::rc::Rc;
use ring_core_units::DetectUnit;

#[derive(Debug, Default, Clone)]
pub struct JavascriptModule {
    npm_package_detector: Rc<NpmPackageDetector>,
    javascript_file_detector: Rc<JavascriptFileDetector>,
}

impl JavascriptModule {
    /// Creates a new instance of JavascriptModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on NpmPackageDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::JavascriptModule;
    ///
    /// let module = JavascriptModule::new();
    /// let detector = module.npm_package_detector();
    /// ```
    #[inline]
    pub fn npm_package_detector(&self) -> Rc<NpmPackageDetector> {
        self.npm_package_detector.clone()
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
        vec![
            self.npm_package_detector(),
            self.javascript_file_detector(),
        ]
    }

    #[inline]
    fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
        vec![self.npm_package_detector()]
    }

    #[inline]
    fn path_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![self.npm_package_detector()]
    }
}
