use crate::language::rust_language;
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::Tagged;
use std::path::{Path, PathBuf};

/// Represents a rust source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    path: PathBuf,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> SourceFile {
        SourceFile { path }
    }

    /// Returns path to this script
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl CodeUnit for SourceFile {
    fn language(&self) -> CodeLanguage {
        rust_language()
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for SourceFile {}