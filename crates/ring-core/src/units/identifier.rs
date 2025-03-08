use crate::units::Unit;
use std::borrow::Borrow;
use std::path::Path;

/// Object able to identify code units
pub trait Identifier {
    type Unit;

    /// Identify unit at given path
    fn identify_unit(&self, path: &Path) -> anyhow::Result<Option<Self::Unit>>;
}