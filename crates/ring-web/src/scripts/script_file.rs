use crate::{Package, WebLanguage};
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::Tagged;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Represents a web script file
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

    /// Returns web language of this script
    pub fn language(&self) -> &WebLanguage {
        &self.language
    }

    /// Returns the package that this script belongs to, if detected
    pub fn package(&self) -> Option<&Rc<Package>> {
        self.package.as_ref()
    }

    pub fn package_mut(&mut self) -> &mut Option<Rc<Package>> {
        &mut self.package
    }

    /// Returns path to this script
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl CodeUnit for ScriptFile {
    fn language(&self) -> CodeLanguage {
        self.language.into()
    }

    /// Returns the package that this script belongs to
    ///
    /// See: [`ScriptFile::package`]
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
