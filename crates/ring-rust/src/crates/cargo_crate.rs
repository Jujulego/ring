use crate::language::rust_language;
use crate::CargoManifest;
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::{Tag, Tagged};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use ring_color::ColorLabel;
use crate::constants::RUST_COLOR;

#[derive(Clone, Debug)]
pub struct CargoCrate {
    manifest: CargoManifest,
    path: PathBuf,
}

impl CargoCrate {
    pub fn new(path: PathBuf, manifest: CargoManifest) -> CargoCrate {
        CargoCrate { manifest, path }
    }

    pub fn manifest(&self) -> &CargoManifest {
        &self.manifest
    }

    pub fn name(&self) -> Option<&str> {
        self.manifest.package.as_ref()
            .map(|pkg| pkg.name.as_str())
            .or_else(|| self.path.file_name().and_then(OsStr::to_str))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl CodeUnit for CargoCrate {
    fn language(&self) -> CodeLanguage {
        rust_language()
    }

    fn name(&self) -> Option<&str> {
        self.name()
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for CargoCrate {
    fn tags(&self) -> Vec<Tag> {
        vec![
            Tag::from("rust:cargo").with_color(ColorLabel::Red, RUST_COLOR),
        ]
    }
}