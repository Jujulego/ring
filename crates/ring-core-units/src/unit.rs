use crate::unit_data::UnitData;
use ring_core_content::Language;
use std::path::Path;

/// Group of files, contained in a same directory, working together in a given purpose
pub trait Unit {
    /// Returns the unit's kind
    fn kind(&self) -> &str;

    /// Returns the unit's root path
    fn root(&self) -> &Path;

    /// Returns the unit's main language, if any
    #[inline]
    fn language(&self) -> Option<Language> {
        None
    }

    /// Returns the unit's name, if any
    #[inline]
    fn name(&self) -> Option<&str> {
        None
    }

    /// Builds a [`UnitData`] object from the current task
    #[inline]
    fn inspect(&self) -> UnitData {
        UnitData {
            kind: self.kind().to_string(),
            language: self.language(),
            name: self.name().map(|n| n.to_string()),
            root: self.root().to_path_buf(),
        }
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        self.language()
            .map(|language| language.style())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    struct TestUnit;

    impl Unit for TestUnit {
        fn kind(&self) -> &str {
            "kind"
        }

        fn root(&self) -> &Path {
            "/test".as_ref()
        }
    }

    struct TestUnitWithLanguage {
        language: Language,
    }

    impl Unit for TestUnitWithLanguage {
        fn kind(&self) -> &str {
            "kind"
        }

        fn root(&self) -> &Path {
            "/test".as_ref()
        }

        fn language(&self) -> Option<Language> {
            Some(self.language.clone())
        }
    }

    #[test]
    fn language_should_return_none_by_default() {
        let unit = TestUnit;

        assert!(unit.language().is_none());
    }

    #[test]
    fn name_should_return_none_by_default() {
        let unit = TestUnit;

        assert!(unit.name().is_none());
    }

    #[test]
    fn inspect_should_build_data_using_other_methods() {
        let unit = TestUnit;
        let data = unit.inspect();

        assert_eq!(data.kind, "kind");
        assert_eq!(data.root, PathBuf::from("/test"));
        assert!(data.language.is_none());
        assert!(data.name.is_none());
    }

    #[test]
    fn inspect_should_build_data_using_other_methods_with_language() {
        let language = Language::new("test".to_string())
            .with_color((0x00, 0xff, 0x00).into());

        let unit = TestUnitWithLanguage { language: language.clone() };
        let data = unit.inspect();

        assert_eq!(data.kind, "kind");
        assert_eq!(data.root, PathBuf::from("/test"));
        assert_eq!(data.language, Some(language));
        assert!(data.name.is_none());
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn style_should_return_default_style() {
        let unit = TestUnit;
        
        assert_eq!(unit.style(), Default::default());
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn style_should_return_language_style() {
        let language = Language::new("test".to_string())
            .with_color((0x00, 0xff, 0x00).into());
        
        let unit = TestUnitWithLanguage {
            language: language.clone(),
        };
        
        assert_eq!(unit.style(), language.style());
    }
}