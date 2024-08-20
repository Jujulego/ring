use std::rc::Rc;
use ring_utils::Tag;
use crate::DetectAs;

pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> &[&'static Tag];
}

pub type TaggedDetector = dyn DetectAs<Rc<dyn Tagged>>;
