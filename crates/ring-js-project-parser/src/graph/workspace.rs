use crate::graph::dependency::WorkspaceDependency;

pub struct Workspace {
    name: String,
    version: String,
    root: String,
    dependencies: Vec<WorkspaceDependency>,
}