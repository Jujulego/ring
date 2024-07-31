use std::path::Path;
use semver::Version;

pub trait Project {
    /// Returns project name
    fn name(&self) -> &str;

    /// Returns project root directory
    fn root(&self) -> &Path;

    /// Returns project version (if any)
    fn version(&self) -> Option<&Version>;
}