use crate::WebLanguage;
use ring_code_unit::CodeUnit;
use ring_tag::{Tag, Tagged};
use std::path::{Path, PathBuf};

////////////////////////////////////////////////////////////////////////////////
// Package
////////////////////////////////////////////////////////////////////////////////

/// Represents a script package
#[derive(Clone, Debug)]
pub struct Package {
    path: PathBuf,
}

impl Package {
    pub fn new(path: PathBuf) -> Package {
        Package { path }
    }
}

impl CodeUnit for Package {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for Package {
    fn tags(&self) -> Vec<Tag> {
        vec![WebLanguage::JavaScript.tag("package".to_string())]
    }
}