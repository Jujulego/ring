use ring_core_file::QualifyPath;
use ring_core_language::DetectLanguage;
use std::rc::Rc;

pub trait Module {
    /// Returns all language detectors implemented by this module.
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![]
    }

    /// Returns all path qualifiers implemented by this module.
    fn path_qualifiers(&self) -> Vec<Rc<dyn QualifyPath>> {
        vec![]
    }
}