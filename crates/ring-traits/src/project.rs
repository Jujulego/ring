use std::rc::Rc;
use semver::Version;
use ring_utils::NormalizedPath;
use crate::{DetectAs, Tagged};

pub trait Project : Tagged {
    /// Returns project root directory
    fn root(&self) -> &NormalizedPath;

    /// Returns project name
    fn name(&self) -> &str;

    /// Returns project version (if any)
    fn version(&self) -> Option<&Version>;
}

pub type ProjectDetector = dyn DetectAs<Rc<dyn Project>>;
pub type ProjectIterator<'a> = dyn Iterator<Item = anyhow::Result<Rc<dyn Project>>> + 'a;