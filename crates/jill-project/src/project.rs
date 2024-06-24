use anyhow::Result;
use std::rc::Rc;

pub trait Project {
    type Workspace;
    
    fn workspaces(&self) -> impl Iterator<Item = Result<Rc<Self::Workspace>>>;
}