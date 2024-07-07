use anyhow::Result;

pub trait Searcher {
    type Item;

    fn search(&mut self) -> Option<Result<Self::Item>>;
}