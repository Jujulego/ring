use std::path::PathBuf;
use semver::VersionReq;
use std::fmt::{Display, Formatter};

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_should_print_required_path() {
        let req = Requirement::PATH(PathBuf::from("/test"));
        assert_eq!(format!("{req}"), "path:/test");
    }
    
    #[test]
    fn it_should_print_required_version() {
        let req = Requirement::VERSION(VersionReq::parse("^1.0.0").unwrap());
        assert_eq!(format!("{req}"), "version:^1.0.0");
    }
}