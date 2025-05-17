use crate::unit_data::UnitData;
use ring_core_content::Language;
use std::path::Path;

/// Group of files, contained in a same directory, working together in a given purpose
pub trait Unit {
    /// Returns the unit's kind
    fn kind(&self) -> &str;

    /// Returns the unit's root path
    fn root(&self) -> &Path;

    /// Returns the unit's main language, if any
    #[inline]
    fn language(&self) -> Option<Language> {
        None
    }

    /// Returns the unit's name, if any
    #[inline]
    fn name(&self) -> Option<&str> {
        None
    }

    /// Builds a [`UnitData`] object from the current task
    #[inline]
    fn inspect(&self) -> UnitData {
        UnitData {
            kind: self.kind().to_string(),
            language: self.language(),
            name: self.name().map(|n| n.to_string()),
            root: self.root().to_path_buf(),
        }
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        self.language()
            .map(|language| language.style())
            .unwrap_or_default()
    }
}