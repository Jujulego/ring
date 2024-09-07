use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use owo_colors::{DynColors, OwoColorize};

#[derive(Clone, Debug)]
pub struct Tag {
    label: String,
    color: Option<DynColors>,
}

impl Tag {
    pub fn new(label: String) -> Tag {
        Tag { label, color: None }
    }

    pub fn with_color(self, color: DynColors) -> Tag {
        Tag { label: self.label, color: Some(color) }
    }

    pub const fn label(&self) -> &String {
        &self.label
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(color) = self.color {
            self.label.color(color).fmt(f)
        } else {
            self.label.fmt(f)
        }
    }
}

impl Eq for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag::new(String::from(value))
    }
}

impl FromStr for Tag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tag::from(s))
    }
}

pub trait Tagged {
    /// Return some tags on entity
    fn tags(&self) -> Vec<Tag>;
}

#[cfg(test)]
mod tests {
    use owo_colors::AnsiColors;
    use owo_colors::DynColors::Ansi;
    use super::*;

    #[test]
    fn it_should_display_with_given_color() {
        assert_eq!(format!("{}", Tag::from("testA")), "testA");
        assert_eq!(
            format!("{}", Tag::from("test1").with_color(Ansi(AnsiColors::Blue))),
            format!("{}", "testA".blue())
        );
    }

    #[test]
    fn it_should_have_same_eq_than_inner_str() {
        assert_eq!(
            Tag::from("testA").eq(&Tag::from("testB")),
            "testA".eq("testB"),
        );
    }

    #[test]
    fn it_should_have_same_ord_than_inner_str() {
        assert_eq!(
            Tag::from("testA").cmp(&Tag::from("testB")),
            "testA".cmp("testB"),
        );
    }

    #[test]
    fn it_should_have_same_partial_ord_than_inner_str() {
        assert_eq!(
            Tag::from("testA").partial_cmp(&Tag::from("testB")),
            "testA".partial_cmp("testB"),
        );
    }
}