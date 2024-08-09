use std::path::Path;
use std::rc::Rc;
use crate::Project;

pub trait ProjectDetector {
    /// Search a project at given path and its ancestors
    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn Project>>>;
}