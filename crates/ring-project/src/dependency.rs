use std::path::PathBuf;
use semver::VersionReq;

#[derive(Debug)]
pub enum Requirement {
    PATH(PathBuf),
    VERSION(VersionReq),
}

#[derive(Debug)]
pub struct Dependency {
    target: String,
    requirement: Requirement,
}

impl Dependency {
    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn requirement(&self) -> &Requirement {
        &self.requirement
    }
}