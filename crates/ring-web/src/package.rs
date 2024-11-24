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
    language: WebLanguage,
}

impl Package {
    pub fn new(path: PathBuf, language: WebLanguage) -> Package {
        Package { path, language }
    }
}

impl CodeUnit for Package {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for Package {
    fn tags(&self) -> Vec<Tag> {
        vec![self.language.tag()]
    }
}