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
use ring_core_content::{DetectLanguage, QualifyPath};
use ring_core_fs::PathAdaptator;
use ring_core_modules::Module;
use ring_core_tasks::DetectProcessTask;
use ring_core_units::DetectUnit;
use std::rc::Rc;

#[derive(Clone)]
pub struct JavascriptModule {
    javascript_file_detector: Rc<JavascriptFileDetector>,
    node_task_detector: Rc<NodeTaskDetector>,
    npm_package_detector: Rc<NpmPackageDetector>,
}

impl JavascriptModule {
    /// Creates a new instance of JavascriptModule
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        let npm_package_detector = Rc::new(NpmPackageDetector::new(path_adaptator.clone()));
        
        JavascriptModule {
            javascript_file_detector: Rc::new(JavascriptFileDetector::new(path_adaptator)),
            node_task_detector: Rc::new(NodeTaskDetector::new(npm_package_detector.clone())),
            npm_package_detector,
        }
    }

    /// Returns a pointer on JavascriptFileDetector
    #[inline]
    pub fn javascript_file_detector(&self) -> Rc<JavascriptFileDetector> {
        self.javascript_file_detector.clone()
    }

    /// Returns a pointer on NodeTaskDetector
    #[inline]
    pub fn node_task_detector(&self) -> Rc<NodeTaskDetector> {
        self.node_task_detector.clone()
    }

    /// Returns a pointer on NpmPackageDetector
    #[inline]
    pub fn npm_package_detector(&self) -> Rc<NpmPackageDetector> {
        self.npm_package_detector.clone()
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
    fn file_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![self.npm_package_detector()]
    }

    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectProcessTask>> {
        vec![self.node_task_detector()]
    }

    #[inline]
    fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
        vec![self.npm_package_detector()]
    }
}
