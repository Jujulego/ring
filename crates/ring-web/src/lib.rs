mod language;
mod packages;
mod processes;
mod scripts;

pub use crate::language::WebLanguage;
pub use crate::packages::{Package, PackageDetector, PackageManager, PackageManifest};
pub use crate::processes::{NodeProcess, NodeProcessDetector};
pub use crate::scripts::{ScriptFile, ScriptFileDetector};
use ring_core::{CodeUnitDetector, CombinedCodeUnitDetector, CombinedProcessUnitDetector, ProcessUnitDetector, RingModule};
use std::rc::Rc;

pub struct WebModule {
    pub package_detector: Rc<PackageDetector>,
    pub script_file_detector: Rc<ScriptFileDetector>,
    pub web_unit_detector: Rc<CombinedCodeUnitDetector>,

    pub node_process_detector: Rc<NodeProcessDetector>,
    pub web_process_detector: Rc<CombinedProcessUnitDetector<1>>,
}

impl WebModule {
    pub fn new() -> WebModule {
        let package_detector = Rc::new(PackageDetector::new());
        let script_file_detector = Rc::new(ScriptFileDetector::new(
            package_detector.clone(),
        ));

        let node_process_detector = Rc::new(NodeProcessDetector::new(script_file_detector.clone()));

        WebModule {
            package_detector: package_detector.clone(),
            script_file_detector: script_file_detector.clone(),
            web_unit_detector: Rc::new(CombinedCodeUnitDetector::new("web".to_string(), &[
                package_detector.clone(),
                script_file_detector.clone(),
            ])),

            node_process_detector: node_process_detector.clone(),
            web_process_detector: Rc::new([node_process_detector.clone()]),
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