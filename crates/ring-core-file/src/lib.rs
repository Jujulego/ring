use std::path::Path;

/// Object able to qualify a given path
pub trait QualifyPath {
    fn qualify_content(&self, path: &Path) -> Option<FileContent>;
}

/// Define the kind of content detected inside a file
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileContent {
    Configuration,
    Lockfile,
    Manifest,
    Source,
    Test,
    Other(String),
}