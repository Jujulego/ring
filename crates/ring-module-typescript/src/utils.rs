use ring_core_file::Language;

/// Creates a typescript language object
///
/// # Example
///
/// ```
/// use ring_module_typescript::typescript_language;
///
/// let language = typescript_language();
/// assert_eq!(language.name(), "typescript");
/// assert_eq!(language.color(), Some(&(0x00, 0x7a, 0xcc).into()));
/// ```
#[inline]
pub fn typescript_language() -> Language {
    Language::new("typescript".to_string()).with_color((0x00, 0x7a, 0xcc).into())
}