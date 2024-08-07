use std::path::Path;
use std::rc::Rc;
use crate::Scope;

pub trait ScopeDetector {
    /// Search a scope at given path and its ancestors
    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn Scope>>>;
}