use std::fmt::{Display, Formatter};
use semver::VersionReq;
use crate::{NormalizedPathBuf, Tag, Tagged};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Requirement {
    #[default] Any,
    Path(NormalizedPathBuf),
    Version(VersionReq),
}

impl Display for Requirement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Requirement::Any => write!(f, "any"),
            Requirement::Path(path) => write!(f, "path:{}", path.display()),
            Requirement::Version(version) => write!(f, "version:{}", version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dependency {
    name: String,
    requirement: Requirement,
    tag: Option<Tag>,
}

impl Dependency {
    pub fn new(name: String, requirement: Requirement) -> Self {
        Dependency { name, requirement, tag: None }
    }
    
    pub fn with_tag(self, tag: Tag) -> Self {
        Dependency { tag: Some(tag), ..self }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn requirement(&self) -> &Requirement {
        &self.requirement
    }
    
    pub fn tag(&self) -> &Option<Tag> {
        &self.tag
    }
}

impl Tagged for Dependency {
    fn tags(&self) -> Vec<Tag> {
        if let Some(tag) = &self.tag {
            vec![tag.clone()]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::Normalize;
    use super::*;

    #[test]
    fn it_should_display_requirement() {
        assert_eq!(format!("{}", Requirement::Any), "any");
        assert_eq!(format!("{}", Requirement::Path(Path::new("/test").normalize())), String::from("path:") + std::path::MAIN_SEPARATOR_STR + "test");
        assert_eq!(format!("{}", Requirement::Version(VersionReq::parse("^1.0.0").unwrap())), "version:^1.0.0");
    }
}