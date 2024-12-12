use std::rc::Rc;
use ring_core::{CodeUnitDetector, ProcessUnitDetector, RingModule};

#[cfg(feature = "web")]
use ring_web::WebModule;

pub struct RingCore {
    modules: Vec<Box<dyn RingModule>>,
}

impl RingCore {
    pub fn new() -> RingCore {
        RingCore {
            modules: vec![
                #[cfg(feature = "web")]
                { Box::new(WebModule::default()) }
            ]
        }
    }

    pub fn code_unit_detectors(&self) -> impl Iterator<Item = Rc<dyn CodeUnitDetector>> + use<'_> {
        self.modules.iter()
            .flat_map(|module| module.code_unit_detector())
    }

    pub fn process_unit_detectors(&self) -> impl Iterator<Item = Rc<dyn ProcessUnitDetector>> + use<'_> {
        self.modules.iter()
            .flat_map(|module| module.process_unit_detector())
    }
}

impl Default for RingCore {
    fn default() -> Self {
        Self::new()
    }
}