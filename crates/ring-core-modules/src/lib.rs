use ring_core_file::{DetectLanguage, QualifyPath};
use ring_core_units::DetectUnit;
use std::rc::Rc;

pub trait Module {
    /// Returns all language detectors implemented by this module.
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![]
    }

    /// Returns all unit detectors implemented by this module.
    fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
        vec![]
    }

    /// Returns all path qualifiers implemented by this module.
    fn path_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![]
    }
}