mod unit;
mod unit_data;

pub use crate::unit::Unit;
pub use crate::unit_data::UnitData;
use std::path::Path;
use std::rc::Rc;

/// Object able to detect a unit at given path
pub trait DetectUnit {
    fn detect_unit(&self, path: &Path) -> Option<Rc<dyn Unit>>;
}