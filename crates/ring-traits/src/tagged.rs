use crate::DetectAs;
use ring_utils::Tag;
use std::rc::Rc;

pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> &[&'static Tag];
}

pub type TaggedDetector = dyn DetectAs<Rc<dyn Tagged>>;