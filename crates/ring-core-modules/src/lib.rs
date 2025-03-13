use ring_core_language::DetectLanguage;
use std::rc::Rc;

pub trait Module {
    /// Returns all language detectors implemented by this module.
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![]
    }
}