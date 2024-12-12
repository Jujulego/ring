use crate::{Package, WebLanguage};
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::Tagged;
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
    
    pub fn set_package(&mut self, package: Rc<Package>) {
        self.package = Some(package);
    }
    
    pub fn package(&self) -> Option<&Rc<Package>> {
        self.package.as_ref()
    }
}

impl CodeUnit for ScriptFile {
    fn language(&self) -> CodeLanguage {
        self.language.into()
    }

    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        self.package
            .clone()
            .map(|p| p as Rc<dyn CodeUnit>)
    }
    
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for ScriptFile {}
