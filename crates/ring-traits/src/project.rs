use semver::Version;
use ring_utils::{NormalizedPath, Tagged};
use crate::DependencyIterator;

pub trait Project : Tagged {
    /// Returns project root directory
    fn root(&self) -> &NormalizedPath;

    /// Returns project name
    fn name(&self) -> &str;

    /// Returns project version (if any)
    fn version(&self) -> Option<&Version>;
    
    /// Returns project dependencies
    fn dependencies(&self) -> Box<DependencyIterator>;
}