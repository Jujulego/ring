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
