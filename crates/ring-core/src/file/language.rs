use rgb::Rgb;

/// A language in which a file can be written
#[derive(Clone, Debug)]
pub struct Language {
    name: &'static str,
    color: Option<Rgb<u8>>,
}

impl Language {
    /// Create a new uncolored language. Can be used to create a constant value
    ///
    /// # Example
    /// ```
    /// use ring_core::file::Language;
    ///
    /// const LANGUAGE: Language = Language::new("html");
    /// ```
    pub const fn new(name: &'static str) -> Self {
        Language { name, color: None }
    }

    /// Adds a color to a language.
    ///
    /// # Example
    /// ```
    /// use rgb::Rgb;
    /// use ring_core::file::Language;
    ///
    /// const LANGUAGE: Language = Language::new("html").with_color(Rgb { r: 0x12, g: 0xa4, b: 0x62 });
    /// ```
    pub const fn with_color(mut self, color: Rgb<u8>) -> Self {
        self.color = Some(color);
        self
    }

    /// Returns the name of the language.
    ///
    /// # Example
    /// ```
    /// use ring_core::file::Language;
    ///
    /// const LANGUAGE: Language = Language::new("html");
    /// assert_eq!(LANGUAGE.name(), "html");
    /// ```
    pub const fn name(&self) -> &str {
        self.name
    }

    /// Returns the color of the language, if any.
    ///
    /// # Example
    /// ```
    /// use rgb::Rgb;
    /// use ring_core::file::Language;
    ///
    /// const LANGUAGE: Language = Language::new("html");
    /// assert_eq!(LANGUAGE.color(), None);
    ///
    /// const COLORED: Language = LANGUAGE.with_color(Rgb { r: 0x12, g: 0xa4, b: 0x62 });
    /// assert_eq!(COLORED.color(), Some(Rgb { r: 0x12, g: 0xa4, b: 0x62 }));
    /// ```
    pub const fn color(&self) -> Option<Rgb<u8>> {
        self.color
    }
}
