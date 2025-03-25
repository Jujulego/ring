use ring_core_file::Language;

/// Creates a yaml language object
///
/// # Example
///
/// ```
/// use ring_module_yaml::yaml_language;
///
/// let language = yaml_language();
/// assert_eq!(language.name(), "yaml");
/// assert_eq!(language.color(), None);
/// ```
#[inline]
pub fn yaml_language() -> Language {
    Language::new("yaml".to_string())
}