mod language;
mod path_content;

pub use crate::language::Language;
pub use crate::path_content::PathContent;
use std::path::Path;

/// Object able to detect language of a given path
pub trait DetectLanguage {
    fn detect_language(&self, path: &Path) -> Option<Language>;
}

/// Object able to qualify a given path
pub trait QualifyPath {
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)>;
}
