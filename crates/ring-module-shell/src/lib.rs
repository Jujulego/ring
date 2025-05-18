mod shell_file_detector;
mod shell_task;
mod shell_task_detector;
mod utils;

pub use crate::shell_file_detector::ShellFileDetector;
pub use crate::shell_task::{ShellKind, ShellTask};
pub use crate::shell_task_detector::ShellTaskDetector;
pub use crate::utils::shell_language;
use ring_core_content::{DetectLanguage, QualifyPath};
use ring_core_modules::{Module, RegistryRef};
use ring_core_tasks::DetectProcessTask;
use std::rc::Rc;

#[derive(Clone)]
pub struct ShellModule {
    shell_file_detector: Rc<ShellFileDetector>,
    shell_task_detector: Rc<ShellTaskDetector>,
}

impl ShellModule {
    /// Creates a new instance of ShellModule
    #[inline]
    pub fn new(registry: Rc<RegistryRef>) -> Self {
        ShellModule {
            shell_file_detector: Rc::new(ShellFileDetector::new()),
            shell_task_detector: Rc::new(ShellTaskDetector::new(registry)),
        }
    }

    /// Returns a pointer on ShellFileDetector
    #[inline]
    pub fn shell_file_detector(&self) -> Rc<ShellFileDetector> {
        self.shell_file_detector.clone()
    }

    /// Returns a pointer on ShellTaskDetector
    #[inline]
    pub fn shell_task_detector(&self) -> Rc<ShellTaskDetector> {
        self.shell_task_detector.clone()
    }
}

impl Module for ShellModule {
    #[inline]
    fn file_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![self.shell_file_detector()]
    }

    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.shell_file_detector()]
    }

    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectProcessTask>> {
        vec![self.shell_task_detector()]
    }
}