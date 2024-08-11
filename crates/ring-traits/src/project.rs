use std::path::Path;
use std::rc::Rc;
use semver::Version;
use crate::{DetectAs, Tagged};

pub trait Project : Tagged {
    /// Returns project root directory
    fn root(&self) -> &Path;

    /// Returns project name
    fn name(&self) -> &str;

    /// Returns project version (if any)
    fn version(&self) -> Option<&Version>;
}

pub type ProjectDetector = dyn DetectAs<Rc<dyn Project>>;
