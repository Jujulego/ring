use std::path::Path;
use semver::Version;
use crate::Dependency;
use crate::requirement::Requirement;

pub trait Workspace {
    fn name(&self) -> &str;
    fn root(&self) -> &Path;
    fn version(&self) -> Option<&Version>;
    fn dependencies(&self) -> &Vec<Dependency>;

    fn reference(&self) -> String {
        if let Some(version) = self.version() {
            format!("{}@{version}", self.name())
        } else {
            self.name().to_string()
        }
    }

    fn matches(&self, requirement: &Requirement) -> bool {
        match requirement {
            Requirement::PATH(req) => {
                assert!(req.is_absolute(), "requirement path is not absolute");
                assert!(self.root().is_absolute(), "workspace root path is not absolute");
                
                req == self.root()
            },
            Requirement::VERSION(req) => self.version()
                .map(|v| req.matches(v))
                .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use mockall::mock;
    use semver::{Version, VersionReq};
    use super::*;

    mock! {
        pub Workspace {}
        
        impl Workspace for Workspace {
            fn name(&self) -> &str;
            fn root(&self) -> &Path;
            fn version(&self) -> Option<&'static Version>;
            fn dependencies(&self) -> &Vec<Dependency>;
        }
    }

    #[test]
    fn it_should_return_name_as_reference() {
        let mut mock = MockWorkspace::new();
        mock.expect_name().return_const("test".to_string());
        mock.expect_version().return_const(None);

        assert_eq!(mock.reference(), "test");
    }

    #[test]
    fn it_should_return_name_with_version_as_reference() {
        let version = Box::new(Version::new(1, 0, 0));
        let version: &'static Version = Box::leak(version);
        
        let mut mock = MockWorkspace::new();
        mock.expect_name().return_const("test".to_string());
        mock.expect_version().return_const(Some(version));

        assert_eq!(mock.reference(), "test@1.0.0");
    }

    #[test]
    fn it_should_match_given_path_requirement() {
        let root = PathBuf::from(".").canonicalize().unwrap();
        let req = Requirement::PATH(root.clone());

        let mut mock = MockWorkspace::new();
        mock.expect_root().return_const(root);

        assert!(mock.matches(&req));
    }

    #[test]
    fn it_should_match_given_version_requirement() {
        let req = Requirement::VERSION(VersionReq::parse("1.0.0").unwrap());

        let mut mock = MockWorkspace::new();
        let version = Box::new(Version::new(1, 0, 0));
        let version: &'static Version = Box::leak(version);
        mock.expect_version().return_const(version);

        assert!(mock.matches(&req));
    }
}
