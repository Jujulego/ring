use ring_core_language::Language;

/// Creates a json language object
///
/// # Example
///
/// ```
/// use ring_module_json::json_language;
///
/// let language = json_language();
/// assert_eq!(language.name(), "json");
/// assert_eq!(language.color(), None);
/// ```
#[inline]
pub fn json_language() -> Language {
    Language::new("json".to_string())
}