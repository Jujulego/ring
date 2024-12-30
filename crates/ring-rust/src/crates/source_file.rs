use crate::language::{rust_language, RUST_COLOR};
use ring_color::ColorLabel;
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::{Tag, Tagged};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Represents a rust source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    path: PathBuf,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> SourceFile {
        SourceFile { path }
    }

    /// Returns path to this script
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl CodeUnit for SourceFile {
    fn language(&self) -> CodeLanguage {
        rust_language()
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