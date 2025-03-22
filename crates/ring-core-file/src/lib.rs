mod file_content;

pub use crate::file_content::FileContent;
use std::path::Path;

/// Object able to qualify a given path
pub trait QualifyPath {
    fn qualify_content(&self, path: &Path) -> Option<FileContent>;
}
