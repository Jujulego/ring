mod language;
mod node_process;
mod package;
mod package_manager;
mod package_manifest;
mod script_file;
mod web_process_detector;
mod web_unit_detector;

pub use crate::language::WebLanguage;
pub use crate::node_process::NodeProcess;
pub use crate::package::Package;
pub use crate::script_file::ScriptFile;
pub use crate::web_process_detector::WebProcessDetector;
pub use crate::web_unit_detector::WebUnitDetector;
use std::rc::Rc;
use ring_core::{CodeUnitDetector, ProcessUnitDetector, RingModule};
////////////////////////////////////////////////////////////////////////////////
// Module
////////////////////////////////////////////////////////////////////////////////

pub struct WebModule {
    pub web_unit_detector: Rc<WebUnitDetector>,
    pub web_process_detector: Rc<WebProcessDetector>,
}

impl WebModule {
    pub fn new() -> WebModule {
        let web_unit_detector = Rc::new(WebUnitDetector::new());

        WebModule {
            web_unit_detector: web_unit_detector.clone(),
            web_process_detector: Rc::new(WebProcessDetector::new(web_unit_detector)),
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