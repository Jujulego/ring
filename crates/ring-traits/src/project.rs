use std::path::Path;
use semver::Version;

pub trait Project {
    /// Returns project root directory
    fn root(&self) -> &Path;

    /// Returns project name
    fn name(&self) -> &str;

    /// Returns project version (if any)
    fn version(&self) -> Option<&Version>;

    /// Return some project tags, identifying the kind of project
    fn tags(&self) -> &[&str];
}