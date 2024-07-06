use std::path::Path;
use semver::Version;

pub trait Workspace {
    fn name(&self) -> &str;
    fn root(&self) -> &Path;
    fn version(&self) -> Option<&Version>;

    fn reference(&self) -> String {
        if let Some(version) = self.version() {
            format!("{}@{version}", self.name())
        } else {
            self.name().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use semver::Version;
    use super::*;

    mock! {
        pub Workspace {}
        
        impl Workspace for Workspace {
            fn name(&self) -> &str;
            fn root(&self) -> &Path;
            fn version(&self) -> Option<&'static Version>;
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
}
