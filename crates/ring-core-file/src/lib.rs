mod file_content;
mod language;

pub use crate::file_content::FileContent;
pub use crate::language::Language;
use std::path::Path;

/// Object able to detect language of a given path
pub trait DetectLanguage {
    fn detect_language(&self, path: &Path) -> Option<Language>;
}

/// Object able to qualify a given path
pub trait QualifyPath {
    fn qualify_content<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)>;
}
