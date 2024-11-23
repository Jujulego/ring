use std::path::Path;
use std::rc::Rc;
use ring_tag::Tagged;

////////////////////////////////////////////////////////////////////////////////
// Code Unit
////////////////////////////////////////////////////////////////////////////////

/// Defines a code unit.
pub trait CodeUnit: Tagged {
    /// Returns a parent code unit containing this one
    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        None
    }
    
    /// Returns location of the code unit
    fn path(&self) -> &Path;
}
