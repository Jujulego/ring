use std::path::Path;
use std::rc::Rc;
use crate::Project;

pub trait ProjectDetector {
    type Project: Project;

    /// Search project containing given path
    fn search_from(&self, path: &Path) -> anyhow::Result<Option<Rc<Self::Project>>>;
}