use ring_utils::{NormalizedPath, Tagged};
use crate::ProjectIterator;

pub trait Scope : Tagged {
    /// Returns scope root directory
    fn root(&self) -> &NormalizedPath;

    /// Returns an iterator over scope projects
    fn projects(&self) -> Box<ProjectIterator>;
}
