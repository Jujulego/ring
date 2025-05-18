use crate::Unit;
use ring_core_content::Language;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Descriptive data of a given unit
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitData {
    pub kind: String,
    pub language: Option<Language>,
    pub name: Option<String>,
    pub root: PathBuf,
}

impl Unit for UnitData {
    #[inline]
    fn kind(&self) -> &str {
        &self.kind
    }

    #[inline]
    fn root(&self) -> &Path {
        &self.root
    }

    #[inline]
    fn language(&self) -> Option<Language> {
        self.language.clone()
    }

    #[inline]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[inline]
    fn inspect(&self) -> UnitData {
        self.clone()
    }
}