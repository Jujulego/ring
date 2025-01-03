mod language;
mod crates;
mod processes;

pub use crate::crates::{CargoCrate, CargoCrateDetector, CargoManifest, SourceFile, SourceFileDetector};
pub use crate::processes::{CargoProcess, CargoProcessDetector};
use ring_core::{CodeUnitDetector, CombinedCodeUnitDetector, ProcessUnitDetector, RingModule};
use std::rc::Rc;

pub struct RustModule {
    pub cargo_crate_detector: Rc<CargoCrateDetector>,
    pub source_file_detector: Rc<SourceFileDetector>,
    pub rust_unit_detector: Rc<CombinedCodeUnitDetector>,

    pub cargo_process_detector: Rc<CargoProcessDetector>,
}

impl RustModule {
    pub fn new() -> Self {
        let cargo_crate_detector = Rc::new(CargoCrateDetector::new());
        let source_file_detector = Rc::new(SourceFileDetector::new());

        let cargo_process_detector = Rc::new(CargoProcessDetector::new(
            cargo_crate_detector.clone(),
        ));

        RustModule {
            cargo_crate_detector: cargo_crate_detector.clone(),
            source_file_detector: source_file_detector.clone(),
            rust_unit_detector: Rc::new(CombinedCodeUnitDetector::new("rust".to_string(), &[
                cargo_crate_detector,
                source_file_detector,
            ])),
            
            cargo_process_detector,
        }
    }
}

impl Default for RustModule {
    fn default() -> Self {
        RustModule::new()
    }
}

impl RingModule for RustModule {
    fn code_unit_detector(&self) -> Vec<Rc<dyn CodeUnitDetector>> {
        vec![self.rust_unit_detector.clone()]
    }

    fn process_unit_detector(&self) -> Vec<Rc<dyn ProcessUnitDetector>> {
        vec![self.cargo_process_detector.clone()]
    }
}