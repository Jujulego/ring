use std::rc::Rc;
use crate::DetectAs;

pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> &[&'static str];
}

pub type TaggedDetector = dyn DetectAs<Rc<dyn Tagged>>;