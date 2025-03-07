use crate::unit::Unit;
use std::path::Path;

/// Object able to identify code units
pub trait Identifier {
    type Unit: Unit;

    /// Identify unit at given path
    fn identify_unit(&self, path: &Path) -> anyhow::Result<Option<Self::Unit>>;
}