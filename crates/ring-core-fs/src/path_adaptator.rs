use std::path::Path;

/// Path based filesystem adaptator
pub trait PathAdaptator {
    /// Indicates if the adaptator can handle given path
    fn is_supported(&self, path: &Path) -> bool;
    
    /// Tests if given path exists and is a file
    fn is_file(&self, path: &Path) -> anyhow::Result<bool>;
}