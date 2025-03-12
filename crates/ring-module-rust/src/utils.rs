use ring_core_language::Language;

/// Creates a rust language object, with a rusty color
///
/// # Example
///
/// ```
/// use ring_module_rust::rust_language;
///
/// let language = rust_language();
/// assert_eq!(language.name(), "rust");
/// assert_eq!(language.color(), Some(&(0xe3, 0x3b, 0x26).into()));
/// ```
#[inline]
pub fn rust_language() -> Language {
    Language::new("rust".to_string()).with_color((0xe3, 0x3b, 0x26).into())
}