use std::path::Path;
use std::rc::Rc;
use crate::Project;

pub trait Scope {
    /// Returns scope root directory
    fn root(&self) -> &Path;

    /// Returns an iterator over scope projects
    fn projects<'a>(&'a self) -> Box<dyn Iterator<Item=anyhow::Result<Rc<dyn Project>>> + 'a>;
    
    /// Return some scope tags, identifying the kind of project
    fn tags(&self) -> &[&str];
}