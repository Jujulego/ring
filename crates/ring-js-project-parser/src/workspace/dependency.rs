use semver::VersionReq;

#[derive(Debug, Eq, PartialEq)]
pub enum WorkspaceKind {
    Prod,
    Dev,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorkspaceRequirement {
    Npm(VersionReq),
}

#[derive(Debug)]
pub struct WorkspaceDependency {
    name: String,
    requirement: WorkspaceRequirement,
    kind: WorkspaceKind,
}

impl WorkspaceDependency {
    pub fn new(name: &str, kind: WorkspaceKind, requirement: &str) -> Result<Self, semver::Error> {
        Ok(WorkspaceDependency {
            name: String::from(name),
            requirement: WorkspaceRequirement::Npm(requirement.parse()?),
            kind,
        })
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }
    
    pub fn get_kind(&self) -> &WorkspaceKind {
        &self.kind
    }
    
    pub fn get_requirement(&self) -> &WorkspaceRequirement {
        &self.requirement
    }
}