mod shell_file_detector;
mod shell_task;
mod shell_task_detector;
mod utils;

pub use crate::shell_file_detector::ShellFileDetector;
pub use crate::shell_task_detector::ShellTaskDetector;
pub use crate::utils::shell_language;
use ring_core_file::{DetectLanguage, QualifyPath};
use ring_core_modules::Module;
use std::rc::Rc;
use ring_core_tasks::DetectTask;

#[derive(Debug, Default, Clone)]
pub struct ShellModule {
    shell_file_detector: Rc<ShellFileDetector>,
    shell_task_detector: Rc<ShellTaskDetector>,
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

    /// Returns a pointer on ShellTaskDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_shell::ShellModule;
    ///
    /// let module = ShellModule::new();
    /// let detector = module.shell_task_detector();
    /// ```
    #[inline]
    pub fn shell_task_detector(&self) -> Rc<ShellTaskDetector> {
        self.shell_task_detector.clone()
    }
}

impl Module for ShellModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.shell_file_detector()]
    }

    #[inline]
    fn path_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![self.shell_file_detector()]
    }

    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectTask>> {
        vec![self.shell_task_detector()]
    }
}