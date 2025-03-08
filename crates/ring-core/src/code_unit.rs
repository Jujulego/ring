use crate::CodeLanguage;
use ring_tag::Tagged;
use std::path::Path;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// Code Unit
////////////////////////////////////////////////////////////////////////////////

/// Defines a code unit.
#[deprecated(note = "Please use `ring-core::units::Unit` instead")]
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

#[deprecated(note = "Please use `ring-core::units::Identifier` instead")]
pub trait CodeUnitDetector {
    fn name(&self) -> &str;
    
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>>;
}

////////////////////////////////////////////////////////////////////////////////
// Combined Code Unit Detector
////////////////////////////////////////////////////////////////////////////////

pub struct CombinedCodeUnitDetector {
    name: String,
    detectors: Vec<Rc<dyn CodeUnitDetector>>,
}

impl CombinedCodeUnitDetector {
    pub fn new(name: String, detectors: &[Rc<dyn CodeUnitDetector>]) -> CombinedCodeUnitDetector {
        CombinedCodeUnitDetector {
            name,
            detectors: Vec::from(detectors),
        }
    }

    pub fn detectors(&self) -> &[Rc<dyn CodeUnitDetector>] {
        &self.detectors
    }
}

impl CodeUnitDetector for CombinedCodeUnitDetector {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        for detector in &self.detectors {
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