use crate::file::file_unit::FileUnit;
use crate::file::Language;

/// Represents a file containing code
pub trait CodeUnit: FileUnit {
    /// Returns code unit's language
    fn language(&self) -> &Language;
}