use ring_core_language::Language;

/// Creates a javascript language object
///
/// # Example
///
/// ```
/// use ring_module_javascript::javascript_language;
///
/// let language = javascript_language();
/// assert_eq!(language.name(), "javascript");
/// assert_eq!(language.color(), Some(&(0xf7, 0xdf, 0x1e).into()));
/// ```
#[inline]
pub fn javascript_language() -> Language {
    Language::new("javascript".to_string()).with_color((0xf7, 0xdf, 0x1e).into())
}