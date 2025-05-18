use std::path::Path;

/// Path based filesystem adaptator
pub trait PathAdaptator {
    /// Tests if given path exists and is a file
    fn is_file(&self, path: &Path) -> anyhow::Result<bool>;
}