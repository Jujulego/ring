use std::path::Path;
use std::rc::Rc;
use crate::Scope;

pub trait ScopeDetector {
    type Scope: Scope;

    /// Search a scope at given path and its ancestors
    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Rc<Self::Scope>>>;
}