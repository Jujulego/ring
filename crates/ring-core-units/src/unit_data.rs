use ring_core_content::Language;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Descriptive data of a given unit
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitData {
    pub kind: String,
    pub language: Option<Language>,
    pub name: Option<String>,
    pub root: PathBuf,
}