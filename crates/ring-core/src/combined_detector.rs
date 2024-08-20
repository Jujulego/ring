use std::path::Path;
use std::rc::Rc;
use ring_traits::DetectAs;

pub struct CombinedDetector<T> {
    detectors: Vec<Rc<dyn DetectAs<T>>>,
}

impl<T> CombinedDetector<T> {
    pub fn new(detectors: Vec<Rc<dyn DetectAs<T>>>) -> CombinedDetector<T> {
        CombinedDetector { detectors }
    }
    
    pub fn detect_from(&self, path: &Path) -> Vec<anyhow::Result<T>> {
        self.detectors.iter()
            .filter_map(|dct| dct.detect_from_as(path).into())
            .collect()
    }
}