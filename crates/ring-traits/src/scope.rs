use std::path::Path;
use std::rc::Rc;
use crate::Project;

pub trait Scope {
    type Project : Project;
    
    /// Returns scope root directory
    fn root(&self) -> &Path;

    /// Returns an iterator over scope projects
    fn projects(&self) -> impl Iterator<Item = anyhow::Result<Rc<Self::Project>>>;
    
    /// Return some scope tags, identifying the kind of project
    fn tags(&self) -> &[&str];
}