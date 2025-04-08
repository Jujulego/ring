mod unit;

use std::path::Path;
use std::rc::Rc;
pub use unit::Unit;

/// Object able to detect a unit at given path
pub trait DetectUnit {
    fn detect_unit(&self, path: &Path) -> Option<Rc<dyn Unit>>;
}