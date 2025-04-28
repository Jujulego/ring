mod cargo_crate;
mod cargo_crate_detector;
mod cargo_task;
mod cargo_task_detector;
mod rust_file;
mod rustup_task;
mod rustup_task_detector;
mod utils;

pub use crate::cargo_crate_detector::CargoCrateDetector;
pub use crate::cargo_task_detector::CargoTaskDetector;
pub use crate::rust_file::RustFileDetector;
pub use crate::rustup_task_detector::RustupTaskDetector;
pub use crate::utils::rust_language;
use ring_core_file::{DetectLanguage, QualifyFile};
use ring_core_modules::Module;
use ring_core_tasks::DetectTask;
use ring_core_units::DetectUnit;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RustModule {
    cargo_crate_detector: Rc<CargoCrateDetector>,
    cargo_task_detector: Rc<CargoTaskDetector>,
    rust_file_detector: Rc<RustFileDetector>,
    rustup_task_detector: Rc<RustupTaskDetector>,
}

impl RustModule {
    /// Creates a new instance of RustModule
    #[inline]
    pub fn new() -> Self {
        let cargo_crate_detector = Rc::new(CargoCrateDetector::new());

        RustModule {
            cargo_crate_detector: cargo_crate_detector.clone(),
            cargo_task_detector: Rc::new(CargoTaskDetector::new(cargo_crate_detector)),
            rust_file_detector: Rc::new(RustFileDetector::new()),
            rustup_task_detector: Rc::new(RustupTaskDetector::new()),
        }
    }

    /// Returns a pointer on CargoCrateDetector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ring_module_rust::RustModule;
    /// 
    /// let module = RustModule::new();
    /// let detector = module.cargo_crate_detector();
    /// ```
    #[inline]
    pub fn cargo_crate_detector(&self) -> Rc<CargoCrateDetector> {
        self.cargo_crate_detector.clone()
    }

    /// Returns a pointer on CargoTaskDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::RustModule;
    ///
    /// let module = RustModule::new();
    /// let detector = module.cargo_task_detector();
    /// ```
    #[inline]
    pub fn cargo_task_detector(&self) -> Rc<CargoTaskDetector> {
        self.cargo_task_detector.clone()
    }

    /// Returns a pointer on RustFileDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::RustModule;
    ///
    /// let module = RustModule::new();
    /// let detector = module.rust_file_detector();
    /// ```
    #[inline]
    pub fn rust_file_detector(&self) -> Rc<RustFileDetector> {
        self.rust_file_detector.clone()
    }

    /// Returns a pointer on RustupTaskDetector
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::RustModule;
    ///
    /// let module = RustModule::new();
    /// let detector = module.rustup_task_detector();
    /// ```
    #[inline]
    pub fn rustup_task_detector(&self) -> Rc<RustupTaskDetector> {
        self.rustup_task_detector.clone()
    }
}

impl Default for RustModule {
    fn default() -> Self {
        RustModule::new()
    }
}

impl Module for RustModule {
    #[inline]
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![
            self.cargo_crate_detector(),
            self.rust_file_detector()
        ]
    }

    #[inline]
    fn file_qualifiers(&self) -> Vec<Rc<dyn QualifyFile>> {
        vec![
            self.cargo_crate_detector(),
        ]
    }

    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectTask>> {
        vec![
            self.cargo_task_detector(),
            self.rustup_task_detector(),
        ]
    }

    #[inline]
    fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
        vec![
            self.cargo_crate_detector(),
        ]
    }
}