use std::path::Path;

pub trait Detector {
    type Item;
    
    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Self::Item>>;
}
