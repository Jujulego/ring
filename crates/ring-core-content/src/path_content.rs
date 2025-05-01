use std::fmt::{Display, Formatter};

/// Define the kind of content that can be found at a given path
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathContent {
    /// Files or directories containing results of a processing (compiled file, or files produced as output of a program)
    Artefact,
    /// Configuration files
    Configuration,
    /// Files or directories containing downloaded dependencies of a project
    Dependency,
    /// Generated file, freezing a resource or a result:
    /// - file defining dependencies version to install (like the "Cargo.lock" file)
    /// - a lock protecting a file/folder
    Lockfile,
    /// Declaration of a "code unit" (npm package, crate, ...)
    Manifest,
    /// Files or directories containing data mainly used as inputs for a project
    Resource,
    /// Files or directories mainly containing source code
    Source,
    /// Files or directories mainly containing test code
    Test,
}

impl Display for PathContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathContent::Artefact => f.write_str(if f.alternate() { "artefacts" } else { "artefact" }),
            PathContent::Configuration => f.write_str(if f.alternate() { "configs" } else { "config" }),
            PathContent::Dependency => f.write_str(if f.alternate() { "dependencies" } else { "dependency" }),
            PathContent::Lockfile => f.write_str("lockfile"),
            PathContent::Manifest => f.write_str("manifest"),
            PathContent::Resource => f.write_str(if f.alternate() { "resources" } else { "resource" }),
            PathContent::Source => f.write_str(if f.alternate() { "sources" } else { "source" }),
            PathContent::Test => f.write_str(if f.alternate() { "tests" } else { "test" }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_display_artefacts() {
        assert_eq!(format!("{}", PathContent::Artefact), "artefact");
        assert_eq!(format!("{:#}", PathContent::Artefact), "artefacts");
    }

    #[test]
    fn it_should_display_configurations() {
        assert_eq!(format!("{}", PathContent::Configuration), "config");
        assert_eq!(format!("{:#}", PathContent::Configuration), "configs");
    }

    #[test]
    fn it_should_display_dependencies() {
        assert_eq!(format!("{}", PathContent::Dependency), "dependency");
        assert_eq!(format!("{:#}", PathContent::Dependency), "dependencies");
    }

    #[test]
    fn it_should_display_lockfile() {
        assert_eq!(format!("{}", PathContent::Lockfile), "lockfile");
    }

    #[test]
    fn it_should_display_manifest() {
        assert_eq!(format!("{}", PathContent::Manifest), "manifest");
    }

    #[test]
    fn it_should_display_resources() {
        assert_eq!(format!("{}", PathContent::Resource), "resource");
        assert_eq!(format!("{:#}", PathContent::Resource), "resources");
    }

    #[test]
    fn it_should_display_sources() {
        assert_eq!(format!("{}", PathContent::Source), "source");
        assert_eq!(format!("{:#}", PathContent::Source), "sources");
    }

    #[test]
    fn it_should_display_tests() {
        assert_eq!(format!("{}", PathContent::Test), "test");
        assert_eq!(format!("{:#}", PathContent::Test), "tests");
    }
}