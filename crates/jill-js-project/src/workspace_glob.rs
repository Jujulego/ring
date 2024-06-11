use std::cell::RefCell;
use anyhow::Result;
use glob::glob;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, trace};

use crate::constants::MANIFEST;
use crate::workspace::JsWorkspace;

#[derive(Debug)]
pub struct WorkspaceGlob {
    patterns: VecDeque<PathBuf>,
    glob_iter: Option<glob::Paths>,
    workspaces: Vec<Rc<JsWorkspace>>,
}

impl WorkspaceGlob {
    pub fn new(patterns: &Vec<String>, root: &Path) -> WorkspaceGlob {
        WorkspaceGlob {
            patterns: patterns.iter()
                .map(|pattern| root.join(pattern).join(MANIFEST))
                .collect(),
            glob_iter: None,
            workspaces: Vec::new(),
        }
    }

    fn search(&mut self) -> Option<Result<Rc<JsWorkspace>>> {
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

#[derive(Debug)]
pub struct WorkspaceIterator<'prj> {
    glob: &'prj RefCell<WorkspaceGlob>,
    next: usize,
}

impl<'prj> WorkspaceIterator<'prj> {
    pub fn new(glob: &'prj RefCell<WorkspaceGlob>) -> WorkspaceIterator<'prj> {
        WorkspaceIterator {
            glob,
            next: 0,
        }
    }
}

impl<'prj> Iterator for WorkspaceIterator<'prj> {
    type Item = Result<Rc<JsWorkspace>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(workspace) = self.glob.borrow().workspaces.get(self.next) {
            self.next += 1;
            return Some(Ok(workspace.clone()));
        }
        
        if let Some(workspace) = self.glob.borrow_mut().search() {
            self.next += 1;
            Some(workspace)
        } else {
            None
        }
    }
}