mod utils;
mod typescript_file;
mod tsconfig_file;

pub use crate::tsconfig_file::TsconfigFileDetector;
pub use crate::typescript_file::TypescriptFileDetector;
pub use crate::utils::typescript_language;
use ring_core_content::{DetectLanguage, QualifyPath};
use ring_core_fs::Filesystem;
use ring_core_modules::Module;
use std::rc::Rc;

#[derive(Clone)]
pub struct TypescriptModule {
    tsconfig_file_detector: Rc<TsconfigFileDetector>,
    typescript_file_detector: Rc<TypescriptFileDetector>,
}

impl TypescriptModule {
    /// Creates a new instance of TypescriptModule
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> Self {
        TypescriptModule {
            tsconfig_file_detector: Rc::new(TsconfigFileDetector::new(filesystem.clone())),
            typescript_file_detector: Rc::new(TypescriptFileDetector::new(filesystem)),
        }
    }

    /// Returns a pointer on TsconfigFileDetector
    #[inline]
    pub fn tsconfig_file_detector(&self) -> Rc<TsconfigFileDetector> {
        self.tsconfig_file_detector.clone()
    }

    /// Returns a pointer on TypescriptFileDetector<
    #[inline]
    pub fn typescript_file_detector(&self) -> Rc<TypescriptFileDetector> {
        self.typescript_file_detector.clone()
    }
}

impl Module for TypescriptModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![
            self.tsconfig_file_detector(),
            self.typescript_file_detector()
        ]
    }

    #[inline]
    fn file_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![
            self.tsconfig_file_detector()
        ]
    }
}