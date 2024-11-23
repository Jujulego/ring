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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCodeUnit {}
    impl Tagged for TestCodeUnit {}
    impl CodeUnit for TestCodeUnit {
        fn path(&self) -> &Path {
            todo!()
        }
    }

    #[test]
    fn code_unit_parent_should_return_none_by_default() {
        let tcu = TestCodeUnit {};
        
        assert!(tcu.parent().is_none());
    }
}