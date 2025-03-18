use std::fmt::Display;
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

impl Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileContent::Configuration => write!(f, "config"),
            FileContent::Lockfile => write!(f, "lockfile"),
            FileContent::Manifest => write!(f, "manifest"),
            FileContent::Source => write!(f, "source"),
            FileContent::Test => write!(f, "test"),
            FileContent::Other(name) => name.fmt(f),
        }
    }
}