mod file_content;

pub use crate::file_content::FileContent;
use std::path::Path;

/// Object able to qualify a given path
pub trait QualifyPath {
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, QualifiedPart<'a>)>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualifiedPart<'a> {
    FileName,
    Ancestor(&'a Path),
}