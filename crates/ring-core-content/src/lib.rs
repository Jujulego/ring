mod file_content;
mod language;
mod path_content;

pub use crate::file_content::FileContent;
pub use crate::path_content::PathContent;
pub use crate::language::Language;
use std::path::Path;

/// Object able to detect language of a given path
pub trait DetectLanguage {
    fn detect_language(&self, path: &Path) -> Option<Language>;
}

/// Object able to qualify a given path
pub trait QualifyPath {
    #[deprecated(note = "use qualify_path instead")]
    fn qualify_file<'a>(&self, path: &'a Path) -> Option<(FileContent, &'a Path)>;
    
    fn qualify_path<'a>(&self, path: &'a Path) -> Option<(PathContent, &'a Path)> {
        self.qualify_file(path).map(|(p, content)| (p.into(), content))
    }
}
