use ring_utils::Tag;

pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> &[&'static Tag];
}
