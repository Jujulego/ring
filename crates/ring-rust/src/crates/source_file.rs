use crate::language::rust_language;
use ring_color::ColorLabel;
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::{Tag, Tagged};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::CargoCrate;
use crate::constants::RUST_COLOR;

/// Represents a rust source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    path: PathBuf,
    cargo_crate: Option<Rc<CargoCrate>>,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> SourceFile {
        SourceFile {
            path,
            cargo_crate: None,
        }
    }

    /// Returns path to this script
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn cargo_crate(&self) -> Option<&Rc<CargoCrate>> {
        self.cargo_crate.as_ref()
    }
    
    pub fn cargo_crate_mut(&mut self) -> &mut Option<Rc<CargoCrate>> {
        &mut self.cargo_crate
    }
}

impl CodeUnit for SourceFile {
    fn language(&self) -> CodeLanguage {
        rust_language()
    }

    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        self.cargo_crate
            .clone()
            .map(|c| c as Rc<dyn CodeUnit>)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for SourceFile {
    fn tags(&self) -> Vec<Tag> {
        let mut tags = vec![];

        let dirname = self.path.parent().and_then(Path::file_name).and_then(OsStr::to_str);
        let filename = self.path.file_name().and_then(OsStr::to_str);

        match (dirname, filename) {
            (Some("src"), Some("lib.rs")) |
            (Some("src"), Some("main.rs")) => tags.push(Tag::from("rust:main").with_color(ColorLabel::Red, RUST_COLOR)),
            (Some(dir), Some("build.rs")) if dir != "src" => tags.push(Tag::from("rust:build").with_color(ColorLabel::Red, RUST_COLOR)),
            _ => {}
        }

        tags
    }
}