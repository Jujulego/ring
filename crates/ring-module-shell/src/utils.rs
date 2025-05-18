use ring_core_content::Language;

/// Creates a powershell language object
///
/// # Example
///
/// ```
/// use ring_module_shell::powershell_language;
///
/// assert_eq!(powershell_language().name(), "powershell");
/// ```
#[inline]
pub fn powershell_language() -> Language {
    Language::new("powershell".to_string()).with_color((0x42, 0x72, 0xc9).into())
}

/// Creates a shell language object
///
/// # Example
///
/// ```
/// use ring_module_shell::shell_language;
///
/// let language = shell_language();
/// assert_eq!(language.name(), "shell");
/// ```
#[inline]
pub fn shell_language() -> Language {
    Language::new("shell".to_string())
}

#[cfg(test)]
mod tests {
    use rgb::Rgb;
    use super::*;

    #[test]
    fn is_should_build_a_powershell_language() {
        let language = powershell_language();

        assert_eq!(language.name(), "powershell");
        assert_eq!(language.color(), Some(&Rgb { r: 0x42, g: 0x72, b: 0xc9 }));
    }

    #[test]
    fn is_should_build_a_shell_language() {
        let language = shell_language();

        assert_eq!(language.name(), "shell");
        assert_eq!(language.color(), None);
    }
}