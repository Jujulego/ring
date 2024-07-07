use std::cell::RefCell;
use anyhow::Result;
use std::rc::Rc;
use crate::searcher::Searcher;

#[derive(Debug)]
pub struct Store<I, S: Searcher<Item =I>> {
    searcher: RefCell<S>,
    workspaces: RefCell<Vec<Rc<I>>>,
}

impl<I, S: Searcher<Item = I>> Store<I, S> {
    pub fn new(searcher: S) -> Self {
        Store::new_with(searcher, Vec::new())
    }

    pub fn new_with(searcher: S, workspaces: Vec<Rc<I>>) -> Self {
        Store {
            searcher: RefCell::new(searcher),
            workspaces: RefCell::new(workspaces)
        }
    }

    pub fn iter(&self) -> Iter<I, S> {
        Iter { store: self, next: 0 }
    }

    fn search(&self) -> Option<Result<Rc<I>>> {
        match self.searcher.borrow_mut().search() {
            Some(Ok(workspace)) => {
                let workspace = Rc::new(workspace);
                self.workspaces.borrow_mut().push(workspace.clone());

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
    store: &'store Store<I, S>,
    next: usize,
}

impl<'store, I, S: Searcher<Item = I>> Iterator for Iter<'store, I, S> {
    type Item = Result<Rc<I>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(workspace) = self.store.workspaces.borrow().get(self.next) {
            self.next += 1;
            return Some(Ok(workspace.clone()));
        }

        if let Some(workspace) = self.store.search() {
            self.next += 1;
            Some(workspace)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use crate::searcher::Searcher;
    use super::*;

    mock! {
        pub Searcher {}

        impl Searcher for Searcher {
            type Item = &'static str;

            fn search(&mut self) -> Option<Result<&'static str>>;
        }
    }

    #[test]
    fn it_should_use_searcher_to_load_items() {
        // Prepare searcher
        let mut searcher = MockSearcher::new();
        let mut first = true;

        searcher.expect_search()
            .times(3)
            .returning(move || if first { first = false; Some(Ok("test")) } else { None });

        // 1st iterator
        let store = Store::new(searcher);
        let mut iter = store.iter();

        assert!(match iter.next() { Some(Ok(val)) => *val == "test", _ => false });
        assert!(iter.next().is_none());

        // 2nd iterator
        let mut iter = store.iter();

        assert!(match iter.next() { Some(Ok(val)) => *val == "test", _ => false });
        assert!(iter.next().is_none());
    }
}