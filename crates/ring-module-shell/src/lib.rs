mod powershell_file_detector;
mod powershell_task;
mod powershell_task_detector;
mod shell_file_detector;
mod shell_task;
mod shell_task_detector;
mod utils;

pub use crate::powershell_file_detector::PowershellFileDetector;
pub use crate::powershell_task_detector::PowershellTaskDetector;
pub use crate::shell_file_detector::ShellFileDetector;
pub use crate::shell_task::{ShellKind, ShellTask};
pub use crate::shell_task_detector::ShellTaskDetector;
pub use crate::utils::{powershell_language, shell_language};
use ring_core_content::{DetectLanguage, QualifyPath};
use ring_core_fs::PathAdaptator;
use ring_core_modules::{Module, RegistryRef};
use ring_core_tasks::DetectProcessTask;
use std::rc::Rc;

#[derive(Clone)]
pub struct ShellModule {
    powershell_file_detector: Rc<PowershellFileDetector>,
    powershell_task_detector: Rc<PowershellTaskDetector>,
    shell_file_detector: Rc<ShellFileDetector>,
    shell_task_detector: Rc<ShellTaskDetector>,
}

impl ShellModule {
    /// Creates a new instance of ShellModule
    #[inline]
    pub fn new(registry: Rc<RegistryRef>, path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        ShellModule {
            powershell_file_detector: Rc::new(PowershellFileDetector::new(path_adaptator.clone())),
            powershell_task_detector: Rc::new(PowershellTaskDetector::new(registry.clone())),
            shell_file_detector: Rc::new(ShellFileDetector::new(path_adaptator)),
            shell_task_detector: Rc::new(ShellTaskDetector::new(registry)),
        }
    }

    /// Returns a pointer on PowershellFileDetector
    #[inline]
    pub fn powershell_file_detector(&self) -> Rc<PowershellFileDetector> {
        self.powershell_file_detector.clone()
    }

    /// Returns a pointer on PowershellTaskDetector
    #[inline]
    pub fn powershell_task_detector(&self) -> Rc<PowershellTaskDetector> {
        self.powershell_task_detector.clone()
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
        vec![self.powershell_file_detector(), self.shell_file_detector()]
    }

    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![self.powershell_file_detector(), self.shell_file_detector()]
    }

    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectProcessTask>> {
        vec![self.powershell_task_detector(), self.shell_task_detector()]
    }
}