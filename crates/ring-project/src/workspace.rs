use std::path::Path;

pub trait Workspace {
    fn name(&self) -> &str;
    fn root(&self) -> &Path;
    fn version(&self) -> Option<&str>;

    fn reference(&self) -> String {
        if let Some(version) = &self.version() {
            format!("{}@{version}", self.name())
        } else {
            self.name().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use super::*;
    
    mock! {
        pub Workspace {}
        
        impl Workspace for Workspace {
            fn name(&self) -> &str;
            fn root(&self) -> &Path;
            fn version(&self) -> Option<&'static str>;
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
        let mut mock = MockWorkspace::new();
        mock.expect_name().return_const("test".to_string());
        mock.expect_version().return_const(Some("1.0.0"));

        assert_eq!(mock.reference(), "test@1.0.0");
    }
}
