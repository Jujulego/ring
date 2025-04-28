use ring_core_file::{DetectLanguage, QualifyFile};
use ring_core_tasks::DetectTask;
use ring_core_units::DetectUnit;
use std::rc::Rc;

pub trait Module {
    /// Returns all path qualifiers implemented by this module.
    fn file_qualifiers(&self) -> Vec<Rc<dyn QualifyFile>> {
        vec![]
    }

    /// Returns all language detectors implemented by this module.
    fn language_detectors(&self) -> Vec<Rc<dyn DetectLanguage>> {
        vec![]
    }

    /// Returns all task detectors implemented by this module.
    fn task_detectors(&self) -> Vec<Rc<dyn DetectTask>> {
        vec![]
    }

    /// Returns all unit detectors implemented by this module.
    fn unit_detectors(&self) -> Vec<Rc<dyn DetectUnit>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModule;
    impl Module for TestModule {}
    
    #[test]
    fn it_should_return_empty_vec_by_default() {
        let module = TestModule;
        
        assert!(module.language_detectors().is_empty());
        assert!(module.file_qualifiers().is_empty());
        assert!(module.task_detectors().is_empty());
        assert!(module.unit_detectors().is_empty());
    }
}