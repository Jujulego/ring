use std::fmt::{Display, Formatter};
use std::path::Path;
use rgb::Rgb;

/// Object able to detect language of a given path
pub trait DetectLanguage {
    fn detect_language(&self, path: &Path) -> Option<Language>;
}

/// Represents a language
#[derive(Clone, Debug)]
pub struct Language {
    name: String,
    color: Option<Rgb<u8>>,
}

impl Language {
    /// Creates a new language
    /// 
    /// # Example
    /// 
    /// ```
    /// use ring_core_language::Language;
    /// 
    /// let language = Language::new("example".to_string());
    /// ```
    #[inline]
    pub fn new(name: String) -> Language {
        Language { name, color: None }
    }

    /// Adds a color to the language
    ///
    /// # Example
    ///
    /// ```
    /// use ring_core_language::Language;
    ///
    /// let language = Language::new("example".to_string()).with_color((0x00, 0xff, 0x00).into());
    /// ```
    #[inline]
    pub fn with_color(mut self, color: Rgb<u8>) -> Language {
        self.color = Some(color);
        self
    }
    
    /// Returns language's name
    ///
    /// # Example
    ///
    /// ```
    /// use ring_core_language::Language;
    ///
    /// let language = Language::new("example".to_string());
    /// assert_eq!(language.name(), "example");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns language's color, if any
    ///
    /// # Example
    ///
    /// ```
    /// use rgb::Rgb;
    /// use ring_core_language::Language;
    ///
    /// let language = Language::new("example".to_string());
    /// assert_eq!(language.color(), None);
    ///
    /// let language = Language::new("example".to_string()).with_color((0x00, 0xff, 0x00).into());
    /// assert_eq!(language.color(), Some(&Rgb::new(0x00, 0xff, 0x00)));
    /// ```
    #[inline]
    pub fn color(&self) -> Option<&Rgb<u8>> {
        self.color.as_ref()
    }
}

impl PartialEq for Language {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Language {}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}