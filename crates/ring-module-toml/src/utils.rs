use ring_core_file::Language;

/// Creates a toml language object
///
/// # Example
///
/// ```
/// use ring_module_toml::toml_language;
///
/// let language = toml_language();
/// assert_eq!(language.name(), "toml");
/// assert_eq!(language.color(), None);
/// ```
#[inline]
pub fn toml_language() -> Language {
    Language::new("toml".to_string())
}