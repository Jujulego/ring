use std::cell::RefCell;
use anyhow::Result;
use std::rc::Rc;
use crate::Searcher;

#[derive(Debug)]
pub struct Store<I, S: Searcher<Item =I>> {
    searcher: S,
    workspaces: Vec<Rc<I>>,
}

impl<I, S: Searcher<Item = I>> Store<I, S> {
    pub fn new(searcher: S, workspaces: Vec<Rc<I>>) -> Self {
        Store { searcher, workspaces }
    }
    
    fn search_next(&mut self) -> Option<Result<Rc<I>>> {
        match self.searcher.search() {
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
pub struct Iter<'store, I, S: Searcher<Item =I>> {
    store: &'store RefCell<Store<I, S>>,
    next: usize,
}

impl<'store, I, S: Searcher<Item = I>> Iter<'store, I, S> {
    pub fn new(store: &'store RefCell<Store<I, S>>) -> Iter<'store, I, S> {
        Iter { store, next: 0 }
    }
}

impl<'store, I, S: Searcher<Item = I>> Iterator for Iter<'store, I, S> {
    type Item = Result<Rc<I>>;

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