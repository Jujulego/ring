use std::path::Path;
use std::rc::Rc;
use crate::{DetectAs, ProjectIterator, Tagged};

pub trait Scope : Tagged {
    /// Returns scope root directory
    fn root(&self) -> &Path;

    /// Returns an iterator over scope projects
    fn projects(&self) -> Box<ProjectIterator<'_>>;
}

pub type ScopeDetector = dyn DetectAs<Rc<dyn Scope>>;