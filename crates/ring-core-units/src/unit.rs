use ring_core_file::Language;
use std::path::Path;

/// Group of files, contained in a same directory, working together in a given purpose
pub trait Unit {
    /// Returns the unit's kind
    fn kind(&self) -> &str;

    /// Returns the unit's root path
    fn root(&self) -> &Path;

    /// Returns the unit's main language, if any
    fn language(&self) -> Option<Language> {
        None
    }

    /// Returns the unit's name, if any
    fn name(&self) -> Option<&str> {
        None
    }
}