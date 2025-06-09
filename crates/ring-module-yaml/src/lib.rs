mod utils;
mod yaml_file;

pub use crate::utils::yaml_language;
pub use crate::yaml_file::YamlFileDetector;
use ring_core_content::DetectLanguage;
use ring_core_fs::Filesystem;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Clone)]
pub struct YamlModule {
    yaml_file_detector: Rc<YamlFileDetector>,
}

impl YamlModule {
    /// Creates a new instance of YamlModule
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> Self {
        YamlModule {
            yaml_file_detector: Rc::new(YamlFileDetector::new(filesystem)),
        }
    }

    /// Returns a pointer on YamlFileDetector
    #[inline]
    pub fn yaml_file_detector(&self) -> Rc<YamlFileDetector> {
        self.yaml_file_detector.clone()
    }
}

impl Module for YamlModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.yaml_file_detector()]
    }
}