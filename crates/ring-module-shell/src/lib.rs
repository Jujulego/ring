mod shell_file_detector;
mod utils;

use std::rc::Rc;
use ring_core_file::DetectLanguage;
use ring_core_modules::Module;
pub use crate::shell_file_detector::ShellFileDetector;
pub use crate::utils::shell_language;

#[derive(Debug, Default, Clone)]
pub struct ShellModule {
    shell_file_detector: Rc<ShellFileDetector>,
}

impl ShellModule {
    /// Creates a new instance of ShellModule
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a pointer on ShellFileDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_shell::ShellModule;
    ///
    /// let module = ShellModule::new();
    /// let detector = module.shell_file_detector();
    /// ```
    #[inline]
    pub fn shell_file_detector(&self) -> Rc<ShellFileDetector> {
        self.shell_file_detector.clone()
    }
}

impl Module for ShellModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.shell_file_detector()]
    }
}