mod language;
mod node_process;
mod packages;
mod scripts;
mod web_process_detector;

pub use crate::language::WebLanguage;
pub use crate::node_process::NodeProcess;
pub use crate::packages::{Package, PackageDetector, PackageManager, PackageManifest};
pub use crate::scripts::{ScriptFile, ScriptFileDetector};
pub use crate::web_process_detector::WebProcessDetector;
use ring_core::{CodeUnitDetector, CombinedCodeUnitDetector, ProcessUnitDetector, RingModule};
use std::rc::Rc;

pub struct WebModule {
    pub package_detector: Rc<PackageDetector>,
    pub script_file_detector: Rc<ScriptFileDetector>,
    pub web_unit_detector: Rc<CombinedCodeUnitDetector<2>>,

    pub web_process_detector: Rc<WebProcessDetector>,
}

impl WebModule {
    pub fn new() -> WebModule {
        let package_detector = Rc::new(PackageDetector::new());
        let script_file_detector = Rc::new(ScriptFileDetector::new(
            package_detector.clone(),
        ));

        WebModule {
            package_detector: package_detector.clone(),
            script_file_detector: script_file_detector.clone(),

            web_unit_detector: Rc::new([
                package_detector.clone(),
                script_file_detector.clone(),
            ]),
            web_process_detector: Rc::new(WebProcessDetector::new(script_file_detector)),
        }
    }
}

impl Default for WebModule {
    fn default() -> Self {
        WebModule::new()
    }
}

impl RingModule for WebModule {
    fn code_unit_detector(&self) -> Vec<Rc<dyn CodeUnitDetector>> {
        vec![self.web_unit_detector.clone()]
    }

    fn process_unit_detector(&self) -> Vec<Rc<dyn ProcessUnitDetector>> {
        vec![self.web_process_detector.clone()]
    }
}