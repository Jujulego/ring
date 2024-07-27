use anyhow::{Context, Result};
use glob::glob;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use tracing::{debug, trace};
use ring_utils::searcher::Searcher;
use crate::constants::MANIFEST;
use crate::workspace::JsWorkspace;

#[derive(Debug)]
pub struct JsWorkspaceSearcher {
    patterns: VecDeque<PathBuf>,
    glob_iter: Option<glob::Paths>,
}

impl JsWorkspaceSearcher {
    pub fn new(patterns: &[String], root: &Path) -> JsWorkspaceSearcher {
        JsWorkspaceSearcher {
            patterns: patterns.iter()
                .map(|pattern| root.join(pattern).join(MANIFEST))
                .collect(),
            glob_iter: None,
        }
    }
}

impl Searcher for JsWorkspaceSearcher {
    type Item = JsWorkspace;

    fn search(&mut self) -> Option<Result<Self::Item>> {
        loop {
            if let Some(glob_iter) = &mut self.glob_iter {
                if let Some(path) = glob_iter.next() {
                    return Some(
                        path.map_err(|error| error.into())
                            .and_then(|path| path.canonicalize().context(format!("Unable to access {}", path.display())))
                            .and_then(|path| {
                                debug!("Found manifest {}", path.display());
                                JsWorkspace::new(path.parent().unwrap())
                            })
                    );
                }
            }

            if let Some(pattern) = self.patterns.pop_front() {
                let pattern = pattern.to_str().unwrap();

                #[cfg(windows)]
                let pattern = pattern.strip_prefix(r"\\?\").unwrap_or(pattern);

                trace!("Search manifests matching pattern {pattern}");

                match glob(pattern) {
                    Ok(glob_iter) => {
                        self.glob_iter = Some(glob_iter);
                    }
                    Err(error) => {
                        return Some(Err(error.into()))
                    }
                }
            } else {
                return None
            }
        }
    }
}
