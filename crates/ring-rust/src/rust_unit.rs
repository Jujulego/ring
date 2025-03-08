use std::path::{Path, PathBuf};
use ring_core::units::{Language, Unit};
use crate::constants::RUST_LANGUAGE;

/// Represents a file written in rust
#[derive(Clone, Debug)]
pub struct RustUnit(PathBuf);

impl RustUnit {
    pub(crate) fn new(path: PathBuf) -> RustUnit {
        RustUnit(path)
    }
}

impl Unit for RustUnit {
    fn language(&self) -> &Language {
        &RUST_LANGUAGE
    }
    
    fn path(&self) -> &Path {
        &self.0
    }
}