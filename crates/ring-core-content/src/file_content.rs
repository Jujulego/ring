use crate::PathContent;
use std::fmt::Display;

/// Define the kind of content detected inside a file
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileContent {
    Configuration,
    Lockfile,
    Manifest,
    Source,
    Storage,
    Tests,
    Other(String),
}

impl FileContent {
    #[cfg(feature = "crossterm")]
    pub fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: match self {
                FileContent::Configuration => Some(crossterm::style::Color::DarkCyan),
                FileContent::Lockfile | FileContent::Manifest => Some(crossterm::style::Color::DarkMagenta),
                FileContent::Other(_) | FileContent::Source => Some(crossterm::style::Color::Blue),
                FileContent::Storage => Some(crossterm::style::Color::DarkYellow),
                FileContent::Tests => Some(crossterm::style::Color::DarkGreen),
            },
            attributes: match self {
                FileContent::Other(_) | FileContent::Lockfile => crossterm::style::Attribute::Dim.into(),
                _ => Default::default()
            },
            ..Default::default()
        }
    }
}

impl From<PathContent> for FileContent {
    fn from(p: PathContent) -> Self {
        match p {
            PathContent::Configuration => FileContent::Configuration,
            PathContent::Lockfile => FileContent::Lockfile,
            PathContent::Manifest => FileContent::Manifest,
            PathContent::Source => FileContent::Source,
            PathContent::Resource => FileContent::Storage,
            PathContent::Test => FileContent::Tests,
            PathContent::Artefact => FileContent::Other("artefact".into()),
            PathContent::Dependency => FileContent::Other("dependency".into()),
            PathContent::Other(label, _) => FileContent::Other(label),
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
            FileContent::Storage => write!(f, "storage"),
            FileContent::Tests => write!(f, "tests"),
            FileContent::Other(name) => name.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_content::FileContent;

    #[test]
    fn it_should_display_to_a_string_matching_content() {
        assert_eq!(FileContent::Configuration.to_string(), "config");
        assert_eq!(FileContent::Lockfile.to_string(), "lockfile");
        assert_eq!(FileContent::Manifest.to_string(), "manifest");
        assert_eq!(FileContent::Source.to_string(), "source");
        assert_eq!(FileContent::Storage.to_string(), "storage");
        assert_eq!(FileContent::Tests.to_string(), "tests");
        assert_eq!(FileContent::Other("toto".into()).to_string(), "toto");
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_config_style() {
        let style = FileContent::Configuration.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::DarkCyan));
        assert!(style.attributes.is_empty());
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
        let style = FileContent::Tests.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::DarkGreen));
        assert!(style.attributes.is_empty());
    }

    #[test]
    #[cfg(feature = "crossterm")]
    fn it_should_return_other_style() {
        let style = FileContent::Other("toto".into()).style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::Blue));
        assert!(style.attributes.has(crossterm::style::Attribute::Dim));
    }
}