use std::fmt::{Display, Formatter};
use semver::VersionReq;
use crate::NormalizedPathBuf;

#[derive(Debug, Eq, PartialEq)]
pub enum Requirement {
    Any,
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