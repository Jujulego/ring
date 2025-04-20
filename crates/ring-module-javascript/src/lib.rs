mod javascript_file;
mod node_task;
mod node_task_detector;
mod npm_package;
mod npm_package_detector;
mod utils;

pub use crate::javascript_file::JavascriptFileDetector;
pub use crate::node_task_detector::NodeTaskDetector;
pub use crate::npm_package::{NpmPackage, PackageManifest};
pub use crate::npm_package_detector::NpmPackageDetector;
pub use crate::utils::javascript_language;
use ring_core_file::{DetectLanguage, QualifyPath};
use ring_core_modules::Module;
use ring_core_tasks::DetectTask;
use ring_core_units::DetectUnit;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct JavascriptModule {
    node_task_detector: Rc<NodeTaskDetector>,
    npm_package_detector: Rc<NpmPackageDetector>,
    javascript_file_detector: Rc<JavascriptFileDetector>,
}

impl JavascriptModule {
    /// Creates a new instance of JavascriptModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on NodeTaskDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::JavascriptModule;
    ///
    /// let module = JavascriptModule::new();
    /// let detector = module.node_task_detector();
    /// ```
    #[inline]
    pub fn node_task_detector(&self) -> Rc<NodeTaskDetector> {
        self.node_task_detector.clone()
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
    fn path_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![self.npm_package_detector()]
    }

    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectTask>> {
        vec![self.node_task_detector()]
    }

    #[inline]
    fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
        vec![self.npm_package_detector()]
    }
}
