use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use semver::VersionReq;

#[derive(Debug)]
pub enum Requirement {
    PATH(PathBuf),
    VERSION(VersionReq),
}

impl Display for Requirement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Requirement::PATH(path) => write!(f, "path:{}", path.display()),
            Requirement::VERSION(version) => write!(f, "version:{version}")
        }
    }
}

#[derive(Debug)]
pub struct Dependency {
    target: String,
    requirement: Requirement,
}

impl Dependency {
    pub fn new(target: String, requirement: Requirement) -> Dependency {
        Dependency { target, requirement }
    }
    
    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn requirement(&self) -> &Requirement {
        &self.requirement
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.target, self.requirement)
    }
}