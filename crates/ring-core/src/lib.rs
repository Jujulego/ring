use ring_core_file::FileContent;
use ring_core_language::Language;
use ring_core_modules::Module;
use std::path::Path;

/// Holds and manages all modules references
pub struct Core {
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    /// Initiate all modules
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    /// 
    /// let core = Core::new();
    /// ```
    pub fn new() -> Self {
        let modules: Vec<Box<dyn Module>> = vec![
            #[cfg(feature = "rust")]
            Box::new(ring_module_rust::RustModule::new()),
            #[cfg(feature = "toml")]
            Box::new(ring_module_toml::TomlModule::new()),
        ];

        Core { modules }
    }

    /// Uses all modules to detect given path's language
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    /// use ring_module_rust::rust_language;
    ///
    /// let core = Core::new();
    /// assert_eq!(core.detect_language("src/lib.rs"), Some(rust_language()));
    /// ```
    #[inline]
    pub fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language> {
        self._detect_language(path.as_ref())
    }

    fn _detect_language(&self, path: &Path) -> Option<Language> {
        self.modules.iter()
            .flat_map(|module| module.language_detectors())
            .filter_map(|detector| detector.detect_language(path))
            .next()
    }

    /// Uses all modules to qualify given path's content
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    /// use ring_core_file::FileContent;
    /// use ring_module_rust::rust_language;
    ///
    /// let core = Core::new();
    /// assert_eq!(core.qualify_content("Cargo.toml"), Some(FileContent::Manifest));
    /// ```
    #[inline]
    pub fn qualify_content<P: AsRef<Path>>(&self, path: P) -> Option<FileContent> {
        self._qualify_content(path.as_ref())
    }

    fn _qualify_content(&self, path: &Path) -> Option<FileContent> {
        self.modules.iter()
            .flat_map(|module| module.path_qualifiers())
            .filter_map(|detector| detector.qualify_content(path))
            .max_by_key(|(_, qualified_path)| qualified_path.components().count())
            .map(|(content, _)| content)
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}