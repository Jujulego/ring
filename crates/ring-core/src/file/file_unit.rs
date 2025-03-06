use std::path::Path;

/// Represents a file
pub trait FileUnit {
    /// Returns file's name
    fn name(&self) -> Option<&str> {
        self.path().file_stem()
            .and_then(|name| name.to_str())
    }
    
    /// Returns path to the file
    fn path(&self) -> &Path;
}