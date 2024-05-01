use std::rc::Rc;
use crate::graph::workspace::Workspace;

pub enum DependencyKind {
    Prod,
    Dev,
}

pub struct WorkspaceDependency {
    kind: DependencyKind,
    workspace: Rc<Workspace>,
}