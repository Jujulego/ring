use crate::{Package, WebLanguage};
use ring_code_unit::CodeUnit;
use ring_tag::{Tag, Tagged};
use std::path::{Path, PathBuf};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// Script File
////////////////////////////////////////////////////////////////////////////////

/// Represents a script file
#[derive(Clone, Debug)]
pub struct ScriptFile {
    path: PathBuf,
    language: WebLanguage,
    package: Option<Rc<Package>>
}

impl ScriptFile {
    pub fn new(path: PathBuf, language: WebLanguage) -> ScriptFile {
        ScriptFile { path, language, package: None }
    }
    
    pub fn with_package(self, package: Rc<Package>) -> ScriptFile {
        ScriptFile {
            package: Some(package),
            ..self
        }
    }
    
    pub fn language(&self) -> &WebLanguage {
        &self.language
    }
    
    pub fn package(&self) -> Option<&Rc<Package>> {
        self.package.as_ref()
    }
}

impl CodeUnit for ScriptFile {
    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        self.package
            .clone()
            .map(|p| p as Rc<dyn CodeUnit>)
    }
    
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for ScriptFile {
    fn tags(&self) -> Vec<Tag> {
        vec![self.language.tag()]
    }
}
