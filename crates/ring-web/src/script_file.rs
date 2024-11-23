use ring_code_unit::CodeUnit;
use ring_tag::{Tag, Tagged};
use std::path::{Path, PathBuf};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// Script File
////////////////////////////////////////////////////////////////////////////////

/// Represents a script file
pub struct ScriptFile {
    path: PathBuf,
    language: Tag,
}

impl ScriptFile {
    pub fn new(path: PathBuf, language: Tag) -> ScriptFile {
        ScriptFile { path, language }
    }
}

impl CodeUnit for ScriptFile {
    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        None
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for ScriptFile {
    fn tags(&self) -> Vec<Tag> {
        vec![self.language.clone()]
    }
}