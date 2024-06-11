use anyhow::Result;
use glob::glob;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

use crate::constants::MANIFEST;
use crate::workspace::JsWorkspace;

#[derive(Debug)]
pub struct WorkspaceSearcher {
    patterns: VecDeque<PathBuf>,
    glob_iter: Option<glob::Paths>,
}

impl WorkspaceSearcher {
    pub fn new(patterns: &Vec<String>, root: &Path) -> WorkspaceSearcher {
        WorkspaceSearcher {
            patterns: patterns.iter()
                .map(|pattern| root.join(pattern).join(MANIFEST))
                .collect(),
            glob_iter: None,
        }
    }
}

impl Iterator for WorkspaceSearcher {
    type Item = Result<JsWorkspace>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(glob_iter) = &mut self.glob_iter {
                match glob_iter.next() {
                    Some(Ok(manifest)) => {
                        debug!("Found manifest {}", manifest.display());
                        return Some(JsWorkspace::new(manifest.parent().unwrap()));
                    }
                    Some(Err(error)) => {
                        return Some(Err(error.into()));
                    }
                    None => {}
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
