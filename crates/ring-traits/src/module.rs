use std::rc::Rc;
use crate::tagged::TaggedDetector;

pub trait Module {
    #[inline]
    fn tagged_detectors(&self) -> Vec<Rc<TaggedDetector>> {
        vec![]
    }
}