use std::ffi::OsStr;
use std::path::Path;
use tracing::{info, instrument};
use ring_core::unit::Identifier;
use crate::rust_unit::RustUnit;

pub struct RustIdentifier();

impl RustIdentifier {
    fn new() -> RustIdentifier {
        RustIdentifier()
    }
}

impl Default for RustIdentifier {
    #[inline]
    fn default() -> Self {
        RustIdentifier::new()
    }
}

impl Identifier for RustIdentifier {
    type Unit = RustUnit;

    #[instrument(name = "rust.identify_unit", skip(self, path))]
    fn identify_unit(&self, path: &Path) -> anyhow::Result<Option<RustUnit>> {
        if path.extension() == Some(OsStr::new("rs")) {
            info!("identified {} as rust unit", path.display());
            Ok(Some(RustUnit::new(path.to_path_buf())))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use ring_core::unit::Unit;
    use super::*;

    #[test]
    fn it_should_identify_as_rust_unit() {
        let identifier = RustIdentifier::new();
        let unit = identifier.identify_unit(&PathBuf::from("/path/to/file.rs")).unwrap();

        assert!(unit.is_some());
        assert_eq!(unit.unwrap().path(), Path::new("/path/to/file.rs"));
    }

    #[test]
    fn it_should_not_identify_as_rust_unit() {
        let identifier = RustIdentifier::new();
        let unit = identifier.identify_unit(&PathBuf::from("/path/to/file")).unwrap();

        assert!(unit.is_none());
    }
}