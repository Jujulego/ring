use anyhow::Result;
use glob::glob;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, trace};

use crate::constants::MANIFEST;
use crate::workspace::JsWorkspace;

#[derive(Debug)]
pub struct WorkspaceGlob<'prj> {
    patterns: VecDeque<PathBuf>,
    glob_iter: Option<glob::Paths>,
    workspaces: &'prj mut Vec<Rc<JsWorkspace>>,
}

impl<'prj> WorkspaceGlob<'prj> {
    pub fn new(patterns: &Vec<String>, root: &Path, workspaces: &'prj mut Vec<Rc<JsWorkspace>>) -> WorkspaceGlob<'prj> {
        WorkspaceGlob {
            patterns: patterns.iter()
                .map(|pattern| root.join(pattern).join(MANIFEST))
                .collect(),
            glob_iter: None,
            workspaces,
        }
    }
}

impl<'prj> Iterator for WorkspaceGlob<'prj> {
    type Item = Result<Rc<JsWorkspace>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(glob_iter) = &mut self.glob_iter {
                match glob_iter.next() {
                    Some(Ok(manifest)) => {
                        debug!("Found manifest {}", manifest.display());

                        let workspace = JsWorkspace::new(manifest.parent().unwrap());

                        return match workspace {
                            Ok(workspace) => {
                                let workspace = Rc::new(workspace);

                                self.workspaces.push(workspace.clone());
                                Some(Ok(workspace))
                            }
                            Err(error) => {
                                Some(Err(error.into()))
                            }
                        }
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