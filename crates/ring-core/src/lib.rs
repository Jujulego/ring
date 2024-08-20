pub use combined_detector::CombinedDetector;
use ring_traits::{Tagged, TaggedDetector};
use std::rc::Rc;

#[cfg(feature = "js")]
use ring_js::{JsProjectDetector, JsScopeDetector};

#[cfg(feature = "rust")]
use ring_rust::{RustProjectDetector, RustScopeDetector};

mod combined_detector;

pub fn build_tagged_detector() -> CombinedDetector<Rc<dyn Tagged>> {
    let mut tagged_detectors: Vec<Rc<TaggedDetector>> = Vec::new();

    #[cfg(feature = "js")] {
        let js_project_detector: Rc<JsProjectDetector> = Rc::new(JsProjectDetector::new());
        let js_scope_detector = Rc::new(JsScopeDetector::new(js_project_detector.clone()));

        tagged_detectors.push(js_project_detector);
        tagged_detectors.push(js_scope_detector);
    }

    #[cfg(feature = "rust")] {
        let rust_project_detector = Rc::new(RustProjectDetector::new());
        let rust_scope_detector = Rc::new(RustScopeDetector::new(rust_project_detector.clone()));

        tagged_detectors.push(rust_project_detector);
        tagged_detectors.push(rust_scope_detector);
    }

    CombinedDetector::new(tagged_detectors)
}
