use ring_color::StableColor;
use ring_tag::{Tag, Tagged};
use std::path::Path;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// Code Language
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct CodeLanguage {
    name: String,
    color: StableColor,
}

impl CodeLanguage {
    pub const fn new(name: String, color: StableColor) -> CodeLanguage {
        CodeLanguage { name, color }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> &StableColor {
        &self.color
    }
}

impl From<CodeLanguage> for Tag {
    fn from(l: CodeLanguage) -> Tag {
        Tag::new(l.name).with_stable_color(l.color)
    }
}

impl From<&CodeLanguage> for Tag {
    fn from(l: &CodeLanguage) -> Tag {
        Tag::new(l.name.clone()).with_stable_color(l.color)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Code Unit
////////////////////////////////////////////////////////////////////////////////

/// Defines a code unit.
pub trait CodeUnit: Tagged {
    /// Returns unit's language
    fn language(&self) -> CodeLanguage;

    /// Returns unit's name
    fn name(&self) -> Option<&str> {
        self.path().file_name()
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