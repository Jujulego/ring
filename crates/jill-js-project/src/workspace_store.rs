use std::cell::RefCell;
use anyhow::Result;
use std::path::Path;
use std::rc::Rc;
use crate::workspace::JsWorkspace;
use crate::workspace_searcher::WorkspaceSearcher;

#[derive(Debug)]
pub struct WorkspaceStore {
    searcher: WorkspaceSearcher,
    workspaces: Vec<Rc<JsWorkspace>>,
}

impl WorkspaceStore {
    pub fn new(patterns: &Vec<String>, root: &Path) -> WorkspaceStore {
        WorkspaceStore {
            searcher: WorkspaceSearcher::new(patterns, root),
            workspaces: Vec::new()
        }
    }
    
    fn search_next(&mut self) -> Option<Result<Rc<JsWorkspace>>> {
        match self.searcher.next() {
            Some(Ok(workspace)) => {
                let workspace = Rc::new(workspace);
                self.workspaces.push(workspace.clone());
                
                Some(Ok(workspace))
            }
            Some(Err(error)) => {
                Some(Err(error))
            }
            None => None
        }
    }
}

#[derive(Debug)]
pub struct Iter<'store> {
    store: &'store RefCell<WorkspaceStore>,
    next: usize,
}

impl<'store> Iter<'store> {
    pub fn new(store: &'store RefCell<WorkspaceStore>) -> Iter<'store> {
        Iter { store, next: 0 }
    }
}

impl<'store> Iterator for Iter<'store> {
    type Item = Result<Rc<JsWorkspace>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(workspace) = self.store.borrow().workspaces.get(self.next) {
            self.next += 1;
            return Some(Ok(workspace.clone()));
        }

        if let Some(workspace) = self.store.borrow_mut().search_next() {
            self.next += 1;
            Some(workspace)
        } else {
            None
        }
    }
}