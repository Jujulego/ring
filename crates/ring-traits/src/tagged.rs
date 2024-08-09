pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> &[&'static str];
}