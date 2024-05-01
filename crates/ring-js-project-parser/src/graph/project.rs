use std::rc::Rc;
use crate::graph::workspace::Workspace;

pub struct Project {
    name: String,
    root: String,
    workspaces: Vec<Rc<Workspace>>,
}