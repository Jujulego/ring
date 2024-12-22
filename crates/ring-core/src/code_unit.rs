use std::path::Path;
use std::rc::Rc;
use ring_tag::Tagged;
use crate::CodeLanguage;

////////////////////////////////////////////////////////////////////////////////
// Code Unit
////////////////////////////////////////////////////////////////////////////////

/// Defines a code unit.
pub trait CodeUnit: Tagged {
    /// Returns unit's language
    fn language(&self) -> CodeLanguage;

    /// Returns unit's name
    fn name(&self) -> Option<&str> {
        self.path().file_stem()
            .and_then(|name| name.to_str())
    }

    /// Returns a parent code unit containing this one
    fn parent(&self) -> Option<Rc<dyn CodeUnit>> {
        None
    }

    /// Returns location of the code unit
    fn path(&self) -> &Path;
}

////////////////////////////////////////////////////////////////////////////////
// Code Unit Detector
////////////////////////////////////////////////////////////////////////////////

pub trait CodeUnitDetector {
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>>;
}

////////////////////////////////////////////////////////////////////////////////
// Combined Code Unit Detector
////////////////////////////////////////////////////////////////////////////////

pub type CombinedCodeUnitDetector<const N: usize> = [Rc<dyn CodeUnitDetector>; N];

impl<const N: usize> CodeUnitDetector for CombinedCodeUnitDetector<N> {
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        for detector in self {
            if let Some(unit) = detector.detect(path)? {
                return Ok(Some(unit));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        TestUnit {}
        impl Tagged for TestUnit {}
        impl CodeUnit for TestUnit {
            fn language(&self) -> CodeLanguage;
            fn path(&self) -> &Path;
        }
    }

    #[test]
    fn code_unit_parent_should_return_none_by_default() {
        let tcu = MockTestUnit::new();

        assert!(tcu.parent().is_none());
    }
}