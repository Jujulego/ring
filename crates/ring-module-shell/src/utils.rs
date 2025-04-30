use ring_core_content::Language;

/// Creates a shell language object
///
/// # Example
///
/// ```
/// use ring_module_shell::shell_language;
///
/// let language = shell_language();
/// assert_eq!(language.name(), "shell");
/// assert_eq!(language.color(), None);
/// ```
#[inline]
pub fn shell_language() -> Language {
    Language::new("shell".to_string())
}