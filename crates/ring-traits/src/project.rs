use std::path::Path;
use semver::Version;
use crate::Tagged;

pub trait Project : Tagged {
    /// Returns project root directory
    fn root(&self) -> &Path;

    /// Returns project name
    fn name(&self) -> &str;

    /// Returns project version (if any)
    fn version(&self) -> Option<&Version>;
}