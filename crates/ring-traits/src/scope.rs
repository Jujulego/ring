use std::path::Path;
use std::rc::Rc;
use crate::{DetectAs, Project, Tagged};

pub trait Scope : Tagged {
    /// Returns scope root directory
    fn root(&self) -> &Path;

    /// Returns an iterator over scope projects
    fn projects<'a>(&'a self) -> Box<dyn Iterator<Item=anyhow::Result<Rc<dyn Project>>> + 'a>;
}

pub type ScopeDetector = dyn DetectAs<Rc<dyn Scope>>;