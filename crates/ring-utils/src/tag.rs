use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use owo_colors::{DynColors, OwoColorize};

#[derive(Debug)]
pub struct Tag {
    label: &'static str,
    color: Option<DynColors>,
}

impl Tag {
    pub const fn new(label: &'static str) -> Tag {
        Tag { label, color: None }
    }

    pub const fn with_color(label: &'static str, color: DynColors) -> Tag {
        Tag { label, color: Some(color) }
    }

    pub const fn label(&self) -> &'static str {
        self.label
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
        self.label.eq(other.label)
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(other.label)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use owo_colors::AnsiColors;
    use owo_colors::DynColors::Ansi;
    use super::*;

    #[test]
    fn it_should_display_with_given_color() {
        assert_eq!(format!("{}", Tag::new("testA")), "testA");
        assert_eq!(
            format!("{}", Tag::with_color("testA", Ansi(AnsiColors::Blue))),
            format!("{}", "testA".blue())
        );
    }

    #[test]
    fn it_should_have_same_eq_than_inner_str() {
        assert_eq!(
            Tag::new("testA").eq(&Tag::new("testB")),
            "testA".eq("testB"),
        );
    }

    #[test]
    fn it_should_have_same_ord_than_inner_str() {
        assert_eq!(
            Tag::new("testA").cmp(&Tag::new("testB")),
            "testA".cmp("testB"),
        );
    }

    #[test]
    fn it_should_have_same_partial_ord_than_inner_str() {
        assert_eq!(
            Tag::new("testA").partial_cmp(&Tag::new("testB")),
            "testA".partial_cmp("testB"),
        );
    }
}