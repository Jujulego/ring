use std::rc::Rc;
use crate::Detector;

pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> &[&'static str];
}

pub type TaggedDetector = dyn Detector<Item = Rc<dyn Tagged>>;