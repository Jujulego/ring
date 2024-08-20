use std::rc::Rc;
use crate::{ProjectDetector, ScopeDetector};
use crate::tagged::TaggedDetector;

pub trait Module {
    fn name(&self) -> &'static str;
    
    #[inline]
    fn project_detectors(&self) -> Vec<Rc<ProjectDetector>> {
        vec![]
    }
    
    #[inline]
    fn scope_detectors(&self) -> Vec<Rc<ScopeDetector>> {
        vec![]
    }
    
    #[inline]
    fn tagged_detectors(&self) -> Vec<Rc<TaggedDetector>> {
        vec![]
    }
}