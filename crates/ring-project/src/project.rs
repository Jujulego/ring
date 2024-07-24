use anyhow::Result;
use std::rc::Rc;
use crate::Workspace;

pub trait Project {
    type Workspace: Workspace;
    
    fn workspaces(&self) -> impl Iterator<Item = Result<Rc<Self::Workspace>>>;
}