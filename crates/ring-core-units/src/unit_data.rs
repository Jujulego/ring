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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_implement_unit_trait() {
        let data = UnitData {
            kind: "kind".to_string(),
            language: None,
            name: Some("name".to_string()),
            root: PathBuf::from("/test"),
        };

        assert_eq!(data.kind(), "kind");
        assert_eq!(data.root(), Path::new("/test"));
        assert!(data.language().is_none());
        assert_eq!(data.name(), Some("name"));
    }

    #[test]
    fn inspect_should_return_a_clone() {
        let data = UnitData {
            kind: "kind".to_string(),
            language: None,
            name: Some("name".to_string()),
            root: PathBuf::from("/test"),
        };

        assert_eq!(data.root, data.inspect().root);
    }
}