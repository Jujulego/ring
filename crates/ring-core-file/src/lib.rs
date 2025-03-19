use std::fmt::Display;
use std::path::Path;

/// Object able to qualify a given path
pub trait QualifyPath {
    fn qualify_content(&self, path: &Path) -> Option<FileContent>;
}

/// Define the kind of content detected inside a file
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileContent {
    Configuration,
    Lockfile,
    Manifest,
    Source,
    Test,
    Other(String),
}

impl FileContent {
    #[cfg(feature = "crossterm")]
    pub fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: match self {
                FileContent::Lockfile
                | FileContent::Manifest => Some(crossterm::style::Color::DarkMagenta),
                FileContent::Source => Some(crossterm::style::Color::Blue),
                FileContent::Test => Some(crossterm::style::Color::Green),
                _ => None
            },
            attributes: match self {
                FileContent::Configuration
                | FileContent::Lockfile => crossterm::style::Attribute::Dim.into(),
                _ => Default::default()
            },
            ..Default::default()
        }
    }
}

impl Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileContent::Configuration => write!(f, "config"),
            FileContent::Lockfile => write!(f, "lockfile"),
            FileContent::Manifest => write!(f, "manifest"),
            FileContent::Source => write!(f, "source"),
            FileContent::Test => write!(f, "test"),
            FileContent::Other(name) => name.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_display_to_a_string_matching_content() {
        assert_eq!(FileContent::Configuration.to_string(), "config");
        assert_eq!(FileContent::Lockfile.to_string(), "lockfile");
        assert_eq!(FileContent::Manifest.to_string(), "manifest");
        assert_eq!(FileContent::Source.to_string(), "source");
        assert_eq!(FileContent::Test.to_string(), "test");
        assert_eq!(FileContent::Other("toto".into()).to_string(), "toto");
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_config_style() {
        let style = FileContent::Configuration.style();

        assert_eq!(style.foreground_color, None);
        assert!(style.attributes.has(crossterm::style::Attribute::Dim));
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_lockfile_style() {
        let style = FileContent::Lockfile.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::DarkMagenta));
        assert!(style.attributes.has(crossterm::style::Attribute::Dim));
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_manifest_style() {
        let style = FileContent::Manifest.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::DarkMagenta));
        assert!(style.attributes.is_empty());
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_source_style() {
        let style = FileContent::Source.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::Blue));
        assert!(style.attributes.is_empty());
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_test_style() {
        let style = FileContent::Test.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::Green));
        assert!(style.attributes.is_empty());
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_other_style() {
        let style = FileContent::Other("toto".into()).style();

        assert_eq!(style.foreground_color, None);
        assert!(style.attributes.is_empty());
    }
}