pub use combined_detector::CombinedDetector;
use ring_traits::{Module, Project, Scope, Tagged};
use std::rc::Rc;

#[cfg(feature = "js")]
use ring_js::JsModule;

#[cfg(feature = "rust")]
use ring_rust::RustModule;

mod combined_detector;

#[derive(Debug, Default)]
pub struct RingCore {
    #[cfg(feature = "js")]   js_module: JsModule,
    #[cfg(feature = "rust")] rust_module: RustModule,
}

impl RingCore {
    pub fn new() -> RingCore {
        Default::default()
    }

    pub fn modules(&self) -> Vec<&dyn Module> {
        vec![
            #[cfg(feature = "js")]   &self.js_module,
            #[cfg(feature = "rust")] &self.rust_module,
        ]
    }

    pub fn project_detector(&self) -> CombinedDetector<Rc<dyn Project>> {
        CombinedDetector::new(
            self.modules().iter()
                .flat_map(|module| module.project_detectors())
                .collect(),
        )
    }

    pub fn scope_detector(&self) -> CombinedDetector<Rc<dyn Scope>> {
        CombinedDetector::new(
            self.modules().iter()
                .flat_map(|module| module.scope_detectors())
                .collect(),
        )
    }

    pub fn tagged_detector(&self) -> CombinedDetector<Rc<dyn Tagged>> {
        CombinedDetector::new(
            self.modules().iter()
                .flat_map(|module| module.tagged_detectors())
                .collect(),
        )
    }
}
