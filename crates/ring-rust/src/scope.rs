use std::path::{Path, PathBuf};
use std::rc::Rc;
use anyhow::Context;
use glob::glob;
use tracing::debug;
use ring_files::PatternIterator;
use ring_traits::{DetectAs, Detector, Project, Scope, Tagged};
use ring_utils::Tag;
use crate::{CargoManifest, CargoWorkspace, RustProjectDetector};
use crate::constants::RUST_TAG;

#[derive(Debug)]
pub struct RustScope {
    root: PathBuf,
    manifest: Rc<CargoManifest>,
    project_detector: Rc<RustProjectDetector>,
}

impl RustScope {
    pub fn new(root: PathBuf, manifest: Rc<CargoManifest>, project_detector: Rc<RustProjectDetector>) -> RustScope {
        RustScope { root, manifest, project_detector }
    }
    
    pub fn workspace(&self) -> &CargoWorkspace {
        self.manifest.workspace.as_ref().unwrap()
    }
}

impl Scope for RustScope {
    fn root(&self) -> &Path {
        &self.root
    }

    fn projects<'a>(&'a self) -> Box<dyn Iterator<Item=anyhow::Result<Rc<dyn Project>>> + 'a> {
        let projects = self.workspace().members.iter()
            .relative_to(self.root())
            .inspect(|pattern| debug!("Search rust project matching {pattern}"))
            .glob()
            .map(|path| path.and_then(|path| {
                path.canonicalize().with_context(|| format!("Unable to access {}", path.display()))
            }))
            .filter_map(|path| match path {
                Ok(path) => self.project_detector.detect_at_as(&path).into(),
                Err(err) => Some(Err(err)),
            });

        Box::new(projects)
    }
}

impl Tagged for RustScope {
    fn tags(&self) ->  &[&'static Tag] {
        &[&RUST_TAG]
    }
}