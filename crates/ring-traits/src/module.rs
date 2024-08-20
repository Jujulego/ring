use std::rc::Rc;
use crate::ProjectDetector;
use crate::tagged::TaggedDetector;

pub trait Module {
    #[inline]
    fn project_detectors(&self) -> Vec<Rc<ProjectDetector>> {
        vec![]
    }
    
    #[inline]
    fn tagged_detectors(&self) -> Vec<Rc<TaggedDetector>> {
        vec![]
    }
}