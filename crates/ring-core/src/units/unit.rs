use crate::units::Language;
use std::path::Path;

/// Object representing a file containing code
pub trait Unit {
    /// Returns code unit's language
    fn language(&self) -> &Language;

    /// Returns code unit's name
    fn name(&self) -> Option<&str> {
        self.path().file_stem()
            .and_then(|name| name.to_str())
    }

    /// Returns path to the code unit
    fn path(&self) -> &Path;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use std::path::PathBuf;

    mock! {
        TestUnit {}

        impl Unit for TestUnit {
            fn language(&self) -> &Language;
            fn path(&self) -> &Path;
        }
    }

    #[test]
    fn it_should_return_file_stem_as_name() {
        let mut mock = MockTestUnit::new();

        mock.expect_path()
            .return_const(PathBuf::from("toto/test.txt"));

        assert_eq!(mock.name(), Some("test"));
    }
}