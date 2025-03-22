use ring_core_language::Language;

/// Creates a toml language object
///
/// # Example
///
/// ```
/// use ring_module_toml::toml_language;
///
/// let language = toml_language();
/// assert_eq!(language.name(), "toml");
/// assert_eq!(language.color(), Some(&(0x9c, 0x42, 0x21).into()));
/// ```
#[inline]
pub fn toml_language() -> Language {
    Language::new("toml".to_string()).with_color((0x9c, 0x42, 0x21).into())
}