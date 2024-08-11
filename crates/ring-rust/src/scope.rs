use std::path::{Path, PathBuf};
use std::rc::Rc;
use anyhow::Context;
use glob::glob;
use tracing::debug;
use ring_traits::{Project, Scope, Tagged};
use crate::{CargoManifest, CargoWorkspace, RustProjectDetector};

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
        let patterns = self.workspace().members.iter()
            .map(|pattern| self.root.join(pattern));

        Box::new(patterns
            .inspect(|pattern| debug!("Search rust project matching {}", pattern.display()))
            .filter_map(|pattern| {
                #[cfg(windows)]
                { glob(&pattern.to_str().unwrap()[4..]).ok() }

                #[cfg(not(windows))]
                { glob(pattern.to_str().unwrap()).ok() }
            })
            .flatten()
            .map(|path| {
                path.map_err(|err| err.into())
                    .and_then(|path| path.canonicalize().with_context(|| format!("Unable to access {}", path.display())))
                    .and_then(|path| self.project_detector.load_at(&path).into_result())
            })
            .filter_map(|result| match result {
                Ok(Some(prj)) => Some(Ok(prj as Rc<dyn Project>)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            }))
    }
}

impl Tagged for RustScope {
    fn tags(&self) -> &[&'static str] {
        &["rust"]
    }
}