use ring_code_unit::CodeUnit;
use ring_tag::{Tag, Tagged};
use std::path::{Path, PathBuf};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// Javascript File
////////////////////////////////////////////////////////////////////////////////

/// Represents a Javascript file.
pub struct JavascriptFile {
    path: PathBuf,
    language: Tag,
}

impl JavascriptFile {
    pub fn new(path: PathBuf, language: Tag) -> JavascriptFile {
        JavascriptFile { path, language }
    }
}

impl CodeUnit for JavascriptFile {
    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        None
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for JavascriptFile {
    fn tags(&self) -> Vec<Tag> {
        vec![self.language.clone()]
    }
}