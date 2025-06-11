mod toml_file;
mod utils;

pub use crate::toml_file::TomlFileDetector;
pub use crate::utils::toml_language;
use ring_core_content::DetectLanguage;
use ring_core_fs::Filesystem;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Clone)]
pub struct TomlModule {
    toml_file_detector: Rc<TomlFileDetector>,
}

impl TomlModule {
    /// Creates a new instance of TomlModule
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> Self {
        TomlModule {
            toml_file_detector: Rc::new(TomlFileDetector::new(filesystem))
        }
    }

    /// Returns a pointer on TomlFileDetector
    #[inline]
    pub fn toml_file_detector(&self) -> Rc<TomlFileDetector> {
        self.toml_file_detector.clone()
    }
}

impl Module for TomlModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.toml_file_detector()]
    }
}